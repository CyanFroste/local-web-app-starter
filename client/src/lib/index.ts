import { clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'
import type { Client as ProcessClient } from './process'
import type { ClassValue } from 'clsx'

export async function openExternal(
  path: string,
  client: Nullable<ProcessClient> = null,
  using?: string,
) {
  if (client) return await client.open(path, using)
  window.open(path, using ?? '_blank')
}

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export type PaginationParams = { page?: number; limit?: number }

export type Nullable<T> = T | null | undefined

export type Dictionary<T = unknown> = Record<string, T>

declare module 'react' {
  interface CSSProperties {
    [key: `--${string}`]: string | number
  }
}
