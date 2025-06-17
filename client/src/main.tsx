import '@/styles.css'

import React from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createBrowserRouter } from 'react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { Client as ConfigClient } from '@/lib/config'
import { Client as SqliteClient } from '@/lib/db/sqlite'
import { Client as MongoClient } from '@/lib/db/mongo'
import { Home } from '@/screens/Home'
import { setTheme } from '@/utils'

const configClient = new ConfigClient()
const config = await configClient.get()

setTheme(config)

const mongoClient = new MongoClient()
const sqliteClient = new SqliteClient()

// connect once from anywhere
// there can be multiple instances of the same db client
mongoClient.connect().catch(err => console.error(err))
sqliteClient.connect().catch(err => console.error(err))

const router = createBrowserRouter([{ path: '/', Component: Home }])

const queryClient = new QueryClient({
  defaultOptions: {
    queries: { retry: false, staleTime: 2 * 60 * 1000, refetchOnMount: 'always' },
  },
})

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </React.StrictMode>,
)
