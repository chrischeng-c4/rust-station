import { useState } from 'react'
import { A2UIRenderer } from './components/A2UIRenderer'
import { PageHeader } from '@/components/shared/PageHeader'
import type { A2UINode, A2UIAction } from './types'
import {
  Box,
  Typography,
  Paper,
  Stack,
  Card
} from '@mui/material'
import { Code as CodeIcon, BugReport as DebugIcon } from '@mui/icons-material'
import { useAppState } from '@/hooks/useAppState'

export function A2UIPage() {
  const { state: appState } = useAppState()
  const payload = appState?.a2ui?.payload as A2UINode | null
  const [lastAction, setLastAction] = useState<A2UIAction | null>(null)

  const handleAction = (action: A2UIAction) => {
    console.log('A2UI Action:', action)
    setLastAction(action)
  }

  return (
    <Box sx={{ display: 'flex', height: '100%', flexDirection: 'column', p: 3 }}>
      <PageHeader 
        title="A2UI Renderer" 
        description="Dynamic UI generation from JSON (Agent-to-UI Protocol)"
        icon={<CodeIcon />}
      />

      <Stack direction="row" spacing={3} sx={{ flex: 1, minHeight: 0 }}>
        {/* Left: The Rendered UI */}
        <Paper 
          variant="outlined" 
          sx={{ 
            flex: 1, 
            p: 4, 
            bgcolor: 'surfaceContainerLow.main', 
            borderRadius: 4,
            overflow: 'auto',
            display: 'flex',
            flexDirection: 'column'
          }}
        >
          {payload ? (
            <A2UIRenderer node={payload} onAction={handleAction} />
          ) : (
            <Box sx={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', color: 'text.secondary', opacity: 0.5 }}>
              <Box sx={{ textAlign: 'center' }}>
                <CodeIcon sx={{ fontSize: 64, mb: 2 }} />
                <Typography variant="h6">No UI Payload</Typography>
                <Typography variant="body2">Use the 'render_ui' MCP tool to push content here.</Typography>
              </Box>
            </Box>
          )}
        </Paper>

        {/* Right: Debug Info */}
        <Stack spacing={3} sx={{ width: 360, flexShrink: 0 }}>
          <Card variant="outlined" sx={{ borderRadius: 4 }}>
            <Box sx={{ p: 2, borderBottom: 1, borderColor: 'outlineVariant' }}>
              <Stack direction="row" spacing={1} alignItems="center">
                <DebugIcon fontSize="small" color="primary" />
                <Typography variant="subtitle2" fontWeight={600}>Last Action</Typography>
              </Stack>
            </Box>
            <Box sx={{ p: 2 }}>
              <Box 
                component="pre" 
                sx={{ 
                  m: 0, 
                  p: 1.5, 
                  bgcolor: 'background.default', 
                  borderRadius: 1, 
                  fontSize: '0.7rem', 
                  fontFamily: 'monospace',
                  overflowX: 'auto',
                  border: 1,
                  borderColor: 'outlineVariant'
                }}
              >
                {lastAction ? JSON.stringify(lastAction, null, 2) : 'None'}
              </Box>
            </Box>
          </Card>

          <Card variant="outlined" sx={{ flex: 1, borderRadius: 4, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
            <Box sx={{ p: 2, borderBottom: 1, borderColor: 'outlineVariant' }}>
              <Typography variant="subtitle2" fontWeight={600}>Source JSON</Typography>
            </Box>
            <Box sx={{ flex: 1, p: 2, overflow: 'auto' }}>
              <Box 
                component="pre" 
                sx={{ 
                  m: 0, 
                  p: 1.5, 
                  bgcolor: 'background.default', 
                  borderRadius: 1, 
                  fontSize: '0.7rem', 
                  fontFamily: 'monospace',
                  color: 'onSurfaceVariant.main',
                  border: 1,
                  borderColor: 'outlineVariant'
                }}
              >
                {payload ? JSON.stringify(payload, null, 2) : '{}'}
              </Box>
            </Box>
          </Card>
        </Stack>
      </Stack>
    </Box>
  )
}
