import { useCallback, useEffect, useState } from 'react'
import { Command } from 'cmdk'
import { styled, alpha } from '@mui/material/styles'
import {
  FolderOpen as FolderOpenIcon,
  AccountTree as GitBranchIcon,
  PlayArrow as PlayIcon,
  Dns as ContainerIcon,
  Settings as SettingsIcon,
  Code as FileCodeIcon,
  ListAlt as ListTodoIcon,
  WbSunny as SunIcon,
  DarkMode as MoonIcon,
  DesktopWindows as MonitorIcon,
  Storage as ServerIcon,
  ChatBubbleOutline as MessageSquareIcon,
  Terminal as TerminalSquareIcon,
} from '@mui/icons-material'
import { useAppState, useActiveProject, useActiveWorktree } from '@/hooks/useAppState'
import type { Theme } from '@/types/state'

// --- Styled Components ---

const StyledDialog = styled(Command.Dialog)(({ theme }) => ({
  position: 'fixed',
  inset: 0,
  zIndex: theme.zIndex.modal,
  display: 'flex',
  alignItems: 'flex-start',
  justifyContent: 'center',
  paddingTop: '20vh',
  backgroundColor: alpha(theme.palette.background.default, 0.5),
  backdropFilter: 'blur(4px)',
  
  '& .cmd-container': {
    width: '100%',
    maxWidth: 640,
    borderRadius: theme.shape.borderRadius,
    backgroundColor: theme.palette.background.paper,
    border: `1px solid ${theme.palette.divider}`,
    boxShadow: theme.shadows[8],
    overflow: 'hidden',
    display: 'flex',
    flexDirection: 'column',
  }
}))

const StyledInput = styled(Command.Input)(({ theme }) => ({
  width: '100%',
  padding: theme.spacing(2),
  fontSize: '1rem',
  border: 'none',
  borderBottom: `1px solid ${theme.palette.divider}`,
  backgroundColor: 'transparent',
  color: theme.palette.text.primary,
  outline: 'none',
  fontFamily: theme.typography.fontFamily,

  '&::placeholder': {
    color: theme.palette.text.secondary,
  }
}))

const StyledList = styled(Command.List)(({ theme }) => ({
  maxHeight: 400,
  overflowY: 'auto',
  padding: theme.spacing(1),
  
  '&::-webkit-scrollbar': {
    width: 8,
  },
  '&::-webkit-scrollbar-track': {
    background: 'transparent',
  },
  '&::-webkit-scrollbar-thumb': {
    backgroundColor: theme.palette.divider,
    borderRadius: 4,
  },
  '&::-webkit-scrollbar-thumb:hover': {
    backgroundColor: theme.palette.action.hover,
  }
}))

const StyledGroup = styled(Command.Group)(({ theme }) => ({
  marginBottom: theme.spacing(1),
  
  '& [cmdk-group-heading]': {
    padding: theme.spacing(1, 1.5, 0.5),
    fontSize: '0.75rem',
    fontWeight: 600,
    textTransform: 'uppercase',
    letterSpacing: '0.05em',
    color: theme.palette.text.secondary,
  }
}))

const StyledItem = styled(Command.Item)(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  gap: theme.spacing(1.5),
  padding: theme.spacing(1.25, 1.5),
  borderRadius: theme.shape.borderRadius / 2,
  fontSize: '0.875rem',
  color: theme.palette.text.primary,
  cursor: 'pointer',
  transition: 'background-color 0.1s',

  '&[data-selected="true"]': {
    backgroundColor: theme.palette.action.selected,
    color: theme.palette.text.primary,
    
    '& .command-icon': {
      color: theme.palette.primary.main,
    }
  },

  '& .command-icon': {
    width: 20,
    height: 20,
    color: theme.palette.text.secondary,
    flexShrink: 0,
  }
}))

const StyledEmpty = styled(Command.Empty)(({ theme }) => ({
  padding: theme.spacing(3),
  textAlign: 'center',
  color: theme.palette.text.secondary,
  fontSize: '0.875rem',
}))

const Badge = styled('span')(({ theme }) => ({
  marginLeft: 'auto',
  padding: theme.spacing(0.25, 1),
  fontSize: '0.7rem',
  fontWeight: 500,
  borderRadius: theme.shape.borderRadius / 4,
  backgroundColor: theme.palette.action.hover,
  color: theme.palette.text.secondary,
}))

const Description = styled('span')(({ theme }) => ({
  marginLeft: 'auto',
  fontSize: '0.75rem',
  color: theme.palette.text.secondary,
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  whiteSpace: 'nowrap',
  maxWidth: 200,
}))


interface CommandPaletteProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

/**
 * Global Command Palette (Cmd+K / Ctrl+K)
 * Provides quick navigation and action execution.
 */
