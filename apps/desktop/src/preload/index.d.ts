import { ElectronAPI } from '@electron-toolkit/preload'

// Docker service types (matching Rust DockerService struct)
interface DockerService {
  id: string
  name: string
  image: string
  status: 'running' | 'stopped' | 'starting'
  port: number
  service_type: 'Database' | 'Cache' | 'MessageBroker' | 'Other'
}

// Justfile command types (matching Rust JustCommand struct)
interface JustCommand {
  name: string
  description: string | null
  recipe: string
}

// Branch info types (matching Rust BranchInfo struct)
interface BranchInfo {
  name: string
  hasWorktree: boolean
  isCurrent: boolean
}

// API exposed to renderer via contextBridge
// NOTE: This is the legacy API (React-first). Use stateApi for new code.
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
  mcp: {
    fetchTools(): Promise<string>
  }
  file: {
    /**
     * Read a file from allowed scopes (project root or ~/.rstn/).
     * @param path - Absolute path to the file
     * @param projectRoot - Project root directory (security scope)
     * @returns File contents as UTF-8 string
     * @throws Error with code: FILE_NOT_FOUND, PERMISSION_DENIED, SECURITY_VIOLATION, FILE_TOO_LARGE, NOT_UTF8
     */
    read(path: string, projectRoot: string): Promise<string>
  }
}

// Dialog API for native dialogs
interface DialogApi {
  /**
   * Open a native folder selection dialog.
   * @returns The selected folder path, or null if canceled
   */
  openFolder(): Promise<string | null>
}

// Screenshot API (dev mode)
interface ScreenshotApi {
  /**
   * Capture a screenshot of the entire window and save to Downloads folder.
   * @returns Result object with success flag and file path or error message
   */
  capture(): Promise<{ success: boolean; filePath?: string; error?: string }>
}

// State-first API
// This is the new architecture where Rust owns all state
interface StateApi {
  /**
   * Dispatch an action to update state.
   * @param action - Action object (will be JSON serialized)
   */
  dispatch(action: unknown): Promise<void>

  /**
   * Get the current state.
   * @returns JSON string of the current state
   */
  getState(): Promise<string>

  /**
   * Subscribe to state updates.
   * @param callback - Called with JSON string whenever state changes
   * @returns Unsubscribe function
   */
  onStateUpdate(callback: (stateJson: string) => void): () => void
}

declare global {
  interface Window {
    electron: ElectronAPI
    api: Api
    stateApi: StateApi
    dialogApi: DialogApi
    screenshotApi: ScreenshotApi
  }
}
