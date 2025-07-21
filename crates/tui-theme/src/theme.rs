use tui_theme_derive::StyleTheme;

use crate::{Color, SetTheme};

#[derive(StyleTheme, SetTheme, Default, Clone, Debug)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub foreground: Color,
    pub background: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

pub const ANSI: Theme = Theme {
    primary: Color::AnsiCyan,
    secondary: Color::AnsiBlue,
    accent: Color::AnsiMagenta,
    foreground: Color::AnsiReset,
    background: Color::AnsiReset,
    success: Color::AnsiGreen,
    warning: Color::AnsiYellow,
    error: Color::AnsiRed,
};
