import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'
import config from '../config.json' with { type: 'json' }

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
   resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
   build: {
    chunkSizeWarningLimit: 2000,
    outDir: '../bin/client',
    emptyOutDir: true,
    target: 'esnext',
  },
  server: {
    proxy: {
      '/api': {
        target: `http://localhost:${config.port}`,
        changeOrigin: true,
      },
    },
  },
})
