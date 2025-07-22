use std::io::{self, IsTerminal};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use ::palette::rgb::Rgb;
use ::palette::{
    Darken, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Lighten, Luv, Okhsl, Okhsv, Okhwb, Oklab, Oklch,
    Xyz, Yxy,
};
use palette::FromColor;
use terminal_colorsaurus::{ColorPalette, QueryOptions};
use termprofile::TermProfile;

mod convert;
mod parse;
pub use parse::*;

use crate::ThemeMode;

static TERM_PROFILE: OnceLock<TermProfile> = OnceLock::new();

static COLOR_PALETTE: OnceLock<Result<ColorPalette, PaletteError>> = OnceLock::new();

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

pub fn load_profile<T>(stream: &T)
where
    T: IsTerminal,
{
    let _ = TERM_PROFILE.set(TermProfile::detect(stream));
}

pub fn load_color_palette() {
    let _ = COLOR_PALETTE
        .set(terminal_colorsaurus::color_palette(QueryOptions::default()).map_err(Into::into));
}

pub fn profile() -> Result<TermProfile, ProfileError> {
    TERM_PROFILE.get().copied().ok_or(ProfileError::NotLoaded)
}

pub fn color_palette() -> Result<ColorPalette, PaletteError> {
    match COLOR_PALETTE.get() {
        Some(res) => res.clone(),
        None => Err(PaletteError::NotLoaded),
    }
}

pub fn color_scheme() -> ThemeMode {
    color_palette()
        .map(|p| p.theme_mode().into())
        .unwrap_or(ThemeMode::Dark)
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Hsl(Hsl),
    Hsluv(Hsluv),
    Hsv(Hsv),
    Hwb(Hwb),
    Lab(Lab),
    Lch(Lch),
    Lchuv(Lchuv),
    Luv(Luv),
    Okhsl(Okhsl),
    Okhsv(Okhsv),
    Okhwb(Okhwb),
    Oklab(Oklab),
    Oklch(Oklch),
    Xyz(Xyz),
    Yxy(Yxy),
    #[default]
    AnsiReset,
    /// ANSI Color: Black. Foreground: 30, Background: 40
    AnsiBlack,
    /// ANSI Color: Red. Foreground: 31, Background: 41
    AnsiRed,
    /// ANSI Color: Green. Foreground: 32, Background: 42
    AnsiGreen,
    /// ANSI Color: Yellow. Foreground: 33, Background: 43
    AnsiYellow,
    /// ANSI Color: Blue. Foreground: 34, Background: 44
    AnsiBlue,
    /// ANSI Color: Magenta. Foreground: 35, Background: 45
    AnsiMagenta,
    /// ANSI Color: Cyan. Foreground: 36, Background: 46
    AnsiCyan,
    /// ANSI Color: White. Foreground: 37, Background: 47
    ///
    /// Note that this is sometimes called `silver` or `white` but we use `white` for bright white
    AnsiGray,
    /// ANSI Color: Bright Black. Foreground: 90, Background: 100
    ///
    /// Note that this is sometimes called `light black` or `bright black` but we use `dark gray`
    AnsiDarkGray,
    /// ANSI Color: Bright Red. Foreground: 91, Background: 101
    AnsiLightRed,
    /// ANSI Color: Bright Green. Foreground: 92, Background: 102
    AnsiLightGreen,
    /// ANSI Color: Bright Yellow. Foreground: 93, Background: 103
    AnsiLightYellow,
    /// ANSI Color: Bright Blue. Foreground: 94, Background: 104
    AnsiLightBlue,
    /// ANSI Color: Bright Magenta. Foreground: 95, Background: 105
    AnsiLightMagenta,
    /// ANSI Color: Bright Cyan. Foreground: 96, Background: 106
    AnsiLightCyan,
    /// ANSI Color: Bright White. Foreground: 97, Background: 107
    /// Sometimes called `bright white` or `light white` in some terminals
    AnsiWhite,
    /// An 8-bit 256 color.
    ///
    /// See also <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
    Indexed(u8),
}

