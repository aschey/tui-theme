use std::borrow::Cow;
use std::env;
use std::fmt::Display;
use std::io::{self, IsTerminal};
use std::sync::Arc;
use std::time::Duration;

use ::palette::{Darken, Lighten};
use palette::Srgb;
use termprofile::{DetectorSettings, TermProfile, TermVars};

mod convert;
mod parse;
pub use parse::*;

use crate::{ThemeMode, local_override};

#[derive(Clone)]
struct CurrentProfile(TermProfile);

impl Default for CurrentProfile {
    fn default() -> Self {
        CurrentProfile(TermProfile::TrueColor)
    }
}

local_override!(CurrentProfile, GLOBAL_TERM_PROFILE, LOCAL_TERM_PROFILE);

local_override!(ColorPalette, GLOBAL_COLOR_PALETTE, LOCAL_COLOR_PALETTE);

#[derive(Clone, Copy, Debug)]
pub struct ColorPalette {
    pub fg: Color,
    pub bg: Color,
    pub theme_mode: ThemeMode,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            fg: Color::White,
            bg: Color::Black,
            theme_mode: ThemeMode::Dark,
        }
    }
}

impl From<terminal_colorsaurus::ColorPalette> for ColorPalette {
    fn from(value: terminal_colorsaurus::ColorPalette) -> Self {
        ColorPalette {
            fg: Color::Rgb(
                value.foreground.r as u8,
                value.foreground.g as u8,
                value.foreground.b as u8,
            ),
            bg: Color::Rgb(
                value.background.r as u8,
                value.background.g as u8,
                value.background.b as u8,
            ),
            theme_mode: override_color_scheme().unwrap_or_else(|| value.theme_mode().into()),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PaletteError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(Arc<io::Error>),
    /// The terminal responded using an unsupported response format
    #[error("unsupported format: {0:?}")]
    Parse(Vec<u8>),
    /// The query timed out. This can happen because \
    /// either the terminal does not support querying for colors \
    /// or the terminal has a lot of latency (e.g. when connected via SSH).
    #[error("query timed out: {0:?}")]
    Timeout(Duration),
    /// The terminal does not support querying for the foreground or background color.
    #[error("unsupported terminal")]
    UnsupportedTerminal,
    #[error("the palette has not been loaded")]
    NotLoaded,
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProfileError {
    #[error("the profile has not been loaded")]
    NotLoaded,
}

impl From<terminal_colorsaurus::Error> for PaletteError {
    fn from(value: terminal_colorsaurus::Error) -> Self {
        match value {
            terminal_colorsaurus::Error::Io(e) => Self::Io(e.into()),
            terminal_colorsaurus::Error::Parse(e) => Self::Parse(e),
            terminal_colorsaurus::Error::Timeout(d) => Self::Timeout(d),
            terminal_colorsaurus::Error::UnsupportedTerminal(_) => Self::UnsupportedTerminal,
            _ => Self::Unknown,
        }
    }
}

pub fn load_profile<T>(stream: &T, settings: DetectorSettings)
where
    T: IsTerminal,
{
    CurrentProfile(TermProfile::detect(stream, settings)).override_set_global();
}

pub fn load_profile_local<T>(stream: &T, settings: DetectorSettings)
where
    T: IsTerminal,
{
    CurrentProfile(TermProfile::detect(stream, settings)).override_set_local();
}

pub fn load_profile_with_vars<T>(stream: &T, vars: TermVars)
where
    T: IsTerminal,
{
    CurrentProfile(TermProfile::detect_with_vars(stream, vars)).override_set_global();
}

pub fn load_profile_local_with_vars<T>(stream: &T, vars: TermVars)
where
    T: IsTerminal,
{
    CurrentProfile(TermProfile::detect_with_vars(stream, vars)).override_set_local();
}

pub fn set_profile(profile: TermProfile) {
    CurrentProfile(profile).override_set_global();
}

pub fn set_profile_local(profile: TermProfile) {
    CurrentProfile(profile).override_set_local();
}

pub fn load_color_palette() {
    let palette =
        terminal_colorsaurus::color_palette(terminal_colorsaurus::QueryOptions::default());
    // Somewhat non-standard variable but can be useful for some terminals
    // see https://github.com/bash/terminal-colorsaurus/issues/26
    if matches!(
        palette,
        Err(terminal_colorsaurus::Error::UnsupportedTerminal(_))
    ) && let Some((fg, bg)) = Color::parse_colorfgbg("COLORFGBG")
    {
        let palette = get_palette_from_override(fg, bg);
        palette.override_set_global();
        return;
    }
    if let Ok(palette) = palette {
        let palette: ColorPalette = palette.into();
        palette.override_set_global();
    }
}

pub fn load_color_palette_local() {
    let palette =
        terminal_colorsaurus::color_palette(terminal_colorsaurus::QueryOptions::default());
    // Somewhat non-standard variable but can be useful for some terminals
    // see https://github.com/bash/terminal-colorsaurus/issues/26
    if matches!(
        palette,
        Err(terminal_colorsaurus::Error::UnsupportedTerminal(_))
    ) && let Some((fg, bg)) = Color::parse_colorfgbg("COLORFGBG")
    {
        let palette = get_palette_from_override(fg, bg);
        palette.override_set_local();
        return;
    }
    if let Ok(palette) = palette {
        let palette: ColorPalette = palette.into();
        palette.override_set_local();
    }
}

pub fn set_color_palette(palette: ColorPalette) {
    palette.override_set_global();
}

pub fn set_color_palette_local(palette: ColorPalette) {
    palette.override_set_global();
}

fn get_palette_from_override(fg: Color, bg: Color) -> ColorPalette {
    let theme_mode = override_color_scheme().unwrap_or_else(|| {
        if bg == Color::White || bg == Color::Gray {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        }
    });
    ColorPalette { fg, bg, theme_mode }
}

pub fn term_profile() -> TermProfile {
    CurrentProfile::override_current().0
}

pub fn is_supported(profile: TermProfile) -> bool {
    term_profile() >= profile
}

pub fn color_palette() -> ColorPalette {
    ColorPalette::override_current()
}

fn override_color_scheme() -> Option<ThemeMode> {
    // https://wiki.tau.garden/cli-theme/
    match env::var("CLITHEME")
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Ok("light") => Some(ThemeMode::Light),
        Ok("dark") => Some(ThemeMode::Dark),
        _ => None,
    }
}

pub fn color_scheme() -> ThemeMode {
    color_palette().theme_mode
}

#[derive(Clone, Debug)]
pub struct NamedColor<'a> {
    pub color: Color,
    pub group: Cow<'a, str>,
    pub variant: Cow<'a, str>,
}

impl NamedColor<'_> {
    pub fn full_name(&self) -> String {
        format!("{}-{}", self.group, self.variant)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    #[default]
    Reset,
    /// ANSI Color: Black. Foreground: 30, Background: 40
    Black,
    /// ANSI Color: Red. Foreground: 31, Background: 41
    Red,
    /// ANSI Color: Green. Foreground: 32, Background: 42
    Green,
    /// ANSI Color: Yellow. Foreground: 33, Background: 43
    Yellow,
    /// ANSI Color: Blue. Foreground: 34, Background: 44
    Blue,
    /// ANSI Color: Magenta. Foreground: 35, Background: 45
    Magenta,
    /// ANSI Color: Cyan. Foreground: 36, Background: 46
    Cyan,
    /// ANSI Color: White. Foreground: 37, Background: 47
    ///
    /// Note that this is sometimes called `silver` or `white` but we use `white` for bright white
    Gray,
    /// ANSI Color: Bright Black. Foreground: 90, Background: 100
    ///
    /// Note that this is sometimes called `light black` or `bright black` but we use `dark gray`
    DarkGray,
    /// ANSI Color: Bright Red. Foreground: 91, Background: 101
    LightRed,
    /// ANSI Color: Bright Green. Foreground: 92, Background: 102
    LightGreen,
    /// ANSI Color: Bright Yellow. Foreground: 93, Background: 103
    LightYellow,
    /// ANSI Color: Bright Blue. Foreground: 94, Background: 104
    LightBlue,
    /// ANSI Color: Bright Magenta. Foreground: 95, Background: 105
    LightMagenta,
    /// ANSI Color: Bright Cyan. Foreground: 96, Background: 106
    LightCyan,
    /// ANSI Color: Bright White. Foreground: 97, Background: 107
    /// Sometimes called `bright white` or `light white` in some terminals
    White,
    /// An 8-bit 256 color.
    ///
    /// See also <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
    Indexed(u8),
}

macro_rules! color_op {
    ($self:ident, $op:ident, $factor:expr) => {
        match $self {
            Self::Rgb(r, g, b) => {
                palette::Srgb::new(*r as f32 / 255., *g as f32 / 255., *b as f32 / 255.)
                    .into_linear()
                    .$op($factor)
                    .into()
            }
            Self::Indexed(i) => indexed_to_color(*i).$op($factor),
            Self::Reset => Self::Reset,
            Self::Black => indexed_to_color(0).$op($factor),
            Self::Red => indexed_to_color(1).$op($factor),
            Self::Green => indexed_to_color(2).$op($factor),
            Self::Yellow => indexed_to_color(3).$op($factor),
            Self::Blue => indexed_to_color(4).$op($factor),
            Self::Magenta => indexed_to_color(5).$op($factor),
            Self::Cyan => indexed_to_color(6).$op($factor),
            Self::Gray => indexed_to_color(7).$op($factor),
            Self::DarkGray => indexed_to_color(8).$op($factor),
            Self::LightRed => indexed_to_color(9).$op($factor),
            Self::LightGreen => indexed_to_color(10).$op($factor),
            Self::LightYellow => indexed_to_color(11).$op($factor),
            Self::LightBlue => indexed_to_color(12).$op($factor),
            Self::LightMagenta => indexed_to_color(13).$op($factor),
            Self::LightCyan => indexed_to_color(14).$op($factor),
            Self::White => indexed_to_color(15).$op($factor),
        }
    };
}

impl Color {
    pub fn terminal_background() -> Self {
        color_palette().bg
    }

