import React, { SyntheticEvent, useCallback } from 'react'
import { Box, Tabs, Tab, IconButton, Stack, styled } from '@mui/material'
import { Add as AddIcon, Close as CloseIcon, Folder as ProjectIcon } from '@mui/icons-material'

import { useActiveProject, useAppState } from '@/hooks/useAppState'
import { GlobalIconBar } from '@/components/layout/GlobalIconBar'

// Custom Styled Tab for M3 "Chrome-like" or "Folder-like" appearance if desired
// For now, adhering to standard M3 Tabs spec
const StyledTabs = styled(Tabs)(({ theme }) => ({
  minHeight: 48,
  backgroundColor: theme.palette.background.default,
  borderBottom: `1px solid ${theme.palette.outlineVariant.main}`,
  '& .MuiTabs-indicator': {
    height: 3,
    borderTopLeftRadius: 3,
    borderTopRightRadius: 3,
  },
}))

const StyledTab = styled(Tab)(({ theme }) => ({
  textTransform: 'none',
  minHeight: 48,
  fontWeight: 500,
  fontSize: '0.875rem',
  color: theme.palette.onSurfaceVariant.main,
  '&.Mui-selected': {
    color: theme.palette.primary.main,
  },
  '&:hover': {
    backgroundColor: theme.palette.action.hover,
  },
}))

export function ProjectTabs() {
  const { projects, activeIndex, dispatch } = useActiveProject()
  const { dispatch: appDispatch } = useAppState()

  const handleChange = useCallback((_event: SyntheticEvent, newValue: number) => {
    dispatch({ type: 'SwitchProject', payload: { index: newValue } })
  }, [dispatch])

  const handleClose = useCallback(async (e: React.MouseEvent, index: number) => {
    e.stopPropagation()
    // TODO: Implement CloseProject action in backend if not exists, 
    // or assume we can't close for now without backend support
    // dispatch({ type: 'CloseProject', payload: { index } }) 
  }, [dispatch])

  const handleAdd = useCallback(async () => {
    const path = await window.dialogApi.openFolder()
    if (path) {
      await appDispatch({ type: 'OpenProject', payload: { path } })
    }
  }, [appDispatch])

  if (projects.length === 0) return null

  return (
    <Box sx={{ width: '100%' }}>
      <Stack direction="row" alignItems="center">
        {/* Left: Project Tabs + Add Button */}
        <Box sx={{ flex: 1, overflow: 'hidden', display: 'flex', alignItems: 'center', borderBottom: 1, borderColor: 'outlineVariant.main' }}>
          <StyledTabs
            value={activeIndex}
            onChange={handleChange}
            variant="scrollable"
            scrollButtons="auto"
            aria-label="project tabs"
          >
            {projects.map((project, index) => (
              <StyledTab
                key={project.id || index}
                label={
                  <Stack direction="row" alignItems="center" spacing={1}>
                    <ProjectIcon fontSize="small" />
                    <span>{project.name}</span>
                    <IconButton
                      size="small"
                      component="span"
                      onClick={(e) => handleClose(e, index)}
                      sx={{
                        ml: 0.5,
                        p: 0.25,
                        opacity: 0.6,
                        '&:hover': { opacity: 1, bgcolor: 'action.selected' }
                      }}
                    >
                      <CloseIcon fontSize="inherit" sx={{ fontSize: 14 }} />
                    </IconButton>
                  </Stack>
                }
              />
            ))}
          </StyledTabs>
          <IconButton size="small" onClick={handleAdd} aria-label="open new project" sx={{ mx: 0.5 }}>
            <AddIcon />
          </IconButton>
        </Box>

        {/* Right: Global Icon Bar */}
        <GlobalIconBar />
      </Stack>
    </Box>
  )
}
