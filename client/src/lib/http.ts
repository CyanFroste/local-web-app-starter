import axios from 'axios'
import { getProxyUrl, stringifyQuery } from '@/lib/urls'
import type { AxiosError } from 'axios'
import type { Dictionary, Nullable } from '@/lib'
import type { StringifyOptions as QueryStringifyOptions } from '@/lib/urls'

export type Method = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE'
export type ResponseType = 'json' | 'text' | 'blob' | 'stream'
export type Headers = Dictionary<string>

export type Options<Q> = {
  method?: Method
  responseType?: ResponseType
  headers?: Headers
  query?: Q
  queryStringifyOptions?: QueryStringifyOptions
  proxy?: boolean
  body?: Nullable<
    | Dictionary
    | ReadonlyArray<Dictionary>
    | string
    | ArrayBuffer
    | Blob
    | FormData
    | URLSearchParams
    | Iterable<Uint8Array>
    | AsyncIterable<Uint8Array>
  >
}

export type Response<T> = { data: T; headers: Headers; status: number }

export type Client = {
  <T, Q extends Dictionary = Dictionary>(url: string, options?: Options<Q>): Promise<Response<T>>
  prefix?: string
}

export async function request<T, Q extends Dictionary = Dictionary>(
  url: string,
  {
    method = 'GET',
    responseType = 'json',
    query,
    body,
    proxy,
    headers,
    queryStringifyOptions = {},
  }: Options<Q> = {},
): Promise<Response<T>> {
  if (proxy) url = getProxyUrl(url)
  if (query) url += stringifyQuery(query, queryStringifyOptions)

  console.log('[http client request]', { method, url, responseType, body, headers })

  try {
    const res = await axios(url, {
      method,
      headers,
      responseType,
      ...(body && { data: body }),
    })

    const response = { data: res.data, headers: res.headers as Headers, status: res.status }

    console.log('[http client response]', {
      method,
      url,
      body,
      responseType,
      requestHeaders: headers,
      ...response,
    })

    return response
  } catch (err) {
    const error = err as AxiosError<Error>
    const message = error.response?.data?.message ?? error.message

    console.error('[http client error]', {
      message,
      error,
      method,
      url,
      headers,
      responseType,
      body,
    })

    throw new Error(message)
  }
}
