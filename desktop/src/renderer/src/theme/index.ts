import { createTheme, alpha } from '@mui/material/styles'

// Extend MUI palette with M3 Surface Container roles
declare module '@mui/material/styles' {
  interface Palette {
    surfaceContainerLowest: Palette['primary']
    surfaceContainerLow: Palette['primary']
    surfaceContainer: Palette['primary']
    surfaceContainerHigh: Palette['primary']
    surfaceContainerHighest: Palette['primary']
    onSurface: Palette['primary']
    onSurfaceVariant: Palette['primary']
    outline: Palette['primary']
    outlineVariant: Palette['primary']
    secondaryContainer: Palette['primary']
    onSecondaryContainer: Palette['primary']
  }
  interface PaletteOptions {
    surfaceContainerLowest?: PaletteOptions['primary']
    surfaceContainerLow?: PaletteOptions['primary']
    surfaceContainer?: PaletteOptions['primary']
    surfaceContainerHigh?: PaletteOptions['primary']
    surfaceContainerHighest?: PaletteOptions['primary']
    onSurface?: PaletteOptions['primary']
    onSurfaceVariant?: PaletteOptions['primary']
    outline?: PaletteOptions['primary']
    outlineVariant?: PaletteOptions['primary']
    secondaryContainer?: PaletteOptions['primary']
    onSecondaryContainer?: PaletteOptions['primary']
  }
}

// Compact theme with M3 palette + defaultProps only (no styleOverrides)
export const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#D0BCFF',
      light: '#E8DEF8',
      dark: '#381E72',
      contrastText: '#381E72',
    },
    secondary: {
      main: '#CCC2DC',
      light: '#E8DEF8',
      dark: '#332D41',
      contrastText: '#332D41',
    },
    background: {
      default: '#1C1B1F',
      paper: '#2B2930',
    },
    // M3 Surface Container Roles
    surfaceContainerLowest: { main: '#0F0D13' },
    surfaceContainerLow: { main: '#1D1B20' },
    surfaceContainer: { main: '#211F26' },
    surfaceContainerHigh: { main: '#2B2930' },
    surfaceContainerHighest: { main: '#36343B' },
    onSurface: { main: '#E6E1E5' },
    onSurfaceVariant: { main: '#CAC4D0' },
    outline: { main: '#938F99' },
    outlineVariant: { main: '#49454F' },
    secondaryContainer: { main: '#4A4458' },
    onSecondaryContainer: { main: '#E8DEF8' },
    error: { main: '#F2B8B5' },
    text: {
      primary: '#E6E1E5',
      secondary: '#CAC4D0',
    },
    divider: alpha('#CAC4D0', 0.12),
  },
  components: {
    // Compact sizing via defaultProps only
    MuiButton: {
      defaultProps: { size: 'small' },
    },
    MuiIconButton: {
      defaultProps: { size: 'small' },
    },
    MuiTextField: {
      defaultProps: { size: 'small' },
    },
    MuiSelect: {
      defaultProps: { size: 'small' },
    },
    MuiChip: {
      defaultProps: { size: 'small' },
    },
    MuiTable: {
      defaultProps: { size: 'small' },
    },
    MuiToolbar: {
      defaultProps: { variant: 'dense' },
    },
    MuiList: {
      defaultProps: { dense: true },
    },
    MuiMenuItem: {
      defaultProps: { dense: true },
    },
  },
})
