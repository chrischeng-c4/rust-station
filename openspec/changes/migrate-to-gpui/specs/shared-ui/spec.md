## MODIFIED Requirements
### Requirement: Global Theme Density
The system SHALL use a compact Material Design 3 theme configuration to maximize information density via GPUI styling.

#### Scenario: Component Default Size
- **WHEN** UI components are rendered
- **THEN** they SHALL use compact sizing (equivalent to 'small') for: Button, IconButton, Input, Select, Chip, Table

#### Scenario: Dense Lists and Menus
- **WHEN** lists or menus are rendered
- **THEN** they SHALL use dense vertical padding

#### Scenario: Compact Toolbar
- **WHEN** toolbars are rendered
- **THEN** they SHALL use dense height variant
