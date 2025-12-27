import { useState, useCallback } from 'react'
import { Bot, RefreshCw, AlertTriangle, Info, Save } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useAgentRulesState } from '@/hooks/useAppState'

/**
 * Agent Rules Management Page.
 * Allows customizing Claude Code's system prompt per project.
 */
export function AgentRulesPage() {
  const { agentRulesConfig, project, dispatch, isLoading } = useAgentRulesState()

  // Local state for textarea editing
  const [localPrompt, setLocalPrompt] = useState(agentRulesConfig?.custom_prompt ?? '')
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false)

  // Sync local state when config changes
  useState(() => {
    if (agentRulesConfig) {
      setLocalPrompt(agentRulesConfig.custom_prompt)
    }
  })

  const handleToggle = useCallback(async () => {
    if (!agentRulesConfig) return
    await dispatch({
      type: 'SetAgentRulesEnabled',
      payload: { enabled: !agentRulesConfig.enabled },
    })
  }, [agentRulesConfig, dispatch])

  const handlePromptChange = useCallback((value: string) => {
    setLocalPrompt(value)
    setHasUnsavedChanges(true)
  }, [])

  const handleSave = useCallback(async () => {
    await dispatch({
      type: 'SetAgentRulesPrompt',
      payload: { prompt: localPrompt },
    })
    setHasUnsavedChanges(false)
  }, [localPrompt, dispatch])

  const handleBlur = useCallback(() => {
    if (hasUnsavedChanges) {
      handleSave()
    }
  }, [hasUnsavedChanges, handleSave])

  // Loading state
  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <RefreshCw className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    )
  }

  // No project open
  if (!project || !agentRulesConfig) {
    return (
      <div className="flex h-full flex-col items-center justify-center">
        <Bot className="h-12 w-12 text-muted-foreground" />
        <h2 className="mt-4 text-xl font-semibold">No Project Open</h2>
        <p className="mt-2 text-muted-foreground">
          Open a project to customize Claude Code behavior.
        </p>
      </div>
    )
  }

  const charCount = localPrompt.length
  const isEnabled = agentRulesConfig.enabled

  return (
    <ScrollArea className="h-full">
      <div className="space-y-6 p-4">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-semibold">Agent Rules</h2>
            <p className="mt-1 text-muted-foreground">
              Custom system prompt for {project.name}
            </p>
          </div>
          <Button
            variant={isEnabled ? 'default' : 'outline'}
            onClick={handleToggle}
          >
            <Bot className="mr-2 h-4 w-4" />
            {isEnabled ? 'Enabled' : 'Disabled'}
          </Button>
        </div>

        {/* Warning Card */}
        {isEnabled && (
          <Card className="border-yellow-500/50 bg-yellow-500/10 p-4">
            <div className="flex items-start gap-3">
              <AlertTriangle className="h-5 w-5 text-yellow-600 dark:text-yellow-500 mt-0.5" />
              <div className="flex-1">
                <h3 className="font-medium text-yellow-900 dark:text-yellow-100">
                  Custom Rules Active
                </h3>
                <p className="mt-1 text-sm text-yellow-800 dark:text-yellow-200">
                  Your custom prompt will <strong>replace</strong> the default CLAUDE.md
                  instructions. Claude Code will follow only the rules you define below.
                </p>
              </div>
            </div>
          </Card>
        )}

        {/* Prompt Editor Card */}
        <Card className="p-4">
          <div className="flex items-center justify-between mb-4">
            <h3 className="flex items-center gap-2 text-lg font-medium">
              <Bot className="h-5 w-5" />
              Custom System Prompt
            </h3>
            {hasUnsavedChanges && (
              <Button size="sm" onClick={handleSave}>
                <Save className="mr-2 h-4 w-4" />
                Save Changes
              </Button>
            )}
          </div>

          <textarea
            value={localPrompt}
            onChange={(e) => handlePromptChange(e.target.value)}
            onBlur={handleBlur}
            placeholder="Enter custom instructions for Claude Code...&#10;&#10;Example:&#10;You are a Rust expert. Always use snake_case naming.&#10;Prefer Result over Option when errors are possible.&#10;Write comprehensive tests for all new functions."
            className="w-full min-h-[300px] rounded-md border border-input bg-background px-3 py-2 text-sm font-mono ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-y"
            disabled={!isEnabled}
          />

          <div className="mt-2 flex items-center justify-between text-xs text-muted-foreground">
            <span>{charCount.toLocaleString()} characters</span>
            {!isEnabled && (
              <span className="text-yellow-600 dark:text-yellow-500">
                Enable agent rules to edit
              </span>
            )}
          </div>
        </Card>

        {/* Info Card */}
        <Card className="p-4">
          <div className="flex items-start gap-3">
            <Info className="h-5 w-5 text-blue-600 dark:text-blue-400 mt-0.5" />
            <div className="flex-1 space-y-2">
              <h3 className="font-medium">How Agent Rules Work</h3>
              <ul className="space-y-1 text-sm text-muted-foreground list-disc list-inside">
                <li>
                  When enabled, your custom prompt is passed to Claude Code via{' '}
                  <code className="text-xs bg-muted px-1 py-0.5 rounded">
                    --system-prompt-file
                  </code>
                </li>
                <li>
                  The default CLAUDE.md instructions are <strong>completely replaced</strong>
                </li>
                <li>
                  Different projects can have different rules (project-scoped)
                </li>
                <li>
                  Changes auto-save when you click outside the text area
                </li>
                <li>
                  Temp file is created at{' '}
                  <code className="text-xs bg-muted px-1 py-0.5 rounded">
                    /tmp/rstn-agent-rules-*.txt
                  </code>{' '}
                  during chat sessions
                </li>
              </ul>
            </div>
          </div>
        </Card>

        {/* Example Card */}
        <Card className="p-4">
          <h3 className="mb-3 font-medium">Example Use Cases</h3>
          <div className="space-y-3 text-sm">
            <div>
              <p className="font-medium text-foreground">Rust Project:</p>
              <p className="text-muted-foreground">
                "Always use snake_case. Prefer Result over unwrap(). Write tests with
                #[test] annotations."
              </p>
            </div>
            <div>
              <p className="font-medium text-foreground">TypeScript Project:</p>
              <p className="text-muted-foreground">
                "Use strict TypeScript. Avoid 'any' types. Prefer functional programming
                patterns."
              </p>
            </div>
            <div>
              <p className="font-medium text-foreground">Team Standards:</p>
              <p className="text-muted-foreground">
                "Follow company coding standards: max 80 chars per line, JSDoc for all
                public APIs."
              </p>
            </div>
          </div>
        </Card>
      </div>
    </ScrollArea>
  )
}
