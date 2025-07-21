use tui_theme_derive::StyleTheme;

use crate::{Color, Theme};

#[derive(StyleTheme, Theme, Default, Clone, Debug)]
pub struct AppTheme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub foreground: Color,
    pub background: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

pub const ANSI: AppTheme = AppTheme {
    primary: Color::AnsiCyan,
    secondary: Color::AnsiBlue,
    accent: Color::AnsiMagenta,
    foreground: Color::AnsiReset,
    background: Color::AnsiReset,
    success: Color::AnsiGreen,
    warning: Color::AnsiYellow,
    error: Color::AnsiRed,
};
