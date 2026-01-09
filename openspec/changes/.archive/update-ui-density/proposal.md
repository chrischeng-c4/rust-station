# Change: Update UI Density

## Why
The current UI uses large default spacing and sizing (Material Design 3 defaults), which reduces information density. For a developer tool, seeing more information on screen (files, logs, tasks) is critical for productivity.

## What Changes
- Reduce global theme `borderRadius` from 16px to 8px
- Reduce global font sizes (body1 from 0.875rem to 0.8125rem)
- Reduce button padding and border radius
- Reduce card border radius
- Update `MuiToolbar` min-height to be more compact
- Reduce `MuiTableCell` padding for denser tables
- **BREAKING**: Visual changes will affect all screens.

## Impact
- Affected specs: `shared-ui`
- Affected code: `desktop/src/renderer/src/theme/index.ts`
