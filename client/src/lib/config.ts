import { getBridgeClientRequestor } from '@/lib/utils'
import type { Dictionary } from '@/lib'

export type Action = 'get'

export class Client {
  private request = getBridgeClientRequestor<Action>('config')

  async get(): Promise<Config> {
    return await this.request({ action: 'get', data: {} })
  }
}

export type Config = {
  port: number
  db: {
    mongo: { name: string; url: string }
    sqlite: { path: string }
  }
  theme: { fontSize: string }
  vars: Dictionary<Dictionary<string>>
}
