import { getBridgeClientRequestor } from '@/lib/utils'
import type { Nullable } from '@/lib'

export type Options = { cwd?: string }

export type Output = { stdout: string; stderr: string; status: Nullable<number> }

export type Action = 'open' | 'output'

export class Client {
  private request = getBridgeClientRequestor<Action>('process')

  async open(path: string, using?: string): Promise<void> {
    return await this.request({ action: 'open', data: { path, using } })
  }

  async output(cmd: string, args: string[] = [], options: Options = {}): Promise<Output> {
    return await this.request({ action: 'output', data: { cmd, args, options } })
  }
}
