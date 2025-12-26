import '@testing-library/jest-dom'
import { vi } from 'vitest'

// Mock window.api (legacy Docker API)
const mockApi = {
  docker: {
    isAvailable: vi.fn().mockResolvedValue(true),
    listServices: vi.fn().mockResolvedValue([]),
    startService: vi.fn().mockResolvedValue(undefined),
    stopService: vi.fn().mockResolvedValue(undefined),
    restartService: vi.fn().mockResolvedValue(undefined),
    getLogs: vi.fn().mockResolvedValue([]),
    removeService: vi.fn().mockResolvedValue(undefined),
    createDatabase: vi.fn().mockResolvedValue('postgresql://localhost:5432/testdb'),
    createVhost: vi.fn().mockResolvedValue('amqp://localhost:5672/testvhost'),
  },
  justfile: {
    parse: vi.fn().mockResolvedValue([]),
    run: vi.fn().mockResolvedValue(''),
  },
}

// Mock window.stateApi (state-first API)
const mockStateApi = {
  dispatch: vi.fn().mockResolvedValue(undefined),
  getState: vi.fn().mockResolvedValue('{}'),
  onStateUpdate: vi.fn().mockReturnValue(() => {}),
}

// Mock window.dialogApi
const mockDialogApi = {
  openFolder: vi.fn().mockResolvedValue(null),
}

// Mock clipboard API
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn().mockResolvedValue(undefined),
  },
})

// Assign mocks to window
Object.defineProperty(window, 'api', {
  value: mockApi,
  writable: true,
})

Object.defineProperty(window, 'stateApi', {
  value: mockStateApi,
  writable: true,
})

Object.defineProperty(window, 'dialogApi', {
  value: mockDialogApi,
  writable: true,
})

// Export mocks for test files to access
export { mockApi, mockStateApi, mockDialogApi }
