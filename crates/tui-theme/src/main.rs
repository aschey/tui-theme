use std::cell::RefCell;
use std::io::stdout;
use std::sync::{Arc, LazyLock, RwLock};

use anstyle_crossterm::to_crossterm;
use termprofile::ProfileColor;
use tui_theme::{
    Color, SetTheme, Style, StyleTheme, SubTheme, Theme, load_color_palette, load_profile, palette,
    profile,
};

#[derive(Clone, Default, Debug)]
struct Borders(String);

#[derive(StyleTheme, SetTheme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppColorTheme {
    #[variants("a", "b")]
    primary: Color,
    secondary: Color,
}

#[derive(StyleTheme, SetTheme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppStyleTheme {
    #[variants("a", "b")]
    primary: Style,
    secondary: Style,
}

#[derive(StyleTheme, SetTheme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppColorTheme2 {
    #[variants("a", "b")]
    primary2: Color,
    secondary2: Color,
}

#[derive(SubTheme, SetTheme, Default, Clone, Debug)]
struct BorderTheme {
    primary: Borders,
    secondary: Borders,
}

#[derive(Theme, Default, Clone, Debug)]
struct AppTheme {
    #[subtheme]
    color: AppColorTheme,
    #[subtheme]
    borders: BorderTheme,
    #[subtheme]
    color2: AppColorTheme2,
}

fn main() {
    let _ = load_profile(&stdout());
    let _ = load_color_palette();
    let a: Option<Color> = ProfileColor::new(Color::AnsiReset)
        .try_adapt(&profile().unwrap())
        .map(Into::into);
    let fg = Color::terminal_background();
    println!("{fg:?}");
    println!("{:?}", palette::RosePine::ROSE_500.into_adaptive());
    let fg2: Option<anstyle::Color> = palette::RosePine::ROSE_500.into_adaptive().into();
    let s = to_crossterm(anstyle::Style::new().fg_color(fg2));
    println!("{}", s.apply("yoo"));

    let col: Color = "chartreuse".parse().unwrap();
    let col2 = col.into_adaptive();
    println!("{col:?}");
    println!("{col2:?}");
    BorderTheme::primary();
    "a".style_primary();

    let theme = AppTheme {
        color: AppColorTheme {
            primary: Color::AnsiRed,
            secondary: Color::AnsiBlue,
        },
        color2: AppColorTheme2 {
            primary2: Color::AnsiRed,
            secondary2: Color::AnsiBlue,
        },

        borders: BorderTheme {
            primary: Borders("a".to_string()),
            secondary: Borders("b".to_string()),
        },
    };
    theme.set_global();
    "a".fg_primary2();
    println!("{:?}", AppTheme::current());

    let theme2 = AppStyleTheme {
        primary: Style::default().fg(palette::RosePine::ROSE_500),
        secondary: Style::default().fg(palette::RosePine::ROSE_100),
    };
    println!("{theme2:?}");
}
