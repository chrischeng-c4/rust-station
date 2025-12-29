/// <reference types="vite/client" />

// Re-export preload types for renderer
import type { ElectronAPI } from '@electron-toolkit/preload'

// Import API types from preload
interface DockerService {
  id: string
  name: string
  image: string
  status: 'running' | 'stopped' | 'starting'
  port: number
  service_type: 'Database' | 'Cache' | 'MessageBroker' | 'Other'
}

interface JustCommand {
  name: string
  description: string | null
  recipe: string
}

interface BranchInfo {
  name: string
  hasWorktree: boolean
  isCurrent: boolean
}

interface Api {
  docker: {
    isAvailable(): Promise<boolean>
    listServices(): Promise<DockerService[]>
    startService(id: string): Promise<void>
    stopService(id: string): Promise<void>
    restartService(id: string): Promise<void>
    getLogs(id: string, tail?: number): Promise<string[]>
    removeService(id: string): Promise<void>
    createDatabase(id: string, dbName: string): Promise<string>
    createVhost(id: string, vhostName: string): Promise<string>
  }
  justfile: {
    parse(path: string): JustCommand[]
    run(command: string, cwd: string): string
  }
  worktree: {
    listBranches(repoPath: string): BranchInfo[]
  }
}

interface DialogApi {
  openFolder(): Promise<string | null>
}

interface ScreenshotApi {
  capture(): Promise<{ success: boolean; filePath?: string; error?: string }>
}

interface StateApi {
  dispatch(action: unknown): Promise<void>
  getState(): Promise<string>
  onStateUpdate(callback: (stateJson: string) => void): () => void
}

// Augment global Window interface
declare global {
  interface Window {
    electron: ElectronAPI
    api: Api
    stateApi: StateApi
    dialogApi: DialogApi
    screenshotApi: ScreenshotApi
  }
}

export {}
