## ADDED Requirements
### Requirement: Global Theme Density
The system SHALL use a compact Material Design 3 theme configuration to maximize information density via MUI's `defaultProps` pattern.

#### Scenario: Component Default Size
- **WHEN** UI components are rendered
- **THEN** they SHALL use `size: 'small'` as default for: Button, IconButton, TextField, Select, Chip, Table

#### Scenario: Dense Lists and Menus
- **WHEN** lists or menus are rendered
- **THEN** they SHALL use `dense: true` as default for: List, MenuItem

#### Scenario: Compact Toolbar
- **WHEN** toolbars are rendered
- **THEN** they SHALL use `variant: 'dense'` as default

#### Scenario: Reduced Border Radius
- **WHEN** components with borders are rendered
- **THEN** they SHALL use reduced border radius (global: 8px, buttons: 16px, cards: 8px)