macro_rules! color_op {
    ($self:ident, $op:ident, $factor:expr) => {
        match $self {
            Self::Rgb(val) => Self::Rgb(val.$op($factor)),
            Self::Hsl(val) => Self::Hsl(val.$op($factor)),
            Self::Hsluv(val) => Self::Hsluv(val.$op($factor)),
            Self::Hsv(val) => Self::Hsv(val.$op($factor)),
            Self::Hwb(val) => Self::Hwb(val.$op($factor)),
            Self::Lab(val) => Self::Lab(val.$op($factor)),
            Self::Lch(val) => Self::Lch(val.$op($factor)),
            Self::Lchuv(val) => Self::Lchuv(val.$op($factor)),
            Self::Luv(val) => Self::Luv(val.$op($factor)),
            Self::Okhsl(val) => Self::Okhsl(val.$op($factor)),
            Self::Okhsv(val) => Self::Okhsv(val.$op($factor)),
            Self::Okhwb(val) => Self::Okhwb(val.$op($factor)),
            Self::Oklab(val) => Self::Oklab(val.$op($factor)),
            Self::Oklch(val) => Self::Oklch(val.$op($factor)),
            Self::Xyz(val) => Self::Xyz(val.lighten_fixed($factor)),
            Self::Yxy(val) => Self::Yxy(val.lighten_fixed($factor)),
            Self::Indexed(i) => indexed_to_rgb(*i).$op($factor),
            Self::AnsiReset => Self::AnsiReset,
            Self::AnsiBlack => indexed_to_rgb(0).$op($factor),
            Self::AnsiRed => indexed_to_rgb(1).$op($factor),
            Self::AnsiGreen => indexed_to_rgb(2).$op($factor),
            Self::AnsiYellow => indexed_to_rgb(3).$op($factor),
            Self::AnsiBlue => indexed_to_rgb(4).$op($factor),
            Self::AnsiMagenta => indexed_to_rgb(5).$op($factor),
            Self::AnsiCyan => indexed_to_rgb(6).$op($factor),
            Self::AnsiGray => indexed_to_rgb(7).$op($factor),
            Self::AnsiDarkGray => indexed_to_rgb(8).$op($factor),
            Self::AnsiLightRed => indexed_to_rgb(9).$op($factor),
            Self::AnsiLightGreen => indexed_to_rgb(10).$op($factor),
            Self::AnsiLightYellow => indexed_to_rgb(11).$op($factor),
            Self::AnsiLightBlue => indexed_to_rgb(12).$op($factor),
            Self::AnsiLightMagenta => indexed_to_rgb(13).$op($factor),
            Self::AnsiLightCyan => indexed_to_rgb(14).$op($factor),
            Self::AnsiWhite => indexed_to_rgb(15).$op($factor),
        }
    };
}

impl Color {
    pub fn terminal_foreground() -> Self {
        color_palette()
            .map(|p| Self::scale_color(&p.foreground))
            .unwrap_or_default()
    }

    pub fn terminal_background() -> Self {
        color_palette()
            .map(|p| Self::scale_color(&p.background))
            .unwrap_or_default()
    }

    fn scale_color(color: &terminal_colorsaurus::Color) -> Self {
        let color = color.scale_to_8bit();
        Self::Rgb(Rgb::new(
            color.0 as f32 / 255.,
            color.1 as f32 / 255.,
            color.2 as f32 / 255.,
        ))
    }

