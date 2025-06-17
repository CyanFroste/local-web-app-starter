import { getBridgeClientRequestor } from '@/lib/utils'
import type { CollectionStats, CreateUniqueIndexParams } from '@/lib/db'

export type ExecutionResult = { rowsAffected: number; lastInsertRow: number }

export type Action = 'connect' | 'execute' | 'fetch' | 'drop' | 'stats' | 'create-unique-indexes'

export class Client {
  private request = getBridgeClientRequestor<Action>('db/sqlite')

  async connect(): Promise<void> {
    return await this.request({ action: 'connect', data: {} })
  }

  async execute(sql: string): Promise<ExecutionResult> {
    return await this.request({ action: 'execute', data: { sql } })
  }

  async fetch<T>(sql: string): Promise<T[]> {
    return await this.request({ action: 'fetch', data: { sql } })
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
