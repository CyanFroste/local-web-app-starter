import type { Config } from '@/lib/config'

export function setTheme(config: Config) {
  document.documentElement.style.setProperty('--font-size', config.theme.fontSize)
}
