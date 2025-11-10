use tui_theme::profile::TermProfile;
use tui_theme::{Color, ProfileVariant, SetTheme, Style, Theme, is_supported};

use crate::colors::{BASIC_ANSI_THEME, THEMES, ThemeColorsColorTheme, ThemeColorsColorThemeExt};

#[derive(Theme, Default, Clone, Debug)]
pub struct AppTheme {
    pub root: Style,
    pub content: Style,
    pub app_title: Style,
    pub main_tabs: Style,
    pub main_tabs_selected: Style,
    pub description: Style,
    pub description_title: Style,
    #[subtheme]
    pub key_binding: KeyBinding,
    #[subtheme]
    pub logo: Logo,
    #[subtheme]
    pub email: Email,
    #[subtheme]
    pub traceroute: Traceroute,
    #[subtheme]
    pub recipe: Recipe,
    #[subtheme]
    pub weather: Weather,
}

#[derive(Theme, Default, Clone, Debug)]
pub struct KeyBinding {
    pub key: Style,
    pub key_description: Style,
}

#[derive(Theme, Default, Clone, Debug)]
pub struct Logo {
    pub rat: Color,
    pub rat_eye: Color,
    pub rat_eye_alt: Color,
    pub term: Color,
    pub term_border: Color,
}

#[derive(Theme, Default, Clone, Debug)]
pub struct Email {
    pub tabs: Style,
    pub tabs_selected: Style,
    pub inbox: Style,
    pub selected_item: Style,
    pub header: Style,
    pub header_value: Style,
    pub body: Style,
}

#[derive(Theme, Default, Clone, Debug)]
pub struct Traceroute {
    pub header: Style,
    pub title: Style,
    pub selected: Style,
    pub ping: Style,
    pub map_main: Style,
    pub map_color: Color,
    pub map_path: Color,
    pub map_source: Color,
    pub map_destination: Color,
    pub map_background_color: Color,
}

#[derive(Theme, Default, Clone, Debug)]
pub struct Recipe {
    pub ingredients: Style,
    pub ingredients_header: Style,
    pub selected: Style,
    pub step_title: Style,
    pub step_content: Style,
}

#[derive(Theme, Default, Clone, Debug)]
pub struct Weather {
    pub bar1: Style,
    pub bar2: Style,
    pub bar_value1: Style,
    pub bar_value2: Style,
    pub calendar_day: Style,
    pub progress: Color,
    pub progress_value: Color,
    pub line_gauge: Color,
}

pub fn enhanced_color_support() -> bool {
    is_supported(TermProfile::Ansi256)
}

pub fn init_theme(index: usize) {
    let colors = if enhanced_color_support() {
        &THEMES[index]
    } else {
        &BASIC_ANSI_THEME
    };
    colors.set_global();
    let base1 = Style::new().fg_text().bg_base1();
    let muted = Style::new().fg_text_muted().bg_base2();
    let theme = AppTheme {
        root: base1,
        content: base1,
        app_title: Style::new().fg_text_bright().bg_base2().bold(),
        main_tabs: muted,
        main_tabs_selected: Style::new().fg_text_bright().bg_base1().bold().reversed(),
        description: base1,
        description_title: Style::new().fg_text().bold(),
        logo: Logo {
            rat: Color::text_bright(),
            rat_eye: Color::base2(),
            rat_eye_alt: Color::danger(),
            term: Color::base2().darken(0.1),
            term_border: Color::accent(),
        },
        key_binding: KeyBinding {
            key: ProfileVariant::new(muted).ansi_16(base1).into(),
            key_description: ProfileVariant::new(Style::new().fg_base2().bg_text_muted())
                .ansi_16(muted)
                .into(),
        },
        email: Email {
            tabs: base1,
            tabs_selected: Style::new().fg_text_bright().bg_base1().bold(),
            inbox: base1,
            selected_item: Style::new().fg_selected(),
            header: Style::new().bold(),
            header_value: Style::new().fg_text(),
            body: base1,
        },
        traceroute: Traceroute {
            header: Style::new().bg_base1().bold().underlined(),
            title: Style::new().fg_text_bright().bold(),
            selected: Style::new().fg_selected(),
            ping: Style::new().fg_text_bright(),
            map_main: Style::new().bg_base1(),
            map_background_color: Color::base1(),
            map_color: Color::text_muted(),
            map_path: Color::primary(),
            map_source: Color::success(),
            map_destination: Color::danger(),
        },
        recipe: Recipe {
            ingredients: base1,
            ingredients_header: Style::new().bold().underlined(),
            selected: Style::new().fg_selected(),
            step_title: Style::new().fg_text_bright().bold(),
            step_content: Style::new().fg_text_muted(),
        },
        weather: Weather {
            bar1: Style::new().fg_danger(),
            bar2: Style::new().fg_warning(),
            bar_value1: Style::new()
                .fg_base2()
                .bg(ProfileVariant::new(Color::danger()).ansi_16(Color::Reset))
                .bold(),
            bar_value2: Style::new()
                .fg_base2()
                .bg(ProfileVariant::new(Color::warning()).ansi_16(Color::Reset))
                .bold(),
            calendar_day: Style::new().fg_danger().bold(),
            progress: Color::warning(),
            progress_value: Color::accent(),
            line_gauge: Color::primary(),
        },
    };

    theme.set_global();
}
