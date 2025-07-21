use tui_theme::palette::{Catppuccin, RosePine};
use tui_theme::{Color, Modifiers, SetTheme, Style, StyleTheme, SubTheme, Theme};

#[derive(Theme, StyleTheme, Default, Clone, Debug)]
pub struct AppTheme {
    pub root: Style,
    pub content: Style,
    pub app_title: Style,
    pub main_tabs: Style,
    pub tabs_selected: Style,
    pub borders: Style,
    pub description: Style,
    pub description_title: Style,
    pub key_binding: KeyBinding,
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

#[derive(Default, Clone, Debug)]
pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
}

#[derive(Theme, SubTheme, StyleTheme, Default, Clone, Debug)]
pub struct Logo {
    pub rat: Color,
    pub rat_eye: Color,
    pub rat_eye_alt: Color,
    pub term: Color,
}

#[derive(Theme, SubTheme, StyleTheme, Default, Clone, Debug)]
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

#[derive(Theme, SubTheme, StyleTheme, Default, Clone, Debug)]
pub struct Traceroute {
    pub header: Style,
    pub title: Style,
    pub selected: Style,
    pub ping: Style,
    pub map: Map,
}

#[derive(Theme, SubTheme, StyleTheme, Default, Clone, Debug)]
pub struct Map {
    pub main: Style,
    pub color: Color,
    pub path: Color,
    pub source: Color,
    pub destination: Color,
    pub background_color: Color,
}

#[derive(Theme, SubTheme, StyleTheme, Default, Clone, Debug)]
pub struct Recipe {
    pub ingredients: Style,
    pub ingredients_header: Style,
    pub selected: Style,
}

#[derive(SubTheme, StyleTheme, Theme, Default, Clone, Debug)]
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

#[derive(StyleTheme, SetTheme, Default, Clone, Debug)]
pub struct Colors {
    dark_blue: Color,
    light_blue: Color,
    light_yellow: Color,
    light_green: Color,
    light_red: Color,
    red: Color,
    black: Color,
    dark_gray: Color,
    mid_gray: Color,
    light_gray: Color,
    white: Color,
}

const THEMES: [Colors; 2] = [
    Colors {
        dark_blue: Catppuccin::BLUE_950,
        light_blue: Catppuccin::BLUE_300,
        light_yellow: Catppuccin::YELLOW_300,
        light_green: Catppuccin::GREEN_300,
        light_red: Catppuccin::ROSEWATER_300,
        red: Catppuccin::ROSEWATER_600,
        black: Catppuccin::BLUE_950,
        dark_gray: Catppuccin::GRAY_700,
        mid_gray: Catppuccin::GRAY_500,
        light_gray: Catppuccin::GRAY_300,
        white: Catppuccin::GRAY_50,
    },
    Colors {
        dark_blue: RosePine::GRAY_900,
        light_blue: RosePine::PINE_300,
        light_yellow: RosePine::GOLD_300,
        light_green: RosePine::FOAM_300,
        light_red: RosePine::ROSE_300,
        red: RosePine::ROSE_600,
        black: RosePine::GRAY_950,
        dark_gray: RosePine::GRAY_700,
        mid_gray: RosePine::GRAY_500,
        light_gray: RosePine::GRAY_300,
        white: RosePine::GRAY_50,
    },
];

pub fn num_themes() -> usize {
    THEMES.len()
}

pub fn init_theme(index: usize) {
    let colors = &THEMES[index];
    colors.set_global();
    let cur = Colors::current();
    let theme = AppTheme {
        root: Style::new().bg_dark_blue(),
        content: Style::new().bg_dark_blue().fg_light_gray(),
        app_title: Style::new()
            .fg_white()
            .bg_dark_blue()
            .modifiers(Modifiers::BOLD),
        main_tabs: Style::new().fg_mid_gray().bg_dark_blue(),
        tabs_selected: Style::new()
            .fg_white()
            .bg_dark_blue()
            .modifiers(Modifiers::BOLD | Modifiers::INVERT),
        borders: Style::new().fg_light_gray(),
        description: Style::new().fg_light_gray().bg_dark_blue(),
        description_title: Style::new().fg_light_gray().modifiers(Modifiers::BOLD),
        logo: Logo {
            rat: cur.white,
            rat_eye: cur.black,
            rat_eye_alt: cur.red,
            term: cur.black,
        },
        key_binding: KeyBinding {
            key: Style::new().fg_black().bg_dark_gray(),
            description: Style::new().fg_dark_gray().bg_black(),
        },
        email: Email {
            tabs: Style::new().fg_mid_gray().bg_dark_blue(),
            tabs_selected: Style::new()
                .fg_white()
                .bg_dark_blue()
                .modifiers(Modifiers::BOLD),
            inbox: Style::new().bg_dark_blue().fg_light_gray(),
            item: Style::new().fg_light_gray(),
            selected_item: Style::new().fg_light_yellow(),
            header: Style::new().modifiers(Modifiers::BOLD),
            header_value: Style::new().fg_light_gray(),
            body: Style::new().bg_dark_blue().fg_light_gray(),
        },
        traceroute: Traceroute {
            header: Style::new()
                .bg_dark_blue()
                .modifiers(Modifiers::BOLD | Modifiers::UNDERLINE),
            title: Style::new().fg_white().bold(),
            selected: Style::new().fg_light_yellow(),
            ping: Style::new().fg_white(),
            map: Map {
                main: Style::new().bg_dark_blue(),
                background_color: cur.dark_blue,
                color: cur.light_gray,
                path: cur.light_blue,
                source: cur.light_green,
                destination: cur.light_red,
            },
        },
        recipe: Recipe {
            ingredients: Style::new().bg_dark_blue().fg_light_gray(),
            ingredients_header: Style::new().modifiers(Modifiers::BOLD | Modifiers::UNDERLINE),
            selected: Style::new().fg_light_yellow(),
        },
        weather: Weather {
            bar1: Style::new().fg_red(),
            bar2: Style::new().fg_light_yellow(),
            bar_value1: Style::new().fg_mid_gray().bg_red().bold(),
            bar_value2: Style::new().fg_mid_gray().bg_light_yellow().bold(),
            calendar_day: Style::new().fg_red().bold(),
            progress: cur.light_yellow,
            progress_value: cur.mid_gray,
            line_gauge: cur.light_blue,
        },
    };

    theme.set_global();
}
