use std::borrow::Cow;
use std::sync::LazyLock;

use confique::Config;
use tui_theme::palette::{
    Catppuccin, Everforest, Gruvbox, Kanagawa, Monokai, Nord, OneDark, RosePine, Solarized,
    Tailwind, TokyoNight,
};
use tui_theme::{Color, ColorScheme, Dark, Light, Theme, color_scheme, deserialize_color};

#[derive(Theme, Config, Default, Clone, Debug)]
pub struct ThemeColors {
    theme_name: Cow<'static, str>,
    #[config(deserialize_with = deserialize_color)]
    base1: Color,
    #[config(deserialize_with = deserialize_color)]
    base2: Color,
    #[config(deserialize_with = deserialize_color)]
    text: Color,
    #[config(deserialize_with = deserialize_color)]
    text_muted: Color,
    #[config(deserialize_with = deserialize_color)]
    text_bright: Color,
    #[config(deserialize_with = deserialize_color)]
    primary: Color,
    #[config(deserialize_with = deserialize_color)]
    accent: Color,
    #[config(deserialize_with = deserialize_color)]
    success: Color,
    #[config(deserialize_with = deserialize_color)]
    warning: Color,
    #[config(deserialize_with = deserialize_color)]
    danger: Color,
    #[config(deserialize_with = deserialize_color)]
    selected: Color,
}