    pub fn terminal_foreground() -> Self {
        color_palette().fg
    }

    pub fn is_compatible(&self) -> bool {
        let color_support = term_profile();
        match self {
            Self::White
            | Self::Gray
            | Self::Blue
            | Self::Cyan
            | Self::Magenta
            | Self::Green
            | Self::Yellow
            | Self::Red
            | Self::LightBlue
            | Self::LightRed
            | Self::LightGreen
            | Self::LightCyan
            | Self::LightMagenta
            | Self::LightYellow
            | Self::Reset
            | Self::Black
            | Self::DarkGray => color_support >= TermProfile::Ansi16,
            Self::Indexed(index) if *index < 16 => color_support >= TermProfile::Ansi16,
            Self::Indexed(_) => color_support >= TermProfile::Ansi256,
            Self::Rgb(_, _, _) => color_support >= TermProfile::TrueColor,
        }
    }

    fn parse_colorfgbg(env_var: &str) -> Option<(Color, Color)> {
        if let Ok(fgbg) = env::var(env_var) {
            let fgbg: Vec<_> = fgbg.split(";").collect();
            match &fgbg[..] {
                // urxvt may set a third variable, but we can ignore it
                [fg, bg] | [fg, _, bg] => {
                    let fg: u8 = fg.parse().ok()?;
                    let bg: u8 = bg.parse().ok()?;
                    return Some((Color::ansi_from_index(fg), Color::ansi_from_index(bg)));
                }
                _ => {}
            }
        }
        None
    }

