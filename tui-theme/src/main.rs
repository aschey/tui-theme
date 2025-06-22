use anstyle_crossterm::to_crossterm;
use ratatui::style::Stylize;
use std::cell::RefCell;
use std::io::stdout;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;
use tui_theme::SubTheme;
use tui_theme::load_color_palette;
use tui_theme::load_profile;
use tui_theme::palette;
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
    load_profile(&stdout());
    load_color_palette();
    let fg = Color::terminal_background();
    println!("{fg:?}");
    let fg2: Option<anstyle::Color> = palette::RosePine::ROSE_500.into();
    let s = to_crossterm(anstyle::Style::new().fg_color(fg2));
    println!("{}", s.apply("yoo"));

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