export function CommandPalette({ open, onOpenChange }: CommandPaletteProps) {
  const { state, dispatch } = useAppState()
  const { projects, activeIndex } = useActiveProject()
  const { worktree, worktrees } = useActiveWorktree()
  const [search, setSearch] = useState('')

  // Get tasks from active worktree
  const tasks = worktree?.tasks?.commands ?? []

  // Reset search when closing
  useEffect(() => {
    if (!open) {
      setSearch('')
    }
  }, [open])

  // Handle project switch
  const handleSwitchProject = useCallback(
    async (index: number) => {
      await dispatch({ type: 'SwitchProject', payload: { index } })
      onOpenChange(false)
    },
    [dispatch, onOpenChange]
  )

  // Handle worktree switch
  const handleSwitchWorktree = useCallback(
    async (index: number) => {
      await dispatch({ type: 'SwitchWorktree', payload: { index } })
      onOpenChange(false)
    },
    [dispatch, onOpenChange]
  )

  // Handle view change
  const handleSetView = useCallback(
    async (view: 'tasks' | 'dockers' | 'settings' | 'env' | 'mcp' | 'chat' | 'terminal') => {
      await dispatch({ type: 'SetActiveView', payload: { view } })
      onOpenChange(false)
    },
    [dispatch, onOpenChange]
  )

  // Handle run task
  const handleRunTask = useCallback(
    async (taskName: string) => {
      if (!worktree) return
      await dispatch({
        type: 'RunJustCommand',
        payload: { name: taskName, cwd: worktree.path },
      })
      await dispatch({ type: 'SetActiveView', payload: { view: 'tasks' } })
      onOpenChange(false)
    },
    [worktree, dispatch, onOpenChange]
  )

  // Handle theme change
  const handleSetTheme = useCallback(
    async (theme: Theme) => {
      await dispatch({ type: 'SetTheme', payload: { theme } })
      onOpenChange(false)
    },
    [dispatch, onOpenChange]
  )

  // Memoize filtered items for performance
  const hasProjects = projects.length > 0
  const hasWorktrees = worktrees.length > 0
  const hasTasks = tasks.length > 0

  return (
    <StyledDialog
      open={open}
      onOpenChange={onOpenChange}
      label="Command Palette"
      data-testid="command-palette-dialog"
    >
      <div className="cmd-container">
        <StyledInput
          value={search}
          onValueChange={setSearch}
          placeholder="Type a command or search..."
        />
        <StyledList>
          <StyledEmpty>No results found.</StyledEmpty>

          {/* Projects */}
          {hasProjects && (
            <StyledGroup heading="Projects">
              {projects.map((project, index) => (
                <StyledItem
                  key={project.id}
                  value={`project ${project.name}`}
                  onSelect={() => handleSwitchProject(index)}
                >
                  <FolderOpenIcon className="command-icon" />
                  <span>{project.name}</span>
                  {index === activeIndex && (
                    <Badge>Active</Badge>
                  )}
                </StyledItem>
              ))}
            </StyledGroup>
          )}

          {/* Worktrees */}
          {hasWorktrees && (
            <StyledGroup heading="Worktrees">
              {worktrees.map((wt, index) => (
                <StyledItem
                  key={wt.id}
                  value={`worktree ${wt.branch}`}
                  onSelect={() => handleSwitchWorktree(index)}
                >
                  <GitBranchIcon className="command-icon" />
                  <span>{wt.branch}</span>
                  {wt.is_main && <Badge>main</Badge>}
                </StyledItem>
              ))}
            </StyledGroup>
          )}

          {/* Tasks */}
          {hasTasks && (
            <StyledGroup heading="Run Task">
              {tasks.slice(0, 10).map((task) => (
                <StyledItem
                  key={task.name}
                  value={`run task ${task.name} ${task.description ?? ''}`}
                  onSelect={() => handleRunTask(task.name)}
                >
                  <PlayIcon className="command-icon" />
                  <span>just {task.name}</span>
                  {task.description && (
                    <Description>{task.description}</Description>
                  )}
                </StyledItem>
              ))}
            </StyledGroup>
          )}

          {/* Views */}
          <StyledGroup heading="Views">
            <StyledItem
              value="view tasks"
              onSelect={() => handleSetView('tasks')}
            >
              <ListTodoIcon className="command-icon" />
              <span>Tasks</span>
            </StyledItem>
            <StyledItem
              value="view rstn-mcp integration server"
              onSelect={() => handleSetView('mcp')}
            >
              <ServerIcon className="command-icon" />
              <span>rstn-mcp Integration</span>
            </StyledItem>
            <StyledItem
              value="view chat claude ai"
              onSelect={() => handleSetView('chat')}
            >
              <MessageSquareIcon className="command-icon" />
              <span>Chat</span>
            </StyledItem>
            <StyledItem
              value="view terminal shell pty"
              onSelect={() => handleSetView('terminal')}
            >
              <TerminalSquareIcon className="command-icon" />
              <span>Terminal</span>
            </StyledItem>
            <StyledItem
              value="view docker containers"
              onSelect={() => handleSetView('dockers')}
            >
              <ContainerIcon className="command-icon" />
              <span>Docker</span>
            </StyledItem>
            <StyledItem
              value="view environment env"
              onSelect={() => handleSetView('env')}
            >
              <FileCodeIcon className="command-icon" />
              <span>Environment</span>
            </StyledItem>
            <StyledItem
              value="view settings preferences"
              onSelect={() => handleSetView('settings')}
            >
              <SettingsIcon className="command-icon" />
              <span>Settings</span>
            </StyledItem>
          </StyledGroup>

          {/* Theme */}
          <StyledGroup heading="Theme">
            <StyledItem
              value="theme system auto"
              onSelect={() => handleSetTheme('system')}
            >
              <MonitorIcon className="command-icon" />
              <span>System Theme</span>
            </StyledItem>
            <StyledItem
              value="theme light"
              onSelect={() => handleSetTheme('light')}
            >
              <SunIcon className="command-icon" />
              <span>Light Theme</span>
            </StyledItem>
            <StyledItem
              value="theme dark"
              onSelect={() => handleSetTheme('dark')}
            >
              <MoonIcon className="command-icon" />
              <span>Dark Theme</span>
            </StyledItem>
          </StyledGroup>
        </StyledList>
      </div>
    </StyledDialog>
  )
}
