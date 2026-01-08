/**
 * AppStateProvider - Single subscription to Rust state updates.
 *
 * This component manages the single IPC subscription to state updates,
 * eliminating the MaxListenersExceededWarning and improving performance
 * by parsing state JSON only once per update.
 */

import { createContext, useState, useEffect, useCallback, ReactNode } from 'react'
import type { AppState, Action } from '../types/state'

interface AppStateContextValue {
  /** Current application state (null if not yet loaded) */
  state: AppState | null
  /** Dispatch an action to update state */
  dispatch: (action: Action) => Promise<void>
  /** Whether state has been loaded */
  isLoading: boolean
}

export const AppStateContext = createContext<AppStateContextValue | null>(null)

interface AppStateProviderProps {
  children: ReactNode
}

/**
 * Provider component that manages the single state subscription.
 * Wrap your app with this component to enable state access via useAppState.
 */
export function AppStateProvider({ children }: AppStateProviderProps) {
  const [state, setState] = useState<AppState | null>(null)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    // Single subscription for the entire app
    const unsubscribe = window.stateApi.onStateUpdate((stateJson: string) => {
      try {
        const parsed = JSON.parse(stateJson) as AppState
        setState(parsed)
        setIsLoading(false)
      } catch (error) {
        console.error('Failed to parse state:', error)
      }
    })

    // Get initial state
    window.stateApi.getState().then((stateJson) => {
      try {
        const parsed = JSON.parse(stateJson) as AppState
        setState(parsed)
        setIsLoading(false)
      } catch (error) {
        console.error('Failed to parse initial state:', error)
      }
    })

    return unsubscribe
  }, [])

  const dispatch = useCallback(async (action: Action): Promise<void> => {
    await window.stateApi.dispatch(action)
  }, [])

  const value: AppStateContextValue = {
    state,
    dispatch,
    isLoading,
  }

  return <AppStateContext.Provider value={value}>{children}</AppStateContext.Provider>
}
