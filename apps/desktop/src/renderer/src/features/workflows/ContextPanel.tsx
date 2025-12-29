import { useCallback } from 'react'
import {
  FileText,
  RefreshCw,
  CheckCircle,
  AlertCircle,
  Sparkles,
  FolderOpen,
  Clock,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { useAppState } from '@/hooks/useAppState'
import ReactMarkdown from 'react-markdown'

/**
 * Context viewing panel for Living Context Layer (CESDD Phase 3).
 * Displays context files from .rstn/context/ directory.
 */
export function ContextPanel() {
  const { state, dispatch, isLoading } = useAppState()

  const activeProject = state?.projects?.[state?.active_project_index ?? 0]
  const worktree = activeProject?.worktrees?.[activeProject?.active_worktree_index ?? 0]
  const context = worktree?.context
  const contextFiles = context?.files ?? []
  const isInitialized = context?.is_initialized
  const lastRefreshed = context?.last_refreshed

  const handleInitialize = useCallback(async () => {
    await dispatch({ type: 'InitializeContext' })
  }, [dispatch])

  const handleRefresh = useCallback(async () => {
    await dispatch({ type: 'RefreshContext' })
  }, [dispatch])

  // Loading state
  if (isLoading || context?.is_loading) {
    return (
      <div className="flex h-full items-center justify-center rounded-lg border">
        <RefreshCw className="h-6 w-6 animate-spin text-muted-foreground" />
        <span className="ml-2 text-sm text-muted-foreground">Loading context...</span>
      </div>
    )
  }

  // Context not initialized
  if (!isInitialized) {
    return (
      <div className="flex h-full flex-col rounded-lg border">
        <div className="flex items-center justify-between border-b bg-muted/40 px-4 py-2">
          <div className="flex items-center gap-2">
            <AlertCircle className="h-4 w-4 text-amber-500" />
            <span className="text-sm font-medium">No Living Context</span>
          </div>
        </div>
        <div className="flex flex-1 items-center justify-center p-4">
          <Card className="max-w-md p-6 border-blue-500/50 bg-blue-50 dark:bg-blue-950/20">
            <FolderOpen className="mx-auto h-12 w-12 text-blue-500 mb-4" />
            <h3 className="text-lg font-medium mb-2 text-center">Initialize Living Context</h3>
            <p className="text-sm text-muted-foreground mb-4 text-center">
              Living Context maintains your project's current state - tech stack, architecture
              decisions, and recent changes. It's auto-curated as you complete changes.
            </p>
            <Button className="w-full" onClick={handleInitialize}>
              <Sparkles className="mr-2 h-4 w-4" />
              Initialize Context
            </Button>
            <p className="text-xs text-center text-muted-foreground mt-2">
              Creates <code>.rstn/context/</code> with default templates
            </p>
          </Card>
        </div>
      </div>
    )
  }

  // Context exists - show files
  return (
    <div className="flex h-full flex-col rounded-lg border">
      {/* Header */}
      <div className="flex items-center justify-between border-b bg-muted/40 px-4 py-2">
        <div className="flex items-center gap-2">
          <CheckCircle className="h-4 w-4 text-green-500" />
          <span className="text-sm font-medium">Living Context</span>
          <Badge variant="outline" className="text-xs">
            {contextFiles.length} files
          </Badge>
        </div>
        <div className="flex items-center gap-2">
          {lastRefreshed && (
            <span className="text-xs text-muted-foreground flex items-center gap-1">
              <Clock className="h-3 w-3" />
              {new Date(lastRefreshed).toLocaleTimeString()}
            </span>
          )}
          <Button variant="ghost" size="sm" onClick={handleRefresh}>
            <RefreshCw className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {/* Content */}
      {contextFiles.length === 0 ? (
        <div className="flex flex-1 items-center justify-center p-4">
          <Card className="p-6 text-center">
            <FileText className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
            <h3 className="text-lg font-medium mb-2">No Context Files</h3>
            <p className="text-sm text-muted-foreground mb-4">
              Context files will appear here after initialization or context sync.
            </p>
            <Button variant="outline" onClick={handleRefresh}>
              <RefreshCw className="mr-2 h-4 w-4" />
              Refresh
            </Button>
          </Card>
        </div>
      ) : (
        <Tabs defaultValue={contextFiles[0]?.name} className="flex-1 flex flex-col">
          <div className="border-b px-4 py-2">
            <TabsList className="h-auto flex-wrap gap-1">
              {contextFiles.map((file) => (
                <TabsTrigger key={file.name} value={file.name} className="text-xs gap-1">
                  <FileText className="h-3 w-3" />
                  {formatContextName(file.name)}
                </TabsTrigger>
              ))}
            </TabsList>
          </div>

          {contextFiles.map((file) => (
            <TabsContent key={file.name} value={file.name} className="flex-1 m-0">
              <ScrollArea className="h-full">
                <div className="p-4">
                  {/* File metadata */}
                  <div className="mb-4 flex items-center gap-4 text-xs text-muted-foreground">
                    <span className="flex items-center gap-1">
                      <Badge variant="secondary" className="text-xs">
                        {file.context_type}
                      </Badge>
                    </span>
                    {file.last_updated && (
                      <span className="flex items-center gap-1">
                        <Clock className="h-3 w-3" />
                        Updated: {formatDate(file.last_updated)}
                      </span>
                    )}
                    <span>~{file.token_estimate} tokens</span>
                  </div>

                  {/* File content */}
                  <Card className="p-4">
                    <div className="prose prose-sm dark:prose-invert max-w-none">
                      <ReactMarkdown>{file.content}</ReactMarkdown>
                    </div>
                  </Card>
                </div>
              </ScrollArea>
            </TabsContent>
          ))}
        </Tabs>
      )}
    </div>
  )
}

/** Format context file name for display */
function formatContextName(name: string): string {
  return name
    .replace(/-/g, ' ')
    .replace(/\b\w/g, (c) => c.toUpperCase())
}

/** Format ISO date string for display */
function formatDate(isoDate: string): string {
  try {
    return new Date(isoDate).toLocaleDateString()
  } catch {
    return isoDate
  }
}
