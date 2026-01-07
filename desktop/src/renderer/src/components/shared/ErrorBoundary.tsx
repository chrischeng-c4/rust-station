import React, { Component, ErrorInfo, ReactNode } from 'react'
import { Box, Typography, Button } from '@mui/material'

interface Props {
  children: ReactNode
}

interface State {
  hasError: boolean
  error: Error | null
  errorInfo: ErrorInfo | null
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
    errorInfo: null
  }

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, errorInfo: null }
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error:', error, errorInfo)
    this.setState({ errorInfo })
  }

  private handleReload = () => {
    window.location.reload()
  }

  public render() {
    if (this.state.hasError) {
      return (
        <Box sx={{ p: 4, height: '100vh', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: 2 }}>
          <Typography variant="h4" color="error">Something went wrong</Typography>
          <Typography variant="body1">{this.state.error?.toString()}</Typography>
          <pre style={{ maxWidth: '100%', overflow: 'auto', fontSize: '0.75rem', padding: 8, background: '#eee' }}>
            {this.state.errorInfo?.componentStack}
          </pre>
          <Button variant="contained" onClick={this.handleReload}>Reload App</Button>
        </Box>
      )
    }

    return this.props.children
  }
}
