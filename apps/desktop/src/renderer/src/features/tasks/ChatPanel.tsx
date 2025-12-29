import { useState, useCallback, useRef, useEffect } from 'react'
import { Send, RefreshCw, Trash2, AlertCircle, User, Bot } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Textarea } from '@/components/ui/textarea'
import { useChatState } from '@/hooks/useAppState'
import type { ChatMessage } from '@/types/state'

/**
 * Chat Panel for Claude Code CLI integration.
 * Embedded in TasksPage when "Claude Code" is selected.
 */
export function ChatPanel() {
  const { chat, dispatch, isLoading } = useChatState()
  const [inputValue, setInputValue] = useState('')
  const scrollRef = useRef<HTMLDivElement>(null)

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight
    }
  }, [chat?.messages])

  const handleSend = useCallback(async () => {
    if (!inputValue.trim() || chat?.is_typing) return

    const text = inputValue.trim()
    setInputValue('')

    // Generate a unique ID for the user message
    const messageId = `user-${Date.now()}`
    const timestamp = new Date().toISOString()

    // Add user message immediately
    await dispatch({
      type: 'AddChatMessage',
      payload: {
        message: {
          id: messageId,
          role: 'user',
          content: text,
          timestamp,
        },
      },
    })

    // Trigger sending to Claude Code CLI
    await dispatch({
      type: 'SendChatMessage',
      payload: { text },
    })
  }, [inputValue, chat?.is_typing, dispatch])

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault()
        handleSend()
      }
    },
    [handleSend]
  )

  const handleClear = useCallback(async () => {
    await dispatch({ type: 'ClearChat' })
  }, [dispatch])

  const handleClearError = useCallback(async () => {
    await dispatch({ type: 'ClearChatError' })
  }, [dispatch])

  // Loading state
  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center rounded-lg border">
        <RefreshCw className="h-6 w-6 animate-spin text-muted-foreground" />
      </div>
    )
  }

  // No chat state (shouldn't happen if we have a project)
  if (!chat) {
    return (
      <div className="flex h-full flex-col items-center justify-center rounded-lg border">
        <Bot className="h-10 w-10 text-muted-foreground" />
        <p className="mt-2 text-sm text-muted-foreground">Chat unavailable</p>
      </div>
    )
  }

  const messages = chat.messages ?? []
  const isTyping = chat.is_typing
  const error = chat.error

  return (
    <div className="flex h-full flex-col rounded-lg border">
      {/* Header */}
      <div className="flex items-center justify-between border-b bg-muted/40 px-4 py-2">
        <div className="flex items-center gap-2">
          <Bot className="h-4 w-4 text-violet-500" />
          <span className="text-sm font-medium">Claude Code</span>
        </div>
        <Button
          variant="ghost"
          size="sm"
          onClick={handleClear}
          disabled={messages.length === 0}
          className="h-7 px-2"
        >
          <Trash2 className="h-3.5 w-3.5" />
        </Button>
      </div>

      {/* Messages Area */}
      <ScrollArea className="flex-1 p-3" ref={scrollRef}>
        {messages.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center py-8 text-center">
            <Bot className="h-12 w-12 text-muted-foreground opacity-40" />
            <p className="mt-3 text-sm text-muted-foreground max-w-[200px]">
              Ask Claude Code about your project
            </p>
          </div>
        ) : (
          <div className="space-y-3">
            {messages.map((message) => (
              <MessageBubble key={message.id} message={message} />
            ))}
            {isTyping && (
              <div className="flex items-center gap-2 text-muted-foreground text-sm">
                <RefreshCw className="h-3.5 w-3.5 animate-spin" />
                <span>Claude is thinking...</span>
              </div>
            )}
          </div>
        )}
      </ScrollArea>

      {/* Error Display */}
      {error && (
        <Card className="mx-3 mb-2 p-2 border-destructive bg-destructive/10">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-destructive">
              <AlertCircle className="h-3.5 w-3.5" />
              <span className="text-xs">{error}</span>
            </div>
            <Button variant="ghost" size="sm" onClick={handleClearError} className="h-6 px-2">
              <span className="text-xs">Dismiss</span>
            </Button>
          </div>
        </Card>
      )}

      {/* Input Area */}
      <div className="border-t p-3">
        <div className="flex gap-2">
          <Textarea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask Claude Code..."
            className="min-h-[60px] resize-none text-sm"
            disabled={isTyping}
          />
          <Button
            onClick={handleSend}
            disabled={!inputValue.trim() || isTyping}
            className="shrink-0 h-auto"
            size="sm"
          >
            {isTyping ? (
              <RefreshCw className="h-4 w-4 animate-spin" />
            ) : (
              <Send className="h-4 w-4" />
            )}
          </Button>
        </div>
        <p className="mt-1.5 text-[10px] text-muted-foreground">
          Enter to send, Shift+Enter for new line
        </p>
      </div>
    </div>
  )
}

function MessageBubble({ message }: { message: ChatMessage }) {
  const isUser = message.role === 'user'

  return (
    <div className={`flex items-start gap-2 ${isUser ? 'flex-row-reverse' : ''}`}>
      {/* Avatar */}
      <div
        className={`flex h-6 w-6 shrink-0 items-center justify-center rounded-full ${
          isUser ? 'bg-primary text-primary-foreground' : 'bg-violet-500 text-white'
        }`}
      >
        {isUser ? <User className="h-3 w-3" /> : <Bot className="h-3 w-3" />}
      </div>

      {/* Message Content */}
      <Card
        className={`max-w-[85%] px-3 py-1.5 text-sm ${
          isUser ? 'bg-primary text-primary-foreground' : 'bg-card'
        }`}
      >
        <div className="whitespace-pre-wrap break-words">{message.content}</div>
        {message.is_streaming && (
          <span className="inline-block w-1.5 h-3 ml-0.5 bg-current animate-pulse" />
        )}
      </Card>
    </div>
  )
}
