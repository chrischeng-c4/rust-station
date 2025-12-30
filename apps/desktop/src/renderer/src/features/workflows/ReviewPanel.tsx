import { useCallback, useState, useMemo } from 'react'
import {
  FileText,
  MessageSquare,
  CheckCircle,
  XCircle,
  AlertCircle,
  ChevronRight,
  Plus,
  RefreshCw,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Textarea } from '@/components/ui/textarea'
import { useAppState } from '@/hooks/useAppState'
import ReactMarkdown from 'react-markdown'
import type {
  ReviewSession,
  CommentTarget,
  ReviewStatus,
  ReviewContentType,
} from '@/types/state'

// ============================================================================
// Status Configuration
// ============================================================================

const STATUS_CONFIG: Record<
  ReviewStatus,
  { label: string; color: string; bgClass: string; textClass: string }
> = {
  pending: {
    label: 'Pending',
    color: 'gray',
    bgClass: 'bg-gray-500/10',
    textClass: 'text-gray-700 dark:text-gray-300',
  },
  reviewing: {
    label: 'Reviewing',
    color: 'blue',
    bgClass: 'bg-blue-500/10',
    textClass: 'text-blue-700 dark:text-blue-300',
  },
  iterating: {
    label: 'Iterating',
    color: 'yellow',
    bgClass: 'bg-yellow-500/10',
    textClass: 'text-yellow-700 dark:text-yellow-300',
  },
  approved: {
    label: 'Approved',
    color: 'green',
    bgClass: 'bg-green-500/10',
    textClass: 'text-green-700 dark:text-green-300',
  },
  rejected: {
    label: 'Rejected',
    color: 'red',
    bgClass: 'bg-red-500/10',
    textClass: 'text-red-700 dark:text-red-300',
  },
}

const CONTENT_TYPE_LABELS: Record<ReviewContentType, string> = {
  Plan: 'Plan',
  Proposal: 'Proposal',
  Code: 'Code',
  Artifact: 'Artifact',
}

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Parse markdown content to extract section markers (H1, H2 headings).
 */
