use crate::{PaginationParams, Result, Timestamp};
use bson::oid::ObjectId;
use bson::{Bson, Document, doc, to_document};
use futures::TryStreamExt;
use mongodb::options::{FindOptions, IndexOptions};
use mongodb::{Client as ExternalClient, Collection, Database, IndexModel};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::to_value as to_json;
use std::str::FromStr;

pub const PRIMARY_KEY: &str = "_id";
pub const MODIFIED_TIME_KEY: &str = "mt";

#[derive(Debug, Deserialize)]
pub struct QueryItemsParams {
    pub collection: String,
    pub pagination: Option<PaginationParams>,
    pub filters: Option<JsonValue>,
    pub sort: Option<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub struct MutateItemsParams {
    pub collection: String,
    pub data: Vec<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUniqueIndexParams {
    pub collection: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Client {
    #[allow(unused)]
    client: ExternalClient,
    pub db: Database,
}

impl Client {
    pub async fn new(uri: impl AsRef<str>, db: &str) -> Result<Self> {
        let client = ExternalClient::with_uri_str(uri).await?;
        let db = client.database(db);

        Ok(Self { client, db })
    }

    pub fn collection<T: Send + Sync>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }

    pub async fn find_all(&self, collection: &str) -> Result<Vec<JsonValue>> {
        let coll: Collection<Document> = self.collection(collection);

        let mut cursor = coll.find(doc! {}).await?;
        let mut res = vec![];

        while let Some(item) = cursor.try_next().await? {
            res.push(item.to_json()?);
        }

        Ok(res)
    }

    // ! this will get all the data if no pagination or limit is specified or if limit is 0
    pub async fn find(
        &self,
        QueryItemsParams {
            collection,
            filters,
            pagination,
            sort,
        }: QueryItemsParams,
    ) -> Result<Vec<JsonValue>> {
        let coll: Collection<Document> = self.collection(&collection);
        let sort = sort.and_then(|it| it.to_document().inspect_err(|err| eprintln!("{err}")).ok());

        let options = if let Some(PaginationParams {
            limit: Some(limit),
            page,
        }) = pagination
        {
            let page = page.unwrap_or(1);
            let skip = (page - 1) * limit;

            Some(
                FindOptions::builder()
                    .skip(skip as u64)
                    .limit(limit as i64)
                    .sort(sort)
                    .build(),
            )
        } else {
            sort.map(|s| FindOptions::builder().sort(s).build())
        };

        let filters = filters
            .and_then(|it| it.to_document().inspect_err(|err| eprintln!("{err}")).ok())
            .unwrap_or_default();

        let mut cursor = coll.find(filters).with_options(options).await?;
        let mut res = vec![];

        while let Some(item) = cursor.try_next().await? {
            res.push(item.to_json()?);
        }

        Ok(res)
    }

    pub async fn add(
        &self,
        MutateItemsParams { collection, data }: MutateItemsParams,
    ) -> Vec<JsonValue> {
        let coll: Collection<Document> = self.collection(&collection);
        let mut res = Vec::with_capacity(data.len());

        for item in data {
            if let Ok(mut item) = item.to_document().inspect_err(|err| eprintln!("{err}")) {
                // ! DO NOT ALLOW MANUAL _id CREATION
                item.remove(PRIMARY_KEY);

                if let Some(item) = coll
                    .insert_one(&item)
                    .await
                    .inspect_err(|err| eprintln!("{err}"))
                    .ok()
                    .and_then(|x| x.inserted_id.as_object_id())
                    .and_then(|id| {
                        item.insert(PRIMARY_KEY, id.to_hex());
                        item.to_json().inspect_err(|err| eprintln!("{err}")).ok()
                    })
                {
                    res.push(item);
                }
            }
        }

        res
    }

    pub async fn update(
        &self,
        MutateItemsParams { collection, data }: MutateItemsParams,
    ) -> Vec<JsonValue> {
        let coll: Collection<Document> = self.collection(&collection);
        let mut res = Vec::with_capacity(data.len());

        for item in data {
            if let Ok(mut item) = item.to_document().inspect_err(|err| eprintln!("{err}")) {
                if let Some(id) = item.remove(PRIMARY_KEY).and_then(|id| id.as_object_id()) {
                    if let Some(item) = coll
                        .update_one(doc! { PRIMARY_KEY: id }, doc! { "$set": &item })
                        .await
                        .inspect_err(|err| eprintln!("{err}"))
                        .ok()
                        .and_then(|r| {
                            (r.modified_count > 0).then(|| {
                                item.insert(PRIMARY_KEY, id.to_hex());
                                item.to_json().inspect_err(|err| eprintln!("{err}")).ok()
                            })?
                        })
                    {
                        res.push(item);
                    }
                }
            }
        }

        res
    }

    pub async fn remove(
        &self,
        MutateItemsParams { collection, data }: MutateItemsParams,
    ) -> Vec<JsonValue> {
        let coll: Collection<Document> = self.collection(&collection);
        let mut res = Vec::with_capacity(data.len());

        for item in data {
            if let Ok(item) = item.to_document().inspect_err(|err| eprintln!("{err}")) {
                if let Some(id) = item.get(PRIMARY_KEY).and_then(|id| id.as_object_id()) {
                    if let Some(item) = coll
                        .delete_one(doc! { PRIMARY_KEY: id })
                        .await
                        .inspect_err(|err| eprintln!("{err}"))
                        .ok()
                        .and_then(|r| {
                            (r.deleted_count > 0).then(|| {
                                item.to_json().inspect_err(|err| eprintln!("{err}")).ok()
                            })?
                        })
                    {
                        res.push(item);
                    }
                }
            }
        }

        res
    }

    pub async fn drop(&self, name: &str) -> Result<()> {
        let coll: Collection<Document> = self.collection(name);

        Ok(coll.drop().await?)
    }

    pub async fn stats(&self) -> Result<Vec<CollectionStats>> {
        let mut res = vec![];

        for name in self.db.list_collection_names().await? {
            let coll: Collection<Document> = self.collection(&name);
            let count = coll.estimated_document_count().await?;

            let data = coll
                .find_one(doc! {})
                .sort(doc! { MODIFIED_TIME_KEY: -1 })
                .await?;

            let latest_mt: Option<Timestamp> = data.and_then(|x| {
                x.get_str(MODIFIED_TIME_KEY)
                    .ok()
                    .and_then(|s| s.parse().ok())
            });

            let latest_mt_formatted = latest_mt
                .as_ref()
                .map(|t| t.format(Timestamp::DISPLAY_FORMAT));

            res.push(CollectionStats {
                name,
                count,
                latest_mt,
                latest_mt_formatted,
            });
        }

        Ok(res)
    }

    pub async fn create_unique_indexes(&self, params: Vec<CreateUniqueIndexParams>) -> Vec<String> {
        let mut res = Vec::with_capacity(params.len());

        for CreateUniqueIndexParams { collection, fields } in params {
            let coll: Collection<Document> = self.collection(&collection);
            let mut keys = Document::new();

            for field in fields {
                keys.insert(field, 1);
            }

            if let Ok(created) = coll
                .create_index(
                    IndexModel::builder()
                        .keys(keys)
                        .options(IndexOptions::builder().unique(true).build())
                        .build(),
                )
                .await
            {
                res.push(format!("{collection}: {}", created.index_name));
            }
        }

        res
    }
}

trait JsonValueExt {
    fn to_document(&self) -> Result<Document>;
}

impl JsonValueExt for JsonValue {
    fn to_document(&self) -> Result<Document> {
        let id = self
            .get(PRIMARY_KEY)
            .and_then(|id| ObjectId::from_str(id.as_str()?).ok());
        let mut value = to_document(self)?;

        if let Some(id) = id {
            value.insert(PRIMARY_KEY, Bson::ObjectId(id));
        }

        Ok(value)
    }
}

trait DocumentExt {
    fn to_json(&self) -> Result<JsonValue>;
}

impl DocumentExt for Document {
    fn to_json(&self) -> Result<JsonValue> {
        let id = self.get(PRIMARY_KEY).and_then(|id| id.as_object_id());
        let mut value = to_json(self)?;

        if let Some(id) = id {
            value[PRIMARY_KEY] = JsonValue::String(id.to_hex());
        }

        Ok(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStats {
    pub name: String,
    pub count: u64,
    pub latest_mt: Option<Timestamp>,
    // for display in backup meta
    pub latest_mt_formatted: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub timestamp: Timestamp,
    pub timestamp_formatted: String,
    pub stats: Vec<CollectionStats>,
}

impl Backup {
    pub fn new(stats: Vec<CollectionStats>) -> Self {
        let timestamp = Timestamp::now();
        let timestamp_formatted = timestamp.format(Timestamp::DISPLAY_FORMAT);

        Self {
            timestamp,
            timestamp_formatted,
            stats,
        }
    }
}