    fn ansi_from_index(index: u8) -> Self {
        match index {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::Gray,
            8 => Color::DarkGray,
            9 => Color::LightRed,
            10 => Color::LightGreen,
            11 => Color::LightYellow,
            12 => Color::LightBlue,
            13 => Color::LightMagenta,
            14 => Color::LightCyan,
            15 => Color::White,
            _ => panic!("invalid index"),
        }
    }

    pub fn lighten(&self, factor: f32) -> Self {
        color_op!(self, lighten, factor)
    }

    pub fn lighten_fixed(&self, factor: f32) -> Self {
        color_op!(self, lighten_fixed, factor)
    }

    pub fn darken(&self, factor: f32) -> Self {
        color_op!(self, darken, factor)
    }

    pub fn darken_fixed(&self, factor: f32) -> Self {
        color_op!(self, darken_fixed, factor)
    }
}

fn indexed_to_color(index: u8) -> Color {
    Color::parse_hex(ANSI_HEX[index as usize]).unwrap()
}

pub fn indexed_to_rgb(index: u8) -> Srgb<u8> {
    ANSI_HEX[index as usize].parse().unwrap()
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rgb(r, g, b) => write!(f, "rgb({r} {g} {b})",),
            Self::Indexed(i) => write!(f, "{i}"),
            Self::Reset => write!(f, "reset"),
            Self::Black => write!(f, "black"),
            Self::Red => write!(f, "red"),
            Self::Green => write!(f, "green"),
            Self::Yellow => write!(f, "yellow"),
            Self::Blue => write!(f, "blue"),
            Self::Magenta => write!(f, "magenta"),
            Self::Cyan => write!(f, "cyan"),
            Self::Gray => write!(f, "gray"),
            Self::DarkGray => write!(f, "darkgray"),
            Self::LightRed => write!(f, "lightred"),
            Self::LightGreen => write!(f, "lightgreen"),
            Self::LightYellow => write!(f, "lightyellow"),
            Self::LightBlue => write!(f, "lightblue"),
            Self::LightMagenta => write!(f, "lightmagenta"),
            Self::LightCyan => write!(f, "lightcyan"),
            Self::White => write!(f, "white"),
        }
    }
}

