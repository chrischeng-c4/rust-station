//! Theme system for rstn UI
//!
//! Implements Material Design 3 color palette and design tokens for GPUI.
//! Based on the old MUI theme configuration from desktop/src/renderer/src/theme/index.ts

use gpui::*;
use serde::{Deserialize, Serialize};

/// Material Design 3 color tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTheme {
    /// Primary color palette
    pub primary: PrimaryColors,
    /// Secondary color palette
    pub secondary: SecondaryColors,
    /// Background colors
    pub background: BackgroundColors,
    /// Surface colors
    pub surface: SurfaceColors,
    /// Border and divider colors
    pub border: BorderColors,
    /// Text colors
    pub text: TextColors,
    /// Shape configuration
    pub shape: ShapeConfig,
    /// Spacing scale
    pub spacing: SpacingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryColors {
    /// Primary main color (M3 Purple 80: #D0BCFF)
    pub main: Rgba,
    /// Primary container background
    pub container: Rgba,
    /// Text/icons on primary background
    pub on_primary: Rgba,
    /// Text/icons on primary container
    pub on_primary_container: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryColors {
    /// Secondary main color (M3 Purple 80 variant: #CCC2DC)
    pub main: Rgba,
    /// Secondary container background
    pub container: Rgba,
    /// Text/icons on secondary background
    pub on_secondary: Rgba,
    /// Text/icons on secondary container
    pub on_secondary_container: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColors {
    /// Default background (M3 Surface: #1C1B1F)
    pub default: Rgba,
    /// Paper/card background (M3 Surface Container: #2B2930)
    pub paper: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceColors {
    /// Base surface color
    pub base: Rgba,
    /// Surface container (elevated)
    pub container: Rgba,
    /// Surface container (highest elevation)
    pub container_high: Rgba,
    /// Surface variant
    pub variant: Rgba,
    /// Text/icons on surface
    pub on_surface: Rgba,
    /// Text/icons on surface variant
    pub on_surface_variant: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderColors {
    /// Divider lines (#3D3D3D)
    pub divider: Rgba,
    /// Subtle borders
    pub subtle: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    /// Primary text color (white)
    pub primary: Rgba,
    /// Secondary text color (dimmed)
    pub secondary: Rgba,
    /// Disabled text color
    pub disabled: Rgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeConfig {
    /// Standard border radius (16px - large rounded corners)
    pub border_radius: Pixels,
    /// Small border radius (8px)
    pub border_radius_sm: Pixels,
    /// Extra small border radius (4px)
    pub border_radius_xs: Pixels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingConfig {
    /// Base spacing unit (8px)
    pub base: Pixels,
}

impl MaterialTheme {
    /// Create the default dark theme (Material Design 3)
    pub fn dark() -> Self {
        Self {
            primary: PrimaryColors {
                main: rgb(0xD0BCFF),
                container: rgb(0x4F378B),
                on_primary: rgb(0x381E72),
                on_primary_container: rgb(0xEADDFF),
            },
            secondary: SecondaryColors {
                main: rgb(0xCCC2DC),
                container: rgb(0x4A4458),
                on_secondary: rgb(0x332D41),
                on_secondary_container: rgb(0xE8DEF8),
            },
            background: BackgroundColors {
                default: rgb(0x1C1B1F),
                paper: rgb(0x2B2930),
            },
            surface: SurfaceColors {
                base: rgb(0x1C1B1F),
                container: rgb(0x2B2930),
                container_high: rgb(0x36343B),
                variant: rgb(0x49454F),
                on_surface: rgb(0xE6E1E5),
                on_surface_variant: rgb(0xCAC4D0),
            },
            border: BorderColors {
                divider: rgb(0x3D3D3D),
                subtle: rgb(0x49454F),
            },
            text: TextColors {
                primary: rgb(0xFFFFFF),
                secondary: rgb(0xAAAAAA),
                disabled: rgb(0x666666),
            },
            shape: ShapeConfig {
                border_radius: px(16.0),
                border_radius_sm: px(8.0),
                border_radius_xs: px(4.0),
            },
            spacing: SpacingConfig { base: px(8.0) },
        }
    }

    /// Get spacing value by multiplier (e.g., spacing(2) = 16px)
    pub fn spacing(&self, multiplier: f32) -> Pixels {
        self.spacing.base * multiplier
    }
}

/// Extension trait for applying theme to GPUI elements
pub trait Themed {
    /// Apply primary button styling
    fn primary_button(self, theme: &MaterialTheme) -> Self;

    /// Apply secondary button styling
    fn secondary_button(self, theme: &MaterialTheme) -> Self;

    /// Apply card/paper styling
    fn card(self, theme: &MaterialTheme) -> Self;

    /// Apply pill-shaped container styling (for sidebar items)
    fn pill(self, theme: &MaterialTheme, selected: bool) -> Self;
}

impl Themed for Div {
    fn primary_button(self, theme: &MaterialTheme) -> Self {
        self.px(theme.spacing(2.0))
            .py(theme.spacing(1.0))
            .bg(theme.primary.main)
            .text_color(theme.primary.on_primary)
            .rounded(theme.shape.border_radius_sm)
            .cursor_pointer()
            .hover(|style| style.bg(theme.primary.container))
    }

    fn secondary_button(self, theme: &MaterialTheme) -> Self {
        self.px(theme.spacing(2.0))
            .py(theme.spacing(1.0))
            .border_1()
            .border_color(theme.border.subtle)
            .text_color(theme.text.primary)
            .rounded(theme.shape.border_radius_sm)
            .cursor_pointer()
            .hover(|style| style.bg(theme.surface.container))
    }

    fn card(self, theme: &MaterialTheme) -> Self {
        self.bg(theme.background.paper)
            .border_1()
            .border_color(theme.border.divider)
            .rounded(theme.shape.border_radius)
            .p(theme.spacing(2.0))
    }

    fn pill(self, theme: &MaterialTheme, selected: bool) -> Self {
        let base = self
            .px(theme.spacing(1.5))
            .py(theme.spacing(1.0))
            .rounded(theme.shape.border_radius)
            .cursor_pointer();

        if selected {
            base.bg(theme.secondary.container)
                .text_color(theme.secondary.on_secondary_container)
        } else {
            base.text_color(theme.surface.on_surface_variant)
                .hover(|style| style.bg(theme.surface.container))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_creation() {
        let theme = MaterialTheme::dark();

        // Verify primary colors
        assert_eq!(theme.primary.main, rgb(0xD0BCFF));

        // Verify background colors
        assert_eq!(theme.background.default, rgb(0x1C1B1F));
        assert_eq!(theme.background.paper, rgb(0x2B2930));

        // Verify shape config
        assert_eq!(theme.shape.border_radius, px(16.0));
    }

    #[test]
    fn test_spacing_multiplier() {
        let theme = MaterialTheme::dark();

        assert_eq!(theme.spacing(1.0), px(8.0));
        assert_eq!(theme.spacing(2.0), px(16.0));
        assert_eq!(theme.spacing(0.5), px(4.0));
    }
}
