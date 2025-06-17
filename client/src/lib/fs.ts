import { getBridgeClientRequestor } from '@/lib/utils'

export type Stats = {
  size: number
  isDir: boolean
  isFile: boolean
  modifiedTime: number
}

export type Action =
  | 'stats'
  | 'read-dir'
  | 'create-dir'
  | 'rename'
  | 'remove'
  | 'copy-file'
  | 'read-text-file'
  | 'write-text-file'
  | 'download-file'
  | 'read-archive'

export class Client {
  private request = getBridgeClientRequestor<Action>('fs')

  async stats(path: string): Promise<Stats> {
    return await this.request({ action: 'stats', data: { path } })
  }

  async readDir(path: string): Promise<string[]> {
    return await this.request({ action: 'read-dir', data: { path } })
  }

  async createDir(path: string, recursive?: boolean): Promise<void> {
    return await this.request({ action: 'create-dir', data: { path, recursive } })
  }

  async rename(src: string, dst: string): Promise<void> {
    return await this.request({ action: 'rename', data: { src, dst } })
  }

  async remove(path: string, recursive?: boolean): Promise<void> {
    return await this.request({ action: 'remove', data: { path, recursive } })
  }

  async copyFile(src: string, dst: string): Promise<void> {
    return await this.request({ action: 'copy-file', data: { src, dst } })
  }

  async readTextFile(path: string): Promise<string> {
    return await this.request({ action: 'read-text-file', data: { path } })
  }

  async writeTextFile(path: string, data: string): Promise<void> {
    return await this.request({ action: 'write-text-file', data: { path, data } })
  }

  async downloadFile(url: string, path: string): Promise<void> {
    return await this.request({ action: 'download-file', data: { url, path } })
  }

  async readArchive(path: string, container: string): Promise<string[]> {
    return await this.request({ action: 'read-archive', data: { path, container } })
  }
}

export function getFileNameExtension(value: string) {
  return value.slice(((value.lastIndexOf('.') - 1) >>> 0) + 2)
}
