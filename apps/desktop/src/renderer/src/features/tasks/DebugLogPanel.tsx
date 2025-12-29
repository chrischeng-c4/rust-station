import { useCallback, useState } from 'react'
import { Terminal, Trash2, Copy, Check } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useChatState } from '@/hooks/useAppState'
import type { ClaudeDebugLog } from '@/types/state'

export function DebugLogPanel() {
  const { chat, dispatch } = useChatState()
  const [copied, setCopied] = useState(false)

  // Get debug logs from chat state
  const debugLogs = chat?.debug_logs ?? []

  const formatDebugLog = (log: ClaudeDebugLog): string => {
    const timestamp = new Date(log.timestamp).toLocaleTimeString()
    const eventType = log.event_type.toUpperCase().replace(/_/g, ' ')
    let formatted = `[${timestamp}] [${log.level.toUpperCase()}] [${eventType}] ${log.message}`

    if (log.details) {
      formatted += `\nDetails: ${JSON.stringify(log.details, null, 2)}`
    }

    return formatted
  }

  const handleCopy = useCallback(async () => {
    const text = debugLogs.map(formatDebugLog).join('\n\n')
    await navigator.clipboard.writeText(text)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }, [debugLogs])

  const handleClear = useCallback(async () => {
    await dispatch({ type: 'ClearDebugLogs' })
  }, [dispatch])

  return (
    <div className="flex h-full flex-col rounded-lg border">
      {/* Header */}
      <div className="flex items-center justify-between border-b bg-muted/40 px-4 py-2">
        <div className="flex items-center gap-2">
          <Terminal className="h-4 w-4" />
          <span className="text-sm font-medium">Debug Logs</span>
        </div>
        <div className="flex gap-1">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleCopy}
            disabled={debugLogs.length === 0}
            className="h-7 px-2"
            title="Copy all logs"
          >
            {copied ? (
              <Check className="h-3.5 w-3.5 text-green-500" />
            ) : (
              <Copy className="h-3.5 w-3.5" />
            )}
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleClear}
            disabled={debugLogs.length === 0}
            className="h-7 px-2"
            title="Clear all logs"
          >
            <Trash2 className="h-3.5 w-3.5" />
          </Button>
        </div>
      </div>

      {/* Log Content */}
      <ScrollArea className="flex-1 p-3">
        {debugLogs.length === 0 ? (
          <div className="flex h-full items-center justify-center text-muted-foreground">
            No debug logs yet
          </div>
        ) : (
          <div className="space-y-1 font-mono text-xs">
            {debugLogs.map((log, index) => (
              <LogEntry key={index} log={log} />
            ))}
          </div>
        )}
      </ScrollArea>
    </div>
  )
}

function LogEntry({ log }: { log: ClaudeDebugLog }) {
  const levelColor = {
    info: 'text-blue-500',
    debug: 'text-gray-500',
    error: 'text-red-500',
  }[log.level] || 'text-gray-500'

  const eventTypeLabel = {
    spawn_attempt: '🚀 SPAWN',
    spawn_success: '✅ STARTED',
    spawn_error: '❌ ERROR',
    stream_event: '📡 EVENT',
    message_complete: '🏁 COMPLETE',
    parse_error: '⚠️ PARSE ERROR',
  }[log.event_type] || log.event_type.toUpperCase()

  return (
    <div className="rounded bg-muted/50 p-2">
      <div className="flex items-center gap-2">
        <span className="text-gray-400">{new Date(log.timestamp).toLocaleTimeString()}</span>
        <span className={levelColor}>[{eventTypeLabel}]</span>
      </div>
      <div className="mt-1 whitespace-pre-wrap break-all">{log.message}</div>
      {log.details && (
        <pre className="mt-1 text-[10px] text-gray-400">
          {JSON.stringify(log.details, null, 2)}
        </pre>
      )}
    </div>
  )
}
