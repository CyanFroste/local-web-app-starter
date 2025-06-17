import { request } from './http'

export type BridgeRequest<T, A extends string = string> = { action: A; data: T }

export type BridgeClientRequestor<A extends string = string> = <T, U>(
  req: BridgeRequest<T, A>,
) => Promise<U>

export function getBridgeClientRequestor<A extends string>(name: string): BridgeClientRequestor<A> {
  return async <T, U>(req: BridgeRequest<T>) =>
    (await request<U>(`/api/bridges/${name}`, { method: 'POST', body: req })).data
}

export function timestamp(date = new Date()) {
  return date.toISOString()
}
