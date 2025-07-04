use std::io::IsTerminal;
use std::sync::{LazyLock, RwLock};

use ::palette::rgb::Rgb;
use ::palette::{
    Darken, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Lighten, Luv, Okhsl, Okhsv, Okhwb, Oklab, Oklch,
    Xyz, Yxy,
};
use terminal_colorsaurus::{ColorPalette, ColorScheme, QueryOptions, color_palette};
use termprofile::TermProfile;

mod convert;
mod parse;
pub use parse::*;

static TERM_PROFILE: LazyLock<RwLock<TermProfile>> =
    LazyLock::new(|| RwLock::new(TermProfile::TrueColor));

static COLOR_PALETTE: LazyLock<RwLock<Option<ColorPalette>>> =
    LazyLock::new(|| RwLock::new(color_palette(QueryOptions::default()).ok()));

pub fn load_profile<T>(stream: &T)
where
    T: IsTerminal,
{
    *TERM_PROFILE.write().unwrap() = TermProfile::detect(stream)
}

pub fn load_color_palette() {
    drop(COLOR_PALETTE.read().unwrap());
}

pub(crate) fn color_scheme() -> ColorScheme {
    COLOR_PALETTE
        .read()
        .unwrap()
        .as_ref()
        .map(|p| p.color_scheme())
        .unwrap_or_default()
}

#[derive(Debug, Clone, Copy, Default)]
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
        COLOR_PALETTE
            .read()
            .unwrap()
            .as_ref()
            .map(|p| Self::scale_color(&p.foreground))
            .unwrap_or_default()
    }

    pub fn terminal_background() -> Self {
        COLOR_PALETTE
            .read()
            .unwrap()
            .as_ref()
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
        let color_support = TERM_PROFILE.read().unwrap();
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
            | Self::AnsiDarkGray => *color_support >= TermProfile::Ansi16,
            Self::Indexed(index) if *index < 16 => *color_support >= TermProfile::Ansi16,
            Self::Indexed(_) => *color_support >= TermProfile::Ansi256,
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
            | Self::Yxy(_) => *color_support >= TermProfile::TrueColor,
        }
    }

    pub fn into_adaptive(self) -> Self {
        if self.is_compatible() {
            return self;
        }
        let anstyle_color: Option<anstyle::Color> = self.into();
        let Some(color) = anstyle_color else {
            return self;
        };
        if let Some(adapted) = TERM_PROFILE.read().unwrap().adapt(color) {
            adapted.into()
        } else {
            Self::AnsiReset
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

fn indexed_to_rgb(index: u8) -> Color {
    Color::parse_hex(ANSI_HEX[index as usize]).unwrap()
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
