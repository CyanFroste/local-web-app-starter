import { useMutation } from '@tanstack/react-query'
import { getBridgeClientRequestor, timestamp } from '@/lib/utils'
import type { Dictionary, Nullable, PaginationParams } from '@/lib'
import type { CollectionStats, CreateUniqueIndexParams } from '@/lib/db'

export const PRIMARY_KEY = '_id'

export type QueryItemsParams = {
  collection: string
  pagination?: Nullable<PaginationParams>
  filters?: Nullable<Partial<Dictionary>>
  sort?: Nullable<Partial<Dictionary<number>>>
}

export type MutateItemsParams<T> = { collection: string; data: T[] }

export type Action =
  | 'connect'
  | 'find'
  | 'add'
  | 'update'
  | 'remove'
  | 'drop'
  | 'stats'
  | 'create-unique-indexes'

export class Client {
  private request = getBridgeClientRequestor<Action>('db/mongo')

  async connect(): Promise<void> {
    return await this.request({ action: 'connect', data: {} })
  }

  async find<T>(params: QueryItemsParams): Promise<WithId<T>[]> {
    return await this.request({ action: 'find', data: { params } })
  }

  async add<T>(params: MutateItemsParams<T>): Promise<WithId<T>[]> {
    return await this.request({ action: 'add', data: { params } })
  }

  async update<T>(params: MutateItemsParams<WithId<T>>): Promise<WithId<T>[]> {
    return await this.request({ action: 'update', data: { params } })
  }

  async remove<T>(params: MutateItemsParams<WithId<T>>): Promise<WithId<T>[]> {
    return await this.request({ action: 'remove', data: { params } })
  }

  async createUniqueIndexes(params: CreateUniqueIndexParams[]): Promise<string[]> {
    return await this.request({ action: 'create-unique-indexes', data: { params } })
  }

  async stats(): Promise<CollectionStats[]> {
    return await this.request({ action: 'stats', data: {} })
  }

  async drop(name: string): Promise<void> {
    return await this.request({ action: 'drop', data: { name } })
  }
}

export type WithId<T = unknown> = T & { [PRIMARY_KEY]: string }

export type Timestamps<T = string> = { ct: T; mt: T }

export function timestamps() {
  const ct = timestamp()
  return { ct, mt: ct }
}

type UseMutateItemsOptions = { collection: string; client: Client }

// ! WE'LL OVERWRITE THE TIMESTAMPS WHILE KEEPING THE TYPES SAME
// ! THIS WILL MAKE EVERY DOCUMENT HAVE TIMESTAMPS IRRESPECTIVE OF THE SPECIFIED TYPE
// ! ASSUMING THIS IS THE INTERFACE USED TO MUTATE THE DATABASE

export function useMutateItems<T>({ collection, client }: UseMutateItemsOptions) {
  const add = useMutation({
    mutationFn: (data: T[]) => client.add({ collection, data }),
  })

  const update = useMutation({
    mutationFn: (data: WithId<T>[]) => {
      const mt = timestamp()
      return client.update({ collection, data: data.map(it => ({ ...it, mt })) })
    },
  })

  const remove = useMutation({
    mutationFn: (data: WithId<T>[]) => client.remove({ collection, data }),
  })

  const drop = useMutation({
    mutationFn: () => client.drop(collection),
  })

  return { add, update, remove, drop }
}
