use crate::Color;
use crate::{ColorTheme, SetTheme};
use ratatui::style::Stylize;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;

#[derive(ColorTheme, SetTheme, Default, Clone, Debug)]
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
