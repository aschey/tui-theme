use std::io::stdout;

use termprofile::{DetectorSettings, ProfileColor};
use tui_theme::{
    Color, SetTheme, Style, Styled, Stylize, Theme, load_color_palette, load_profile, palette,
    term_profile,
};

#[derive(Clone, Default, Debug)]
struct Borders(String);

#[derive(Theme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppColorTheme {
    #[variants("a", "b")]
    primary: Color,
    secondary: Color,
}

#[derive(Theme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppStyleTheme {
    #[variants("a", "b")]
    primary: Style,
    secondary: Style,
}

#[derive(Theme, Default, Clone, Debug)]
#[variants("a", "b")]
struct AppColorTheme2 {
    #[variants("a", "b")]
    primary2: Color,
    secondary2: Color,
}

#[derive(Theme, Default, Clone, Debug)]
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
    load_profile(&stdout(), DetectorSettings::new());
    load_color_palette();

    ratatui::style::Style::new().fg_primary2();
    Color::primary2();
    Style::primary();
    ratatui::style::Color::primary2();
    let a: Option<Color> = ProfileColor::new(
        anstyle::Color::Rgb(anstyle::RgbColor(0, 0, 0)),
        term_profile().unwrap(),
    )
    .adapt()
    .map(Into::into);
    let fg = Color::terminal_background();
    println!("{fg:?}");
    println!("{:?}", palette::RosePine::ROSE_500.into_adaptive());
    let fg2: Option<anstyle::Color> = palette::RosePine::ROSE_500.into_adaptive().into();
    //let s = to_crossterm(anstyle::Style::new().fg_color(fg2));
    //println!("{}", s.apply("yoo"));

    let col: Color = "chartreuse".parse().unwrap();
    let col2 = col.into_adaptive();
    println!("{col:?}");
    println!("{col2:?}");
    BorderTheme::primary();
    "a".style_primary();

    let theme = AppTheme {
        color: AppColorTheme {
            primary: Color::Red,
            secondary: Color::Blue,
        },
        color2: AppColorTheme2 {
            primary2: Color::Red,
            secondary2: Color::Blue,
        },

        borders: BorderTheme {
            primary: Borders("a".to_string()),
            secondary: Borders("b".to_string()),
        },
    };
    theme.set_global();
    "a".fg_primary2();
    "a".fg(Color::Red);
    "a".fg(ratatui::style::Color::Red);
    "a".set_style(Style::new());
    "a".set_style(ratatui::style::Style::new());
    println!("{:?}", AppTheme::current());

    let theme2 = AppStyleTheme {
        primary: Style::default().fg(palette::RosePine::ROSE_500),
        secondary: Style::default().fg(palette::RosePine::ROSE_100),
    };
    println!("{theme2:?}");
}
