use ::palette::{
    Darken, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Lighten, Luv, Okhsl, Okhsv, Okhwb,
    Oklab, Oklch, Xyz, Yxy, rgb::Rgb,
};
use std::{
    io::IsTerminal,
    sync::{LazyLock, RwLock},
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
        match self {
            Self::Rgb(val) => Self::Rgb(val.lighten(factor)),
            Self::Hsl(val) => Self::Hsl(val.lighten(factor)),
            Self::Hsluv(val) => Self::Hsluv(val.lighten(factor)),
            Self::Hsv(val) => Self::Hsv(val.lighten(factor)),
            Self::Hwb(val) => Self::Hwb(val.lighten(factor)),
            Self::Lab(val) => Self::Lab(val.lighten(factor)),
            Self::Lch(val) => Self::Lch(val.lighten(factor)),
            Self::Lchuv(val) => Self::Lchuv(val.lighten(factor)),
            Self::Luv(val) => Self::Luv(val.lighten(factor)),
            Self::Okhsl(val) => Self::Okhsl(val.lighten(factor)),
            Self::Okhsv(val) => Self::Okhsv(val.lighten(factor)),
            Self::Okhwb(val) => Self::Okhwb(val.lighten(factor)),
            Self::Oklab(val) => Self::Oklab(val.lighten(factor)),
            Self::Oklch(val) => Self::Oklch(val.lighten(factor)),
            Self::Xyz(val) => Self::Xyz(val.lighten(factor)),
            Self::Yxy(val) => Self::Yxy(val.lighten(factor)),
            _ => *self,
        }
    }

    pub fn lighten_fixed(&self, factor: f32) -> Self {
        match self {
            Self::Rgb(val) => Self::Rgb(val.lighten_fixed(factor)),
            Self::Hsl(val) => Self::Hsl(val.lighten_fixed(factor)),
            Self::Hsluv(val) => Self::Hsluv(val.lighten_fixed(factor)),
            Self::Hsv(val) => Self::Hsv(val.lighten_fixed(factor)),
            Self::Hwb(val) => Self::Hwb(val.lighten_fixed(factor)),
            Self::Lab(val) => Self::Lab(val.lighten_fixed(factor)),
            Self::Lch(val) => Self::Lch(val.lighten_fixed(factor)),
            Self::Lchuv(val) => Self::Lchuv(val.lighten_fixed(factor)),
            Self::Luv(val) => Self::Luv(val.lighten_fixed(factor)),
            Self::Okhsl(val) => Self::Okhsl(val.lighten_fixed(factor)),
            Self::Okhsv(val) => Self::Okhsv(val.lighten_fixed(factor)),
            Self::Okhwb(val) => Self::Okhwb(val.lighten_fixed(factor)),
            Self::Oklab(val) => Self::Oklab(val.lighten_fixed(factor)),
            Self::Oklch(val) => Self::Oklch(val.lighten_fixed(factor)),
            Self::Xyz(val) => Self::Xyz(val.lighten_fixed(factor)),
            Self::Yxy(val) => Self::Yxy(val.lighten_fixed(factor)),
            _ => *self,
        }
    }

    pub fn darken(&self, factor: f32) -> Self {
        match self {
            Self::Rgb(val) => Self::Rgb(val.darken(factor)),
            Self::Hsl(val) => Self::Hsl(val.darken(factor)),
            Self::Hsluv(val) => Self::Hsluv(val.darken(factor)),
            Self::Hsv(val) => Self::Hsv(val.darken(factor)),
            Self::Hwb(val) => Self::Hwb(val.darken(factor)),
            Self::Lab(val) => Self::Lab(val.darken(factor)),
            Self::Lch(val) => Self::Lch(val.darken(factor)),
            Self::Lchuv(val) => Self::Lchuv(val.darken(factor)),
            Self::Luv(val) => Self::Luv(val.darken(factor)),
            Self::Okhsl(val) => Self::Okhsl(val.darken(factor)),
            Self::Okhsv(val) => Self::Okhsv(val.darken(factor)),
            Self::Okhwb(val) => Self::Okhwb(val.darken(factor)),
            Self::Oklab(val) => Self::Oklab(val.darken(factor)),
            Self::Oklch(val) => Self::Oklch(val.darken(factor)),
            Self::Xyz(val) => Self::Xyz(val.darken(factor)),
            Self::Yxy(val) => Self::Yxy(val.darken(factor)),
            _ => *self,
        }
    }

    pub fn darken_fixed(&self, factor: f32) -> Self {
        match self {
            Self::Rgb(val) => Self::Rgb(val.darken_fixed(factor)),
            Self::Hsl(val) => Self::Hsl(val.darken_fixed(factor)),
            Self::Hsluv(val) => Self::Hsluv(val.darken_fixed(factor)),
            Self::Hsv(val) => Self::Hsv(val.darken_fixed(factor)),
            Self::Hwb(val) => Self::Hwb(val.darken_fixed(factor)),
            Self::Lab(val) => Self::Lab(val.darken_fixed(factor)),
            Self::Lch(val) => Self::Lch(val.darken_fixed(factor)),
            Self::Lchuv(val) => Self::Lchuv(val.darken_fixed(factor)),
            Self::Luv(val) => Self::Luv(val.darken_fixed(factor)),
            Self::Okhsl(val) => Self::Okhsl(val.darken_fixed(factor)),
            Self::Okhsv(val) => Self::Okhsv(val.darken_fixed(factor)),
            Self::Okhwb(val) => Self::Okhwb(val.darken_fixed(factor)),
            Self::Oklab(val) => Self::Oklab(val.darken_fixed(factor)),
            Self::Oklch(val) => Self::Oklch(val.darken_fixed(factor)),
            Self::Xyz(val) => Self::Xyz(val.darken_fixed(factor)),
            Self::Yxy(val) => Self::Yxy(val.darken_fixed(factor)),
            _ => *self,
        }
    }
}
