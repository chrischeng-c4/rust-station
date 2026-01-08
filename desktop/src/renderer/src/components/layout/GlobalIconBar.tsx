import { useCallback } from 'react'
import { IconButton, Stack, Tooltip } from '@mui/material'
import {
  Assignment as TasksIcon,
  PhotoCamera as SnapshotIcon,
  Download as ImportIcon,
  Notifications as NotificationsIcon,
  BarChart as MetricsIcon,
  Inventory as DockerIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material'

/**
 * GlobalIconBar - 7 icon buttons for global actions.
 * Positioned on the right side of the ProjectTabs.
 */
export function GlobalIconBar() {
  const handleSnapshot = useCallback(async () => {
    try {
      const result = await window.screenshotApi.capture()
      if (result.success) {
        console.log('Screenshot saved to:', result.filePath)
      } else {
        console.error('Screenshot failed:', result.error)
      }
    } catch (error) {
      console.error('Screenshot error:', error)
    }
  }, [])

  const icons = [
    { icon: <TasksIcon />, label: 'Tasks', onClick: () => console.log('Tasks clicked') },
    { icon: <SnapshotIcon />, label: 'Snapshot', onClick: handleSnapshot },
    { icon: <ImportIcon />, label: 'Import', onClick: () => console.log('Import clicked') },
    { icon: <NotificationsIcon />, label: 'Notifications', onClick: () => console.log('Notifications clicked') },
    { icon: <MetricsIcon />, label: 'Metrics', onClick: () => console.log('Metrics clicked') },
    { icon: <DockerIcon />, label: 'Docker', onClick: () => console.log('Docker clicked') },
    { icon: <SettingsIcon />, label: 'Settings', onClick: () => console.log('Settings clicked') },
  ]

  return (
    <Stack
      direction="row"
      spacing={0.5}
      sx={{
        alignItems: 'center',
        px: 1,
        borderBottom: 1,
        borderColor: 'outlineVariant.main',
        minHeight: 48,
      }}
    >
      {icons.map((item) => (
        <Tooltip key={item.label} title={item.label}>
          <IconButton
            size="small"
            onClick={item.onClick}
            aria-label={item.label.toLowerCase()}
            sx={{
              color: 'onSurfaceVariant.main',
              '&:hover': {
                bgcolor: 'action.hover',
              },
            }}
          >
            {item.icon}
          </IconButton>
        </Tooltip>
      ))}
    </Stack>
  )
}
