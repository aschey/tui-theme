use palette::rgb::Srgb;
use ratatui::style::Stylize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use tui_theme::SubTheme;
use tui_theme::{Color, ColorTheme, SetTheme, Theme};

#[derive(Clone, Default, Debug)]
struct Borders(String);

#[derive(ColorTheme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppColorTheme {
    #[variants("a", "b")]
    primary: Color,
    secondary: Color,
}

#[derive(SubTheme, Default, Clone, Debug)]
struct BorderTheme {
    primary: Borders,
    secondary: Borders,
}

#[derive(Theme, Default, Clone, Debug)]
struct AppTheme {
    color: AppColorTheme,
    borders: BorderTheme,
}

fn main() {
    let col: Color = "lab(50.0 100% 40%)".parse().unwrap();
    println!("{col:?}");
    let theme = AppTheme {
        color: AppColorTheme {
            primary: Color::Red,
            secondary: Color::Blue,
        },
        borders: BorderTheme {
            primary: Borders("a".to_string()),
            secondary: Borders("b".to_string()),
        },
    };
    theme.set_global();
    println!("{:?}", AppTheme::current())
}