function parseMarkdownSections(content: string): { id: string; title: string; level: number }[] {
  const sections: { id: string; title: string; level: number }[] = []
  const lines = content.split('\n')

  lines.forEach((line) => {
    const h1Match = line.match(/^#\s+(.+)$/)
    const h2Match = line.match(/^#{2}\s+(.+)$/)

    if (h1Match) {
      const title = h1Match[1].trim()
      const id = title.toLowerCase().replace(/\s+/g, '-').replace(/[^\w-]/g, '')
      sections.push({ id, title, level: 1 })
    } else if (h2Match) {
      const title = h2Match[1].trim()
      const id = title.toLowerCase().replace(/\s+/g, '-').replace(/[^\w-]/g, '')
      sections.push({ id, title, level: 2 })
    }
  })

  return sections
}

/**
 * Get display text for comment target.
 */
function getTargetDisplay(target: CommentTarget): string {
  if (target.type === 'document') return 'General'
  if (target.type === 'section') return `Section: ${target.id}`
  if (target.type === 'file') return `File: ${target.path}`
  return 'Unknown'
}

// ============================================================================
// Sub-Components
// ============================================================================

interface ContentViewProps {
  session: ReviewSession
  onSectionClick: (sectionId: string) => void
}

function ContentView({ session, onSectionClick }: ContentViewProps) {
  const sections = useMemo(() => parseMarkdownSections(session.content.content), [session.content.content])

  return (
    <div className="flex h-full flex-col">
      {/* Content Header */}
      <div className="border-b bg-muted/40 px-4 py-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <FileText className="h-4 w-4" />
            <span className="text-sm font-medium">
              {CONTENT_TYPE_LABELS[session.content.content_type]}
            </span>
            {session.iteration > 1 && (
              <Badge variant="outline" className="text-xs">
                Iteration {session.iteration}
              </Badge>
            )}
          </div>
          <Badge className={STATUS_CONFIG[session.status].bgClass}>
            <span className={STATUS_CONFIG[session.status].textClass}>
              {STATUS_CONFIG[session.status].label}
            </span>
          </Badge>
        </div>
      </div>

      {/* Section Markers */}
      {sections.length > 0 && (
        <div className="border-b bg-muted/20 px-4 py-2">
          <div className="text-xs font-medium text-muted-foreground mb-1">Sections:</div>
          <div className="flex flex-wrap gap-1">
            {sections.map((section) => (
              <Button
                key={section.id}
                variant="ghost"
                size="sm"
                className="h-6 px-2 text-xs"
                onClick={() => onSectionClick(section.id)}
              >
                {section.level === 1 ? '# ' : '## '}
                {section.title}
              </Button>
            ))}
          </div>
        </div>
      )}

      {/* Content Body */}
      <ScrollArea className="flex-1 p-4">
        <div className="prose prose-sm dark:prose-invert max-w-none">
          <ReactMarkdown>{session.content.content}</ReactMarkdown>
        </div>

        {/* File Changes */}
        {session.content.file_changes.length > 0 && (
          <Card className="mt-4">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm">File Changes</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2">
              {session.content.file_changes.map((change, idx) => (
                <div key={idx} className="flex items-start gap-2 text-xs">
                  <Badge
                    variant={
                      change.action === 'create'
                        ? 'default'
                        : change.action === 'modify'
                          ? 'secondary'
                          : 'destructive'
                    }
                    className="mt-0.5 h-5"
                  >
                    {change.action}
                  </Badge>
                  <div className="flex-1">
                    <code className="text-xs">{change.path}</code>
                    <p className="text-muted-foreground">{change.summary}</p>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>
        )}
      </ScrollArea>
    </div>
  )
}

interface CommentsSidebarProps {
  session: ReviewSession
  onAddComment: (target: CommentTarget, content: string) => void
  onResolveComment: (commentId: string) => void
}

function CommentsSidebar({ session, onAddComment, onResolveComment }: CommentsSidebarProps) {
  const [newCommentContent, setNewCommentContent] = useState('')
  const [newCommentTarget, setNewCommentTarget] = useState<CommentTarget>({ type: 'document' })

  const handleAddComment = useCallback(() => {
    if (!newCommentContent.trim()) return
    onAddComment(newCommentTarget, newCommentContent.trim())
    setNewCommentContent('')
    setNewCommentTarget({ type: 'document' })
  }, [newCommentContent, newCommentTarget, onAddComment])

  const unresolvedComments = session.comments.filter((c) => !c.resolved)
  const resolvedComments = session.comments.filter((c) => c.resolved)

  return (
    <div className="flex h-full w-80 flex-col border-l">
      {/* Sidebar Header */}
      <div className="border-b bg-muted/40 px-4 py-2">
        <div className="flex items-center gap-2">
          <MessageSquare className="h-4 w-4" />
          <span className="text-sm font-medium">Comments</span>
          <Badge variant="secondary" className="ml-auto">
            {unresolvedComments.length}
          </Badge>
        </div>
      </div>

      {/* Comments List */}
      <ScrollArea className="flex-1 p-4">
        <div className="space-y-3">
          {/* Unresolved Comments */}
          {unresolvedComments.length > 0 && (
            <div>
              <div className="text-xs font-medium text-muted-foreground mb-2">Active</div>
              {unresolvedComments.map((comment) => (
                <Card key={comment.id} className="mb-2 p-3">
                  <div className="flex items-start justify-between gap-2 mb-2">
                    <Badge variant="outline" className="text-xs">
                      {getTargetDisplay(comment.target)}
                    </Badge>
                    <Button
                      variant="ghost"
                      size="sm"
                      className="h-5 w-5 p-0"
                      onClick={() => onResolveComment(comment.id)}
                    >
                      <CheckCircle className="h-3.5 w-3.5 text-green-500" />
                    </Button>
                  </div>
                  <p className="text-xs text-foreground whitespace-pre-wrap">{comment.content}</p>
                  <div className="mt-2 text-xs text-muted-foreground">
                    {comment.author === 'user' ? 'You' : 'System'} •{' '}
                    {new Date(comment.created_at).toLocaleTimeString()}
                  </div>
                </Card>
              ))}
            </div>
          )}

          {/* Resolved Comments */}
          {resolvedComments.length > 0 && (
            <div>
              <div className="text-xs font-medium text-muted-foreground mb-2">Resolved</div>
              {resolvedComments.map((comment) => (
                <Card key={comment.id} className="mb-2 p-3 opacity-60">
                  <div className="flex items-start justify-between gap-2 mb-2">
                    <Badge variant="outline" className="text-xs">
                      {getTargetDisplay(comment.target)}
                    </Badge>
                    <CheckCircle className="h-3.5 w-3.5 text-green-500" />
                  </div>
                  <p className="text-xs text-foreground whitespace-pre-wrap">{comment.content}</p>
                  <div className="mt-2 text-xs text-muted-foreground">
                    {comment.author === 'user' ? 'You' : 'System'} •{' '}
                    {new Date(comment.created_at).toLocaleTimeString()}
                  </div>
                </Card>
              ))}
            </div>
          )}

          {session.comments.length === 0 && (
            <div className="flex flex-col items-center justify-center py-8 text-center">
              <MessageSquare className="h-8 w-8 text-muted-foreground mb-2" />
              <p className="text-xs text-muted-foreground">No comments yet</p>
            </div>
          )}
        </div>
      </ScrollArea>

      {/* Add Comment Form */}
      <div className="border-t p-4 space-y-2">
        <Textarea
          value={newCommentContent}
          onChange={(e) => setNewCommentContent(e.target.value)}
          placeholder="Add a comment..."
          className="min-h-[80px] resize-none text-xs"
        />
        <div className="flex items-center gap-2">
          <select
            value={newCommentTarget.type}
            onChange={(e) => {
              const type = e.target.value as 'document' | 'section' | 'file'
              if (type === 'document') {
                setNewCommentTarget({ type: 'document' })
              } else if (type === 'section') {
                setNewCommentTarget({ type: 'section', id: '' })
              } else {
                setNewCommentTarget({ type: 'file', path: '' })
              }
            }}
            className="flex h-8 rounded-md border border-input bg-background px-2 py-1 text-xs"
          >
            <option value="document">General</option>
            <option value="section">Section</option>
            <option value="file">File</option>
          </select>
          <Button
            size="sm"
            onClick={handleAddComment}
            disabled={!newCommentContent.trim()}
            className="flex-1"
          >
            <Plus className="mr-1 h-3 w-3" />
            Add
          </Button>
        </div>
      </div>
    </div>
  )
}

interface ActionBarProps {
  session: ReviewSession
  onApprove: () => void
  onRequestChanges: () => void
  onReject: () => void
}

function ActionBar({ session, onApprove, onRequestChanges, onReject }: ActionBarProps) {
  const canApprove = session.status === 'reviewing'
  const canRequestChanges = session.status === 'reviewing'
  const canReject = session.status === 'reviewing'

  return (
    <div className="border-t bg-muted/20 px-4 py-3">
      <div className="flex items-center gap-2">
        <Button
          variant="default"
          size="sm"
          onClick={onApprove}
          disabled={!canApprove}
          className="bg-green-600 hover:bg-green-700"
        >
          <CheckCircle className="mr-1 h-4 w-4" />
          Approve
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={onRequestChanges}
          disabled={!canRequestChanges}
          className="border-yellow-500 text-yellow-700 dark:text-yellow-300"
        >
          <AlertCircle className="mr-1 h-4 w-4" />
          Request Changes
        </Button>
        <Button
          variant="destructive"
          size="sm"
          onClick={onReject}
          disabled={!canReject}
          className="ml-auto"
        >
          <XCircle className="mr-1 h-4 w-4" />
          Reject
        </Button>
      </div>
    </div>
  )
}

// ============================================================================
// Main Component
// ============================================================================

export function ReviewPanel() {
  const { state, dispatch, isLoading } = useAppState()

  const activeProject = state?.projects?.[state?.active_project_index ?? 0]
  const worktree = activeProject?.worktrees?.[activeProject?.active_worktree_index ?? 0]
  const reviewGate = worktree?.tasks?.review_gate

  const activeSession = useMemo(() => {
    if (!reviewGate?.active_session_id || !reviewGate.sessions) return null
    return reviewGate.sessions[reviewGate.active_session_id] ?? null
  }, [reviewGate])

  const allSessions = useMemo(() => {
    if (!reviewGate?.sessions) return []
    return Object.values(reviewGate.sessions).sort(
      (a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    )
  }, [reviewGate])

  // Handlers
  const handleSessionSelect = useCallback(
    (sessionId: string) => {
      dispatch({ type: 'SetActiveReviewSession', payload: { session_id: sessionId } })
    },
    [dispatch]
  )

  const handleSectionClick = useCallback((sectionId: string) => {
    // Scroll to section in content view
    const element = document.getElementById(sectionId)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' })
    }
  }, [])

  const handleAddComment = useCallback(
    (target: CommentTarget, content: string) => {
      if (!activeSession) return
      dispatch({
        type: 'AddReviewComment',
        payload: {
          session_id: activeSession.id,
          target,
          content,
        },
      })
    },
    [activeSession, dispatch]
  )

  const handleResolveComment = useCallback(
    (commentId: string) => {
      if (!activeSession) return
      dispatch({
        type: 'ResolveReviewComment',
        payload: {
          session_id: activeSession.id,
          comment_id: commentId,
        },
      })
    },
    [activeSession, dispatch]
  )

  const handleApprove = useCallback(() => {
    if (!activeSession) return
    dispatch({
      type: 'ApproveReview',
      payload: { session_id: activeSession.id },
    })
  }, [activeSession, dispatch])

  const handleRequestChanges = useCallback(() => {
    if (!activeSession) return
    dispatch({
      type: 'SubmitReviewFeedback',
      payload: { session_id: activeSession.id },
    })
  }, [activeSession, dispatch])

  const handleReject = useCallback(() => {
    if (!activeSession) return
    const reason = prompt('Reason for rejection (optional):')
    dispatch({
      type: 'RejectReview',
      payload: {
        session_id: activeSession.id,
        reason: reason || 'Rejected by user',
      },
    })
  }, [activeSession, dispatch])

  // Loading state
  if (isLoading || reviewGate?.is_loading) {
    return (
      <div className="flex h-full items-center justify-center rounded-lg border">
        <RefreshCw className="h-6 w-6 animate-spin text-muted-foreground" />
        <span className="ml-2 text-sm text-muted-foreground">Loading review sessions...</span>
      </div>
    )
  }

  // Error state
  if (reviewGate?.error) {
    return (
      <div className="flex h-full flex-col items-center justify-center rounded-lg border p-8">
        <XCircle className="h-12 w-12 text-red-500 mb-4" />
        <p className="text-sm font-medium">Failed to load review sessions</p>
        <p className="text-xs text-muted-foreground mt-1">{reviewGate.error}</p>
      </div>
    )
  }

  // No sessions state
  if (allSessions.length === 0) {
    return (
      <div className="flex h-full flex-col items-center justify-center rounded-lg border p-8">
        <MessageSquare className="h-12 w-12 text-muted-foreground mb-4" />
        <p className="text-sm font-medium">No review sessions</p>
        <p className="text-xs text-muted-foreground mt-1 text-center">
          Review sessions will appear here when Claude generates artifacts that require review.
        </p>
      </div>
    )
  }

  // Multiple sessions - show session selector
  if (allSessions.length > 1 && !activeSession) {
    return (
      <div className="flex h-full flex-col rounded-lg border">
        <div className="border-b bg-muted/40 px-4 py-2">
          <div className="flex items-center gap-2">
            <MessageSquare className="h-4 w-4" />
            <span className="text-sm font-medium">Review Sessions</span>
            <Badge variant="secondary" className="ml-auto">
              {allSessions.length}
            </Badge>
          </div>
        </div>

        <ScrollArea className="flex-1 p-4">
          <div className="space-y-2">
            {allSessions.map((session) => (
              <Card
                key={session.id}
                className="p-4 cursor-pointer hover:bg-muted/50 transition-colors"
                onClick={() => handleSessionSelect(session.id)}
              >
                <div className="flex items-start justify-between gap-2 mb-2">
                  <div className="flex items-center gap-2">
                    <FileText className="h-4 w-4" />
                    <span className="text-sm font-medium">
                      {CONTENT_TYPE_LABELS[session.content.content_type]}
                    </span>
                  </div>
                  <Badge className={STATUS_CONFIG[session.status].bgClass}>
                    <span className={STATUS_CONFIG[session.status].textClass}>
                      {STATUS_CONFIG[session.status].label}
                    </span>
                  </Badge>
                </div>
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                  <span>{new Date(session.created_at).toLocaleString()}</span>
                  {session.iteration > 1 && (
                    <>
                      <span>•</span>
                      <span>Iteration {session.iteration}</span>
                    </>
                  )}
                  {session.comments.length > 0 && (
                    <>
                      <span>•</span>
                      <MessageSquare className="h-3 w-3 inline" />
                      <span>{session.comments.filter((c) => !c.resolved).length}</span>
                    </>
                  )}
                </div>
                <ChevronRight className="h-4 w-4 absolute top-4 right-4 text-muted-foreground" />
              </Card>
            ))}
          </div>
        </ScrollArea>
      </div>
    )
  }

  // Active session view
  if (!activeSession) {
    return (
      <div className="flex h-full items-center justify-center rounded-lg border">
        <p className="text-sm text-muted-foreground">No active review session</p>
      </div>
    )
  }

  return (
    <div className="flex h-full flex-col rounded-lg border">
      {/* Session selector (if multiple sessions exist) */}
      {allSessions.length > 1 && (
        <div className="border-b bg-muted/40 px-4 py-2">
          <select
            value={activeSession.id}
            onChange={(e) => handleSessionSelect(e.target.value)}
            className="w-full rounded-md border border-input bg-background px-3 py-1 text-sm"
          >
            {allSessions.map((session) => (
              <option key={session.id} value={session.id}>
                {CONTENT_TYPE_LABELS[session.content.content_type]} -{' '}
                {new Date(session.created_at).toLocaleString()} -{' '}
                {STATUS_CONFIG[session.status].label}
              </option>
            ))}
          </select>
        </div>
      )}

      {/* Two-column layout: Content + Comments */}
      <div className="flex flex-1 overflow-hidden">
        <div className="flex-1">
          <ContentView session={activeSession} onSectionClick={handleSectionClick} />
        </div>
        <CommentsSidebar
          session={activeSession}
          onAddComment={handleAddComment}
          onResolveComment={handleResolveComment}
        />
      </div>

      {/* Action Bar */}
      <ActionBar
        session={activeSession}
        onApprove={handleApprove}
        onRequestChanges={handleRequestChanges}
        onReject={handleReject}
      />
    </div>
  )
}
