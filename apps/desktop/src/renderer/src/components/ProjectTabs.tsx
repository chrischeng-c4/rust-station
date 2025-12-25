/**
 * Project tabs component - displays open projects and worktrees at the top of the window.
 *
 * Two-level tab structure:
 * - Top row: Project tabs (git repos)
 * - Second row: Worktree sub-tabs (git worktrees within the active project)
 */

import { X, Plus, FolderOpen, GitBranch, ChevronDown, History } from 'lucide-react'
import { Button } from './ui/button'
import { useActiveProject, useActiveWorktree, useAppState } from '../hooks/useAppState'
import { cn } from '@/lib/utils'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from './ui/dropdown-menu'

export function ProjectTabs() {
  const { state } = useAppState()
  const { projects, activeIndex, dispatch } = useActiveProject()
  const { worktrees, activeWorktreeIndex, worktree } = useActiveWorktree()

  const recentProjects = state?.recent_projects ?? []

  const handleOpenProject = async () => {
    const path = await window.dialogApi.openFolder()
    if (path) {
      await dispatch({ type: 'OpenProject', payload: { path } })
    }
  }

  const handleSwitchProject = async (index: number) => {
    await dispatch({ type: 'SwitchProject', payload: { index } })
  }

  const handleCloseProject = async (e: React.MouseEvent, index: number) => {
    e.stopPropagation()
    await dispatch({ type: 'CloseProject', payload: { index } })
  }

  const handleSwitchWorktree = async (index: number) => {
    await dispatch({ type: 'SwitchWorktree', payload: { index } })
  }

  const handleOpenRecentProject = async (path: string) => {
    await dispatch({ type: 'OpenProject', payload: { path } })
  }

  // Check if worktree has unsaved changes
  const getWorktreeModified = (wt: typeof worktree) => {
    return wt?.is_modified ?? false
  }

  // Filter out already open projects from recent list
  const openProjectPaths = new Set(projects.map(p => p.path))
  const filteredRecentProjects = recentProjects.filter(r => !openProjectPaths.has(r.path))

  return (
    <div className="flex flex-col border-b bg-muted/30">
      {/* Project Tabs (Top Row) */}
      <div className="flex items-center gap-1 px-2 py-1 min-h-[40px]">
        {projects.length === 0 ? (
          <Button
            variant="ghost"
            size="sm"
            onClick={handleOpenProject}
            className="gap-2 text-muted-foreground"
          >
            <FolderOpen className="h-4 w-4" />
            Open Project
          </Button>
        ) : (
          <>
            {projects.map((project, index) => {
              // Check if any worktree in this project is modified
              const hasModifiedWorktree = project.worktrees.some(wt => wt.is_modified)

              return (
                <div
                  key={project.id}
                  onClick={() => handleSwitchProject(index)}
                  className={cn(
                    'flex items-center gap-2 px-3 py-1.5 rounded-md cursor-pointer transition-colors',
                    'hover:bg-accent group',
                    index === activeIndex
                      ? 'bg-background border shadow-sm'
                      : 'text-muted-foreground'
                  )}
                >
                  <span className="text-sm truncate max-w-[120px]">
                    {hasModifiedWorktree && <span className="text-yellow-500 mr-1">*</span>}
                    {project.name}
                  </span>
                  <button
                    onClick={(e) => handleCloseProject(e, index)}
                    className="opacity-0 group-hover:opacity-100 hover:bg-destructive/20 rounded p-0.5 transition-opacity"
                  >
                    <X className="h-3 w-3" />
                  </button>
                </div>
              )
            })}
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-7 w-7 text-muted-foreground"
                >
                  <Plus className="h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="start">
                <DropdownMenuItem onClick={handleOpenProject}>
                  <FolderOpen className="h-4 w-4 mr-2" />
                  Open Project...
                </DropdownMenuItem>
                {filteredRecentProjects.length > 0 && (
                  <>
                    <DropdownMenuSeparator />
                    <DropdownMenuLabel className="flex items-center gap-2">
                      <History className="h-3 w-3" />
                      Recent Projects
                    </DropdownMenuLabel>
                    {filteredRecentProjects.slice(0, 5).map((recent) => (
                      <DropdownMenuItem
                        key={recent.path}
                        onClick={() => handleOpenRecentProject(recent.path)}
                      >
                        <span className="truncate max-w-[200px]">
                          {recent.path.split('/').pop()}
                        </span>
                      </DropdownMenuItem>
                    ))}
                  </>
                )}
              </DropdownMenuContent>
            </DropdownMenu>
          </>
        )}
      </div>

      {/* Worktree Sub-Tabs (Second Row) - Only show if project has multiple worktrees */}
      {worktrees.length > 1 && (
        <div className="flex items-center gap-1 px-2 py-1 border-t border-border/50 bg-muted/20">
          <GitBranch className="h-3.5 w-3.5 text-muted-foreground mr-1" />
          {worktrees.map((wt, index) => (
            <div
              key={wt.id}
              onClick={() => handleSwitchWorktree(index)}
              className={cn(
                'flex items-center gap-1 px-2 py-1 rounded text-xs cursor-pointer transition-colors',
                'hover:bg-accent',
                index === activeWorktreeIndex
                  ? 'bg-background border shadow-sm font-medium'
                  : 'text-muted-foreground'
              )}
            >
              {getWorktreeModified(wt) && <span className="text-yellow-500">*</span>}
              <span className="truncate max-w-[100px]">{wt.branch}</span>
              {wt.is_main && (
                <span className="text-[10px] text-muted-foreground/70">(main)</span>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
