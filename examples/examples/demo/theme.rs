use std::sync::LazyLock;

use palette::color_difference::Wcag21RelativeContrast;
use tui_theme::palette::{Catppuccin, Everforest, Gruvbox, Kanagawa, RosePine};
use tui_theme::profile::TermProfile;
use tui_theme::{Color, Dark, Light, ProfileVariant, SetTheme, Style, Theme, is_supported};

#[derive(Theme, Default, Clone, Debug)]
pub struct AppTheme {
    pub root: Style,
    pub content: Style,
    pub app_title: Style,
    pub main_tabs: Style,
    pub main_tabs_selected: Style,
    pub borders: Style,
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
    pub item: Style,
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

#[derive(Theme, Default, Clone, Debug)]
pub struct Colors {
    base1: Color,
    base2: Color,
    text: Color,
    text_muted: Color,
    text_bright: Color,
    primary: Color,
    accent: Color,
    success: Color,
    warning: Color,
    danger: Color,
    selected: Color,
}

const BASIC_ANSI_THEME: Colors = Colors {
    base1: Color::Reset,
    base2: Color::Reset,
    primary: Color::Blue,
    accent: Color::Cyan,
    success: Color::Green,
    warning: Color::Yellow,
    danger: Color::Red,
    text: Color::Reset,
    text_muted: Color::Gray,
    text_bright: Color::White,
    selected: Color::Yellow,
};

static THEMES: LazyLock<[Colors; 6]> = LazyLock::new(|| {
    let bg = Color::terminal_background();
    let fg = Color::terminal_foreground();
    let rel_luma = bg.to_rgb_bg().into_linear::<f32>().relative_luminance();
    let factor = 0.15;
    let base2 = if rel_luma.luma <= 0.01 {
        bg.lighten(factor)
    } else {
        bg.darken(factor)
    };

    [
        Colors {
            base1: bg,
            base2,
            primary: Color::Blue,
            accent: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            text: Color::terminal_foreground(),
            text_muted: fg.darken(0.15),
            text_bright: fg.lighten(0.15),
            selected: Color::Yellow,
        },
        Colors {
            base1: Catppuccin::GRAY[(Light(1), Dark(9))],
            base2: Catppuccin::GRAY[(Light(0), Dark(10))],
            primary: Catppuccin::BLUE[(Light(7), Dark(3))],
            accent: Catppuccin::BLUE[(Light(7), Dark(3))],
            success: Catppuccin::GREEN[(Light(7), Dark(3))],
            warning: Catppuccin::YELLOW[(Light(7), Dark(3))],
            danger: Catppuccin::ROSEWATER[(Light(7), Dark(3))],
            text: Catppuccin::GRAY[(Light(7), Dark(3))],
            text_muted: Catppuccin::GRAY[(Light(6), Dark(5))],
            text_bright: Catppuccin::GRAY[(Light(9), Dark(1))],
            selected: Catppuccin::YELLOW[(Light(7), Dark(3))],
        },
        Colors {
            base1: Everforest::BLUE_GRAY[(Light(1), Dark(9))],
            base2: Everforest::BLUE_GRAY[(Light(0), Dark(10))],
            primary: Everforest::BLUE_GRAY[(Light(7), Dark(3))],
            accent: Everforest::BLUE[(Light(7), Dark(3))],
            success: Everforest::GREEN[(Light(7), Dark(3))],
            warning: Everforest::YELLOW[(Light(7), Dark(3))],
            danger: Everforest::RED[(Light(7), Dark(3))],
            text: Everforest::BLUE[(Light(7), Dark(3))],
            text_muted: Everforest::BLUE_GRAY[(Light(6), Dark(5))],
            text_bright: Everforest::BLUE_GRAY[(Light(9), Dark(1))],
            selected: Everforest::YELLOW[(Light(7), Dark(3))],
        },
        Colors {
            base1: RosePine::GRAY[(Light(1), Dark(9))],
            base2: RosePine::GRAY[(Light(0), Dark(10))],
            primary: RosePine::PINE[(Light(7), Dark(3))],
            accent: RosePine::ROSE[(Light(7), Dark(3))],
            success: RosePine::FOAM[(Light(7), Dark(3))],
            warning: RosePine::GOLD[(Light(7), Dark(3))],
            danger: RosePine::ROSE[(Light(7), Dark(3))],
            text: RosePine::GRAY[(Light(7), Dark(3))],
            text_muted: RosePine::GRAY[(Light(6), Dark(5))],
            text_bright: RosePine::GRAY[(Light(9), Dark(1))],
            selected: RosePine::GOLD[(Light(7), Dark(3))],
        },
        Colors {
            base1: Gruvbox::BROWN[(Light(1), Dark(9))],
            base2: Gruvbox::BROWN[(Light(0), Dark(10))],
            primary: Gruvbox::GREEN[(Light(7), Dark(3))],
            accent: Gruvbox::ORANGE[(Light(7), Dark(3))],
            success: Gruvbox::AQUA[(Light(7), Dark(3))],
            warning: Gruvbox::YELLOW[(Light(7), Dark(3))],
            danger: Gruvbox::RED[(Light(7), Dark(3))],
            text: Gruvbox::GRAY[(Light(7), Dark(3))],
            text_muted: Gruvbox::GRAY[(Light(6), Dark(5))],
            text_bright: Gruvbox::GRAY[(Light(9), Dark(1))],
            selected: Gruvbox::YELLOW[(Light(7), Dark(3))],
        },
        Colors {
            base1: Kanagawa::HONEYDEW[(Light(1), Dark(9))],
            base2: Kanagawa::HONEYDEW[(Light(0), Dark(10))],
            primary: Kanagawa::INDIGO[(Light(7), Dark(3))],
            accent: Kanagawa::AQUA[(Light(7), Dark(3))],
            success: Kanagawa::GREEN[(Light(7), Dark(3))],
            warning: Kanagawa::PEACH[(Light(7), Dark(3))],
            danger: Kanagawa::RED[(Light(7), Dark(3))],
            text: Kanagawa::GRAY[(Light(7), Dark(3))],
            text_muted: Kanagawa::GRAY[(Light(6), Dark(5))],
            text_bright: Kanagawa::GRAY[(Light(9), Dark(1))],
            selected: Kanagawa::EARTH_YELLOW[(Light(7), Dark(3))],
        },
    ]
});

pub fn num_themes() -> usize {
    THEMES.len()
}

pub fn enhanced_color_support() -> bool {
    matches!(is_supported(TermProfile::Ansi256), Ok(true))
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
        borders: Style::new().fg_text_muted(),
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
            item: Style::new().fg_text(),
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
