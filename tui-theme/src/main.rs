use ratatui::style::Stylize;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use tui_theme::SubTheme;
use tui_theme::{Color, ColorTheme, SetTheme, Theme};

#[derive(Clone, Default, Debug)]
struct Borders(String);

#[derive(ColorTheme, SetTheme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppColorTheme {
    #[variants("a", "b")]
    primary: Color,
    secondary: Color,
}

#[derive(SubTheme, SetTheme, Default, Clone, Debug)]
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
    let col: Color = "chartreuse".parse().unwrap();
    let col2 = col.into_adaptive();
    println!("{col:?}");
    println!("{col2:?}");
    BorderTheme::primary();

    let theme = AppTheme {
        color: AppColorTheme {
            primary: Color::AnsiRed,
            secondary: Color::AnsiBlue,
        },
        borders: BorderTheme {
            primary: Borders("a".to_string()),
            secondary: Borders("b".to_string()),
        },
    };
    theme.set_global();
    println!("{:?}", AppTheme::current())
}