pub const BASIC_ANSI_THEME: ThemeColors = ThemeColors {
    theme_name: Cow::Borrowed("ansi"),
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

const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/theme.toml");

pub static THEMES: LazyLock<[ThemeColors; 13]> = LazyLock::new(|| {
    let file_config = match ThemeColors::builder().file(CONFIG_PATH).load() {
        Ok(config) => config,
        Err(e) => {
            panic!("{e:#?}")
        }
    };
    let bg = Color::terminal_background();
    let fg = Color::terminal_foreground();
    let rel_luma = bg.luminance_bg();
    let factor = 0.15;
    let base2 = if rel_luma <= 0.01 {
        bg.lighten(factor)
    } else {
        bg.darken(factor)
    };

    let scheme = color_scheme();

    let ansi = ThemeColors {
        theme_name: Cow::Borrowed("ansi"),
        base1: bg,
        base2,
        primary: Color::Blue,
        accent: Color::Cyan,
        success: Color::Green,
        warning: Color::Yellow,
        danger: Color::Red,
        text: fg,
        text_muted: fg.darken(0.15),
        text_bright: fg.lighten(0.15),
        selected: Color::Yellow,
    };
    let base1 = (Light(0), Dark(10));
    let base2 = (Light(1), Dark(9));
    let primary = (Light(6), Dark(3));
    let accent = (Light(6), Dark(3));
    let success = (Light(6), Dark(3));
    let warning = (Light(6), Dark(3));
    let danger = (Light(6), Dark(3));
    let text = (Light(7), Dark(3));
    let text_muted = (Light(6), Dark(5));
    let text_bright = (Light(9), Dark(1));
    let selected = (Light(5), Dark(3));
    [
        file_config,
        ansi,
        ThemeColors {
            theme_name: Cow::Borrowed(Catppuccin::NAME),
            base1: Catppuccin::GRAY[base1],
            base2: Catppuccin::GRAY[base2],
            primary: Catppuccin::BLUE[primary],
            accent: Catppuccin::SAPPHIRE[accent],
            success: Catppuccin::GREEN[success],
            warning: Catppuccin::YELLOW[warning],
            danger: Catppuccin::ROSEWATER[danger],
            text: Catppuccin::GRAY[text],
            text_muted: Catppuccin::GRAY[text_muted],
            text_bright: Catppuccin::GRAY[text_bright],
            selected: Catppuccin::YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Everforest::NAME),
            base1: Everforest::BLUE_GRAY[base1],
            base2: Everforest::BLUE_GRAY[base2],
            primary: Everforest::BLUE_GRAY[primary],
            accent: Everforest::BLUE[accent],
            success: Everforest::GREEN[success],
            warning: Everforest::YELLOW[warning],
            danger: Everforest::RED[danger],
            text: Everforest::BLUE[text],
            text_muted: Everforest::BLUE_GRAY[text_muted],
            text_bright: Everforest::BLUE_GRAY[text_bright],
            selected: Everforest::YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(RosePine::NAME),
            base1: RosePine::GRAY[base1],
            base2: RosePine::GRAY[base2],
            primary: RosePine::PINE[primary],
            accent: RosePine::ROSE[accent],
            success: RosePine::FOAM[success],
            warning: RosePine::GOLD[warning],
            danger: RosePine::ROSE[danger],
            text: RosePine::GRAY[text],
            text_muted: RosePine::GRAY[text_muted],
            text_bright: RosePine::GRAY[text_bright],
            selected: RosePine::GOLD[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Gruvbox::NAME),
            base1: Gruvbox::BROWN[base1],
            base2: Gruvbox::BROWN[base2],
            primary: Gruvbox::GREEN[primary],
            accent: Gruvbox::ORANGE[accent],
            success: Gruvbox::AQUA[success],
            warning: Gruvbox::YELLOW[warning],
            danger: Gruvbox::RED[danger],
            text: Gruvbox::GRAY[text],
            text_muted: Gruvbox::GRAY[text_muted],
            text_bright: Gruvbox::GRAY[text_bright],
            selected: Gruvbox::YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Kanagawa::NAME),
            base1: Kanagawa::HONEYDEW[base1],
            base2: Kanagawa::HONEYDEW[base2],
            primary: Kanagawa::INDIGO[primary],
            accent: Kanagawa::AQUA[accent],
            success: Kanagawa::GREEN[success],
            warning: Kanagawa::PEACH[warning],
            danger: Kanagawa::RED[danger],
            text: Kanagawa::GRAY[text],
            text_muted: Kanagawa::GRAY[text_muted],
            text_bright: Kanagawa::GRAY[text_bright],
            selected: Kanagawa::EARTH_YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Monokai::NAME),
            base1: Monokai::GRAY[base1],
            base2: Monokai::GRAY[base2],
            primary: Monokai::BLUE[primary],
            accent: Monokai::PURPLE[accent],
            success: Monokai::GREEN[success],
            warning: Monokai::YELLOW[warning],
            danger: Monokai::RED[warning],
            text: Monokai::GRAY[text],
            text_muted: Monokai::GRAY[text_muted],
            text_bright: Monokai::GRAY[text_bright],
            selected: Monokai::YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Nord::NAME),
            base1: Nord::GRAY[base1],
            base2: Nord::GRAY[base2],
            primary: Nord::BLUE[primary],
            accent: Nord::TEAL[accent],
            success: Nord::GREEN[success],
            warning: Nord::YELLOW[warning],
            danger: Nord::RED[danger],
            text: Nord::GRAY[text],
            text_muted: Nord::GRAY[text_muted],
            text_bright: Nord::GRAY[text_bright],
            selected: Nord::YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(OneDark::NAME),
            base1: OneDark::GRAY[base1],
            base2: OneDark::GRAY[base2],
            primary: OneDark::BLUE[primary],
            accent: OneDark::CYAN[accent],
            success: OneDark::GREEN[success],
            warning: OneDark::YELLOW[warning],
            danger: OneDark::RED[danger],
            text: OneDark::GRAY[text],
            text_muted: OneDark::GRAY[text_muted],
            text_bright: OneDark::GRAY[text_bright],
            selected: OneDark::PEACH[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Solarized::NAME),
            base1: if scheme == ColorScheme::Dark {
                Solarized::BASE_BLUE[base1]
            } else {
                Solarized::BROWN[base1]
            },
            base2: if scheme == ColorScheme::Dark {
                Solarized::BASE_BLUE[base2]
            } else {
                Solarized::BROWN[base2]
            },
            primary: Solarized::BLUE[primary],
            accent: Solarized::CYAN[accent],
            success: Solarized::GREEN[accent],
            warning: Solarized::YELLOW[warning],
            danger: Solarized::RED[danger],
            text: Solarized::GRAY[text],
            text_muted: Solarized::GRAY[text_muted],
            text_bright: Solarized::GRAY[text_bright],
            selected: Solarized::YELLOW[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(Tailwind::NAME),
            base1: Tailwind::GRAY[base1],
            base2: Tailwind::GRAY[base2],
            primary: Tailwind::SKY[primary],
            accent: Tailwind::BLUE[accent],
            success: Tailwind::GREEN[success],
            warning: Tailwind::YELLOW[warning],
            danger: Tailwind::RED[danger],
            text: Tailwind::GRAY[text],
            text_muted: Tailwind::GRAY[text_muted],
            text_bright: Tailwind::GRAY[text_bright],
            selected: Tailwind::AMBER[selected],
        },
        ThemeColors {
            theme_name: Cow::Borrowed(TokyoNight::NAME),
            base1: TokyoNight::GRAY[base1],
            base2: TokyoNight::GRAY[base2],
            primary: TokyoNight::BLUE[primary],
            accent: TokyoNight::CYAN[accent],
            success: TokyoNight::GREEN[success],
            warning: TokyoNight::YELLOW[warning],
            danger: TokyoNight::RED[danger],
            text: TokyoNight::GRAY[text],
            text_muted: TokyoNight::GRAY[text_muted],
            text_bright: TokyoNight::GRAY[text_bright],
            selected: TokyoNight::YELLOW[selected],
        },
    ]
});

pub fn num_themes() -> usize {
    THEMES.len()
}

pub fn theme_name(index: usize) -> String {
    THEMES[index].theme_name.to_string()
}
