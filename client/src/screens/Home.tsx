import { useState } from 'react'
import { Client as ProcessClient } from '@/lib/process'
import { Client as FsClient } from '@/lib/fs'
import { openExternal } from '@/lib'

const processClient = new ProcessClient()
const fsClient = new FsClient()

export function Home() {
  const [folderPath, setFolderPath] = useState('.')
  const [folderContents, setFolderContents] = useState<string[]>([])

  return (
    <div className="flex flex-col gap-6 p-6">
      <div className="text-2xl">Local Web App Starter</div>

      <div className="flex gap-3 items-center">
        <input
          type="text"
          placeholder="Folder Path"
          className="border p-2"
          value={folderPath}
          onChange={evt => setFolderPath(evt.currentTarget.value)}
        />

        <button
          className="border p-2 bg-zinc-200"
          onClick={() => openExternal(folderPath, processClient)}
        >
          Open Folder
        </button>

        <button
          className="border p-2 bg-zinc-200"
          onClick={async () => {
            const contents = await fsClient.readDir(folderPath)
            setFolderContents(contents)
          }}
        >
          Read Folder Contents
        </button>
      </div>

      <div className="flex flex-col gap-1 p-3 border self-start">
        <div className="font-bold">Folder Contents</div>

        {folderContents.map(item => (
          <div key={item}>{item}</div>
        ))}
      </div>

      <div className="text-lg">Open Assets from File System using getAssetUrl(...)</div>
      <div className="text-lg">Bypass CORS using getProxyUrl(...)</div>
    </div>
  )
}
