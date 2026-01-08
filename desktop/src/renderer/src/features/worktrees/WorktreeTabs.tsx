import { SyntheticEvent, useCallback } from 'react'
import { Box, Tabs, Tab, IconButton, Stack, styled, Chip } from '@mui/material'
import { Add as AddIcon, FolderOpen as WorktreeIcon } from '@mui/icons-material'
import { useActiveProject, useActiveWorktree, useAppState } from '@/hooks/useAppState'

const StyledTabs = styled(Tabs)(({ theme }) => ({
  minHeight: 40,
  backgroundColor: theme.palette.surfaceVariant?.main || theme.palette.background.paper,
  borderBottom: `1px solid ${theme.palette.outlineVariant.main}`,
  '& .MuiTabs-indicator': {
    height: 2,
    borderTopLeftRadius: 2,
    borderTopRightRadius: 2,
  },
}))

const StyledTab = styled(Tab)(({ theme }) => ({
  textTransform: 'none',
  minHeight: 40,
  fontWeight: 500,
  fontSize: '0.8125rem',
  color: theme.palette.onSurfaceVariant?.main || theme.palette.text.secondary,
  '&.Mui-selected': {
    color: theme.palette.secondary?.main || theme.palette.primary.main,
  },
  '&:hover': {
    backgroundColor: theme.palette.action.hover,
  },
}))

/**
 * WorktreeTabs - Level 2 tabs for switching between worktrees.
 * Only shown when a project is active.
 */
export function WorktreeTabs() {
  const { project, dispatch: projectDispatch } = useActiveProject()
  const { activeWorktreeIndex } = useActiveWorktree()
  const { dispatch: appDispatch } = useAppState()

  const handleChange = useCallback(
    (_event: SyntheticEvent, newValue: number) => {
      if (newValue === -1) {
        // This is the "Env" tab - switch to Env view
        appDispatch({ type: 'SetActiveView', payload: { view: 'env' } })
      } else {
        projectDispatch({ type: 'SwitchWorktree', payload: { index: newValue } })
      }
    },
    [projectDispatch, appDispatch]
  )

  const handleAddWorktree = useCallback(() => {
    console.log('Add worktree clicked - TODO: implement')
    // TODO: Open dialog to select branch or create new branch
  }, [])

  if (!project || project.worktrees.length === 0) {
    return null
  }

  return (
    <Box sx={{ width: '100%' }}>
      <Stack direction="row" alignItems="center">
        {/* Left: Worktree Tabs + Add Button */}
        <Box sx={{ flex: 1, overflow: 'hidden', display: 'flex', alignItems: 'center', borderBottom: 1, borderColor: 'outlineVariant.main' }}>
          <StyledTabs
            value={activeWorktreeIndex}
            onChange={handleChange}
            variant="scrollable"
            scrollButtons="auto"
            aria-label="worktree tabs"
          >
            {project.worktrees.map((worktree, index) => (
              <StyledTab
                key={worktree.id || index}
                label={
                  <Stack direction="row" alignItems="center" spacing={1}>
                    <WorktreeIcon fontSize="small" />
                    <span>{worktree.branch}</span>
                    {worktree.is_main && (
                      <Chip
                        label="main"
                        size="small"
                        sx={{
                          height: 18,
                          fontSize: '0.65rem',
                          fontWeight: 600,
                          ml: 0.5,
                        }}
                      />
                    )}
                  </Stack>
                }
              />
            ))}
          </StyledTabs>
          <IconButton size="small" onClick={handleAddWorktree} aria-label="add worktree" sx={{ mx: 0.5 }}>
            <AddIcon fontSize="small" />
          </IconButton>
        </Box>

        {/* Right: Environment Tab */}
        <Box
          sx={{
            borderBottom: 1,
            borderColor: 'outlineVariant.main',
            minHeight: 40,
            display: 'flex',
            alignItems: 'center',
            px: 1,
          }}
        >
          <StyledTab
            label="Env"
            onClick={() => appDispatch({ type: 'SetActiveView', payload: { view: 'env' } })}
            sx={{ minWidth: 60 }}
          />
        </Box>
      </Stack>
    </Box>
  )
}
