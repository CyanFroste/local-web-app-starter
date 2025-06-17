import qs from 'qs'
import type { IStringifyOptions as ExternalStringifyOptions } from 'qs'

const DEFAULT_STRINGIFY_OPTIONS: StringifyOptions = {
  encode: false,
  addQueryPrefix: true,
  arrayFormat: 'repeat',
}

export type StringifyOptions = ExternalStringifyOptions & { append?: boolean }

export function stringifyQuery<Q>(query: Q, options: StringifyOptions = {}) {
  const stringified = qs.stringify(query, { ...DEFAULT_STRINGIFY_OPTIONS, ...options })
  if (options.append) return '&' + stringified.slice(stringified.indexOf('?') + 1)
  return stringified
}

export function getProxyUrl(url: string) {
  return `/api/bridges/proxy/${url}`
}

export function getAssetUrl(path: string, container?: string) {
  return `/api/bridges/asset/${path}` + stringifyQuery({ container })
}
