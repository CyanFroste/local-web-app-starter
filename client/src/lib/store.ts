import { getBridgeClientRequestor } from '@/lib/utils'
import type { Dictionary, Nullable } from '@/lib'

export type Action = 'get' | 'set' | 'remove' | 'entries' | 'keys'

export class Client {
  private request = getBridgeClientRequestor<Action>('store')

  async get<T>(key: string): Promise<Nullable<T>> {
    return await this.request({ action: 'get', data: { key } })
  }

  async set<T>(key: string, value: T): Promise<void> {
    return await this.request({ action: 'set', data: { key, value } })
  }

  async remove<T>(key: string): Promise<Nullable<T>> {
    return await this.request({ action: 'remove', data: { key } })
  }

  async entries<T>(): Promise<Dictionary<T>> {
    return await this.request({ action: 'entries', data: {} })
  }

  async keys(): Promise<string[]> {
    return await this.request({ action: 'keys', data: {} })
  }
}