    pub fn is_compatible(&self) -> bool {
        let color_support = profile().unwrap_or(TermProfile::TrueColor);
        match self {
            Self::AnsiWhite
            | Self::AnsiGray
            | Self::AnsiBlue
            | Self::AnsiCyan
            | Self::AnsiMagenta
            | Self::AnsiGreen
            | Self::AnsiYellow
            | Self::AnsiRed
            | Self::AnsiLightBlue
            | Self::AnsiLightRed
            | Self::AnsiLightGreen
            | Self::AnsiLightCyan
            | Self::AnsiLightMagenta
            | Self::AnsiLightYellow
            | Self::AnsiReset
            | Self::AnsiBlack
            | Self::AnsiDarkGray => color_support >= TermProfile::Ansi16,
            Self::Indexed(index) if *index < 16 => color_support >= TermProfile::Ansi16,
            Self::Indexed(_) => color_support >= TermProfile::Ansi256,
            Self::Rgb(_)
            | Self::Hsl(_)
            | Self::Hsv(_)
            | Self::Hsluv(_)
            | Self::Hwb(_)
            | Self::Lab(_)
            | Self::Okhsl(_)
            | Self::Oklab(_)
            | Self::Lch(_)
            | Self::Lchuv(_)
            | Self::Luv(_)
            | Self::Okhsv(_)
            | Self::Okhwb(_)
            | Self::Oklch(_)
            | Self::Xyz(_)
            | Self::Yxy(_) => color_support >= TermProfile::TrueColor,
        }
    }

    pub fn into_adaptive(self) -> Self {
        if self.is_compatible() {
            return self;
        }
        self.into_anstyle()
            .map(Into::into)
            .unwrap_or(Self::AnsiReset)
    }

    fn into_anstyle(self) -> Option<anstyle::Color> {
        let value = match self {
            Color::AnsiReset => return None,
            Color::AnsiBlack => anstyle::Color::Ansi(anstyle::AnsiColor::Black),
            Color::AnsiRed => anstyle::Color::Ansi(anstyle::AnsiColor::Red),
            Color::AnsiGreen => anstyle::Color::Ansi(anstyle::AnsiColor::Green),
            Color::AnsiYellow => anstyle::Color::Ansi(anstyle::AnsiColor::Yellow),
            Color::AnsiBlue => anstyle::Color::Ansi(anstyle::AnsiColor::Blue),
            Color::AnsiMagenta => anstyle::Color::Ansi(anstyle::AnsiColor::Magenta),
            Color::AnsiCyan => anstyle::Color::Ansi(anstyle::AnsiColor::Cyan),
            Color::AnsiGray => anstyle::Color::Ansi(anstyle::AnsiColor::White),
            Color::AnsiDarkGray => anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlack),
            Color::AnsiLightRed => anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed),
            Color::AnsiLightGreen => anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen),
            Color::AnsiLightYellow => anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow),
            Color::AnsiLightBlue => anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue),
            Color::AnsiLightMagenta => anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta),
            Color::AnsiLightCyan => anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan),
            Color::AnsiWhite => anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite),
            Color::Indexed(index) => anstyle::Color::Ansi256(anstyle::Ansi256Color(index)),
            Color::Rgb(rgb_color) => palette_to_anstyle(rgb_color),
            Color::Hsl(val) => palette_to_anstyle(Rgb::from_color(val)),
            Color::Hsluv(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Hsv(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Hwb(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Lab(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Lch(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Lchuv(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Luv(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Okhsl(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Okhsv(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Okhwb(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Oklab(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Oklch(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Xyz(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Yxy(val) => {
                palette_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
            }
        };
        let profile = profile().unwrap_or(TermProfile::TrueColor);
        profile.adapt_color(value)
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

fn indexed_to_rgb(index: u8) -> Color {
    Color::parse_hex(ANSI_HEX[index as usize]).unwrap()
}

fn palette_to_anstyle(rgb_color: Rgb) -> anstyle::Color {
    anstyle::Color::Rgb(anstyle::RgbColor(
        (rgb_color.red * 255.) as u8,
        (rgb_color.green * 255.) as u8,
        (rgb_color.blue * 255.) as u8,
    ))
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