const ANSI_HEX: [&str; 256] = [
    "#000000", "#800000", "#008000", "#808000", "#000080", "#800080", "#008080", "#c0c0c0",
    "#808080", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff",
    "#000000", "#00005f", "#000087", "#0000af", "#0000d7", "#0000ff", "#005f00", "#005f5f",
    "#005f87", "#005faf", "#005fd7", "#005fff", "#008700", "#00875f", "#008787", "#0087af",
    "#0087d7", "#0087ff", "#00af00", "#00af5f", "#00af87", "#00afaf", "#00afd7", "#00afff",
    "#00d700", "#00d75f", "#00d787", "#00d7af", "#00d7d7", "#00d7ff", "#00ff00", "#00ff5f",
    "#00ff87", "#00ffaf", "#00ffd7", "#00ffff", "#5f0000", "#5f005f", "#5f0087", "#5f00af",
    "#5f00d7", "#5f00ff", "#5f5f00", "#5f5f5f", "#5f5f87", "#5f5faf", "#5f5fd7", "#5f5fff",
    "#5f8700", "#5f875f", "#5f8787", "#5f87af", "#5f87d7", "#5f87ff", "#5faf00", "#5faf5f",
    "#5faf87", "#5fafaf", "#5fafd7", "#5fafff", "#5fd700", "#5fd75f", "#5fd787", "#5fd7af",
    "#5fd7d7", "#5fd7ff", "#5fff00", "#5fff5f", "#5fff87", "#5fffaf", "#5fffd7", "#5fffff",
    "#870000", "#87005f", "#870087", "#8700af", "#8700d7", "#8700ff", "#875f00", "#875f5f",
    "#875f87", "#875faf", "#875fd7", "#875fff", "#878700", "#87875f", "#878787", "#8787af",
    "#8787d7", "#8787ff", "#87af00", "#87af5f", "#87af87", "#87afaf", "#87afd7", "#87afff",
    "#87d700", "#87d75f", "#87d787", "#87d7af", "#87d7d7", "#87d7ff", "#87ff00", "#87ff5f",
    "#87ff87", "#87ffaf", "#87ffd7", "#87ffff", "#af0000", "#af005f", "#af0087", "#af00af",
    "#af00d7", "#af00ff", "#af5f00", "#af5f5f", "#af5f87", "#af5faf", "#af5fd7", "#af5fff",
    "#af8700", "#af875f", "#af8787", "#af87af", "#af87d7", "#af87ff", "#afaf00", "#afaf5f",
    "#afaf87", "#afafaf", "#afafd7", "#afafff", "#afd700", "#afd75f", "#afd787", "#afd7af",
    "#afd7d7", "#afd7ff", "#afff00", "#afff5f", "#afff87", "#afffaf", "#afffd7", "#afffff",
    "#d70000", "#d7005f", "#d70087", "#d700af", "#d700d7", "#d700ff", "#d75f00", "#d75f5f",
    "#d75f87", "#d75faf", "#d75fd7", "#d75fff", "#d78700", "#d7875f", "#d78787", "#d787af",
    "#d787d7", "#d787ff", "#d7af00", "#d7af5f", "#d7af87", "#d7afaf", "#d7afd7", "#d7afff",
    "#d7d700", "#d7d75f", "#d7d787", "#d7d7af", "#d7d7d7", "#d7d7ff", "#d7ff00", "#d7ff5f",
    "#d7ff87", "#d7ffaf", "#d7ffd7", "#d7ffff", "#ff0000", "#ff005f", "#ff0087", "#ff00af",
    "#ff00d7", "#ff00ff", "#ff5f00", "#ff5f5f", "#ff5f87", "#ff5faf", "#ff5fd7", "#ff5fff",
    "#ff8700", "#ff875f", "#ff8787", "#ff87af", "#ff87d7", "#ff87ff", "#ffaf00", "#ffaf5f",
    "#ffaf87", "#ffafaf", "#ffafd7", "#ffafff", "#ffd700", "#ffd75f", "#ffd787", "#ffd7af",
    "#ffd7d7", "#ffd7ff", "#ffff00", "#ffff5f", "#ffff87", "#ffffaf", "#ffffd7", "#ffffff",
    "#080808", "#121212", "#1c1c1c", "#262626", "#303030", "#3a3a3a", "#444444", "#4e4e4e",
    "#585858", "#626262", "#6c6c6c", "#767676", "#808080", "#8a8a8a", "#949494", "#9e9e9e",
    "#a8a8a8", "#b2b2b2", "#bcbcbc", "#c6c6c6", "#d0d0d0", "#dadada", "#e4e4e4", "#eeeeee",
];

#[cfg(test)]
#[path = "./color_test.rs"]
mod color_test;
