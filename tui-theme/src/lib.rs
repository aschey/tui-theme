use std::{
    error::Error,
    fmt::Display,
    io::IsTerminal,
    str::FromStr,
    sync::{LazyLock, RwLock},
};

use palette::{
    Darken, FromColor, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Lighten, Luv, Okhsl, Okhsv, Okhwb,
    Oklab, Oklch, Srgb, Xyz, Yxy, rgb::Rgb, white_point::D50,
};
use regex::{Captures, Regex};
use terminal_colorsaurus::{ColorPalette, ColorScheme, QueryOptions, color_palette};
use termprofile::TermProfile;
pub use tui_theme_derive::*;

pub enum ThemeChoice {
    Dark,
    Light,
}

pub struct AdaptiveTheme<T> {
    dark: T,
    light: T,
    choice: ThemeChoice,
}

impl<T> AdaptiveTheme<T> {
    pub fn auto(dark: T, light: T) -> Self {
        let theme = COLOR_PALETTE
            .read()
            .unwrap()
            .as_ref()
            .map(|p| p.color_scheme())
            .unwrap_or_default();
        Self::new(
            dark,
            light,
            match theme {
                ColorScheme::Light => ThemeChoice::Light,
                ColorScheme::Dark => ThemeChoice::Dark,
            },
        )
    }

    pub fn new(dark: T, light: T, theme: ThemeChoice) -> Self {
        Self {
            dark,
            light,
            choice: theme,
        }
    }
}

impl<T> SetTheme for AdaptiveTheme<T>
where
    T: SetTheme,
{
    type Theme = T::Theme;

    fn set_global(&self) {
        match self.choice {
            ThemeChoice::Dark => {
                self.dark.set_global();
            }
            ThemeChoice::Light => {
                self.light.set_global();
            }
        }
    }

    fn unset_local() {
        T::unset_local();
    }

    fn set_local(&self) {
        match self.choice {
            ThemeChoice::Dark => {
                self.dark.set_local();
            }
            ThemeChoice::Light => {
                self.light.set_local();
            }
        }
    }

    fn current() -> Self::Theme {
        T::current()
    }

    fn with_theme<F, R>(f: F) -> R
    where
        F: FnOnce(&Self::Theme) -> R,
    {
        T::with_theme(f)
    }
}

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

pub trait SetTheme {
    type Theme;

    fn set_local(&self);

    fn unset_local();

    fn set_global(&self);

    fn current() -> Self::Theme;

    fn with_theme<F, T>(f: F) -> T
    where
        F: FnOnce(&Self::Theme) -> T;
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
        let fg = &COLOR_PALETTE
            .read()
            .unwrap()
            .as_ref()
            .map(|p| p.foreground.clone())
            .unwrap_or_default();
        Self::Rgb(Rgb::new(
            fg.r as f32 / 255.,
            fg.g as f32 / 255.,
            fg.b as f32 / 255.,
        ))
    }

    pub fn terminal_background() -> Self {
        let bg = &COLOR_PALETTE
            .read()
            .unwrap()
            .as_ref()
            .map(|p| p.background.clone())
            .unwrap_or_default();
        Self::Rgb(Rgb::new(
            bg.r as f32 / 255.,
            bg.g as f32 / 255.,
            bg.b as f32 / 255.,
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

static SEP: &str = r"\s*(?:,|\s+)\s*";
static DIGITS: &str = r"\d{1,3}";
static DEC: &str = r"(?:\.\d+)?";
static PCT: &str = r"\d{1,3}(?:\.\d+)?%?";

struct Bounds<T> {
    min: T,
    max: T,
}

impl<T> Bounds<T> {
    fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

fn parse_capture(
    i: usize,
    bounds: impl Into<Option<Bounds<f32>>>,
    captures: &Captures,
) -> Option<f32> {
    let bounds = bounds.into();
    let s = captures.get(i).unwrap().as_str();
    if let Some(bounds) = bounds {
        if s.ends_with('%') {
            let s = s.trim_end_matches('%');
            let mut val: f32 = s.parse().unwrap();
            val /= 100.0;
            return Some(val * (bounds.max - bounds.min) + bounds.min);
        }
    }
    s.parse().ok()
}

impl Color {
    fn parse_hex(s: &str) -> Option<Self> {
        let re = Regex::new(r"#[a-fA-F0-9]{6}").unwrap();
        if re.is_match(s) {
            let rgb: Srgb<u8> = s.parse().unwrap();
            Some(Self::Rgb(rgb.into()))
        } else {
            None
        }
    }

    fn parse_rgb(s: &str) -> Option<Self> {
        let re = Regex::new(&format!(
            "rgb\\(({DIGITS}){SEP}({DIGITS}){SEP}({DIGITS})\\)"
        ))
        .unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Rgb(Rgb::new(
                parse_capture(1, None, &captures)? / 255.0,
                parse_capture(2, None, &captures)? / 255.0,
                parse_capture(3, None, &captures)? / 255.0,
            )))
        })
    }

    fn parse_hsl(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("hsl\\(({DIGITS}){SEP}({PCT}){SEP}({PCT})\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Hsl(Hsl::new(
                parse_capture(1, None, &captures)?,
                parse_capture(
                    2,
                    Bounds::new(Hsl::<Srgb>::min_saturation(), Hsl::<Srgb>::max_saturation()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Hsl::<Srgb>::min_lightness(), Hsl::<Srgb>::max_lightness()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_hsluv(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("hsluv\\(({DIGITS}){SEP}({PCT}){SEP}({PCT})\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Hsluv(Hsluv::new(
                parse_capture(1, None, &captures)?,
                parse_capture(
                    2,
                    Bounds::new(
                        Hsluv::<Srgb>::min_saturation(),
                        Hsluv::<Srgb>::max_saturation(),
                    ),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Hsluv::<Srgb>::min_l(), Hsluv::<Srgb>::max_l()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_hwb(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("hwb\\(({DIGITS}){SEP}({PCT}){SEP}({PCT})\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Hwb(Hwb::new(
                parse_capture(1, None, &captures)?,
                parse_capture(
                    2,
                    Bounds::new(Hwb::<Srgb>::min_whiteness(), Hwb::<Srgb>::max_whiteness()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Hwb::<Srgb>::min_blackness(), Hwb::<Srgb>::max_blackness()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_lab(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("lab\\(({PCT}){SEP}(-?{PCT}){SEP}(-?{PCT})\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Lab(Lab::new(
                parse_capture(
                    1,
                    Bounds::new(Lab::<Srgb>::min_l(), Lab::<Srgb>::max_l()),
                    &captures,
                )?,
                parse_capture(
                    2,
                    Bounds::new(Lab::<Srgb>::min_a(), Lab::<Srgb>::max_a()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Lab::<Srgb>::min_b(), Lab::<Srgb>::max_b()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_lch(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("lch\\({PCT}{SEP}{PCT}{SEP}{DIGITS}{DEC}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Lch(Lch::new(
                parse_capture(
                    1,
                    Bounds::new(Lch::<Srgb>::min_l(), Lch::<Srgb>::max_l()),
                    &captures,
                )?,
                parse_capture(
                    2,
                    Bounds::new(Lch::<Srgb>::min_chroma(), Lch::<Srgb>::max_chroma()),
                    &captures,
                )?,
                parse_capture(3, None, &captures)?,
            )))
        })
    }

    fn parse_lchuv(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("lchuv\\({PCT}{SEP}{PCT}{SEP}{DIGITS}{DEC}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Lchuv(Lchuv::new(
                parse_capture(
                    1,
                    Bounds::new(Lchuv::<Srgb>::min_l(), Lch::<Srgb>::max_l()),
                    &captures,
                )?,
                parse_capture(
                    2,
                    Bounds::new(Lchuv::<Srgb>::min_chroma(), Lch::<Srgb>::max_chroma()),
                    &captures,
                )?,
                parse_capture(3, None, &captures)?,
            )))
        })
    }

    fn parse_luv(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("luv\\({PCT}{SEP}-?{PCT}{SEP}-?{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Luv(Luv::new(
                parse_capture(
                    1,
                    Bounds::new(Luv::<Srgb>::min_l(), Lch::<Srgb>::max_l()),
                    &captures,
                )?,
                parse_capture(
                    2,
                    Bounds::new(Luv::<Srgb>::min_u(), Luv::<Srgb>::max_u()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Luv::<Srgb>::min_v(), Luv::<Srgb>::max_v()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_okhsl(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("okhsl\\({DIGITS}{DEC}{SEP}{PCT}{SEP}{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Okhsl(Okhsl::new(
                parse_capture(1, None, &captures)?,
                parse_capture(
                    2,
                    Bounds::new(Okhsl::min_saturation(), Okhsl::max_saturation()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Okhsl::min_lightness(), Okhsl::max_lightness()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_okhsv(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("okhsv\\({DIGITS}{DEC}{SEP}{PCT}{SEP}{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Okhsv(Okhsv::new(
                parse_capture(1, None, &captures)?,
                parse_capture(
                    2,
                    Bounds::new(Okhsv::min_saturation(), Okhsl::max_saturation()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Okhsv::min_value(), Okhsv::max_value()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_okhwb(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("okhwb\\({DIGITS}{DEC}{SEP}{PCT}{SEP}{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Okhwb(Okhwb::new(
                parse_capture(1, None, &captures)?,
                parse_capture(
                    2,
                    Bounds::new(Okhwb::min_whiteness(), Okhwb::max_whiteness()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Okhwb::min_blackness(), Okhwb::max_blackness()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_oklab(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("oklab\\({PCT}{SEP}-?{PCT}{SEP}-?{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Oklab(Oklab::new(
                parse_capture(1, Bounds::new(Oklab::min_l(), Oklab::max_l()), &captures)?,
                parse_capture(2, None, &captures)?,
                parse_capture(3, None, &captures)?,
            )))
        })
    }

    fn parse_oklch(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("oklch\\({PCT}{SEP}{PCT}{SEP}{DIGITS}{DEC}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Oklch(Oklch::new(
                parse_capture(1, Bounds::new(Oklch::min_l(), Oklch::max_l()), &captures)?,
                parse_capture(2, None, &captures)?,
                parse_capture(3, None, &captures)?,
            )))
        })
    }

    fn parse_xyz(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("xyz\\({PCT}{SEP}{PCT}{SEP}{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Xyz(Xyz::new(
                parse_capture(
                    1,
                    Bounds::new(Xyz::<D50>::min_x(), Xyz::<D50>::max_x()),
                    &captures,
                )?,
                parse_capture(
                    2,
                    Bounds::new(Xyz::<D50>::min_y(), Xyz::<D50>::max_y()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Xyz::<D50>::min_z(), Xyz::<D50>::max_z()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_yxy(s: &str) -> Option<Self> {
        let re = Regex::new(&format!("yxy\\({PCT}{SEP}{PCT}{SEP}{PCT}\\)")).unwrap();
        re.captures(s).and_then(|captures| {
            Some(Self::Yxy(Yxy::new(
                parse_capture(
                    1,
                    Bounds::new(Yxy::<D50>::min_x(), Yxy::<D50>::max_x()),
                    &captures,
                )?,
                parse_capture(
                    2,
                    Bounds::new(Yxy::<D50>::min_y(), Yxy::<D50>::max_y()),
                    &captures,
                )?,
                parse_capture(
                    3,
                    Bounds::new(Yxy::<D50>::min_luma(), Yxy::<D50>::max_luma()),
                    &captures,
                )?,
            )))
        })
    }

    fn parse_named(s: &str) -> Option<Self> {
        match s
            .to_ascii_lowercase()
            .replace("-", "")
            .replace("_", "")
            .trim()
        {
            "ansireset" => Some(Self::AnsiReset),
            "ansiblack" => Some(Self::AnsiBlack),
            "ansired" => Some(Self::AnsiRed),
            "ansigreen" => Some(Self::AnsiGreen),
            "ansiyellow" => Some(Self::AnsiGreen),
            "ansiblue" => Some(Self::AnsiBlue),
            "ansimagenta" => Some(Self::AnsiMagenta),
            "ansicyan" => Some(Self::AnsiCyan),
            "ansigray" | "ansigrey" => Some(Self::AnsiGray),
            "ansidarkgray" | "ansidarkgrey" => Some(Self::AnsiDarkGray),
            "ansilightred" => Some(Self::AnsiLightRed),
            "ansilightgreen" => Some(Self::AnsiLightGreen),
            "ansilightyellow" => Some(Self::AnsiLightYellow),
            "ansilightblue" => Some(Self::AnsiLightBlue),
            "ansilightmagenta" => Some(Self::AnsiLightMagenta),
            "ansilightcyan" => Some(Self::AnsiLightCyan),
            "aliceblue" => Some(Self::from_hex("#f0f8ff")),
            "antiquewhite" => Some(Self::from_hex("#faebd7")),
            "aqua" => Some(Self::from_hex("#00ffff")),
            "aquamarine" => Some(Self::from_hex("#7fffd4")),
            "azure" => Some(Self::from_hex("#f0ffff")),
            "beige" => Some(Self::from_hex("#f5f5dc")),
            "bisque" => Some(Self::from_hex("#ffe4c4")),
            "black" => Some(Self::from_hex("#000000")),
            "blanchedalmond" => Some(Self::from_hex("#ffebcd")),
            "blue" => Some(Self::from_hex("#0000ff")),
            "blueviolet" => Some(Self::from_hex("#8a2be2")),
            "brown" => Some(Self::from_hex("#a52a2a")),
            "burlywood" => Some(Self::from_hex("#deb887")),
            "cadetblue" => Some(Self::from_hex("#5f9ea0")),
            "chartreuse" => Some(Self::from_hex("#7fff00")),
            "chocolate" => Some(Self::from_hex("#d2691e")),
            "coral" => Some(Self::from_hex("#ff7f50")),
            "cornflowerblue" => Some(Self::from_hex("#6495ed")),
            "cornsilk" => Some(Self::from_hex("#fff8dc")),
            "crimson" => Some(Self::from_hex("#dc143c")),
            "cyan" => Some(Self::from_hex("#00ffff")),
            "darkblue" => Some(Self::from_hex("#00008b")),
            "darkcyan" => Some(Self::from_hex("#008b8b")),
            "darkgoldenrod" => Some(Self::from_hex("#b8860b")),
            "darkgray" | "darkgrey" => Some(Self::from_hex("#a9a9a9")),
            "darkgreen" => Some(Self::from_hex("#006400")),
            "darkkhaki" => Some(Self::from_hex("#bdb76b")),
            "darkmagenta" => Some(Self::from_hex("#8b008b")),
            "darkolivegreen" => Some(Self::from_hex("#556b2f")),
            "darkorange" => Some(Self::from_hex("#ff8c00")),
            "darkorchid" => Some(Self::from_hex("#9932cc")),
            "darkred" => Some(Self::from_hex("#8b0000")),
            "darksalmon" => Some(Self::from_hex("#e9967a")),
            "darkseagreen" => Some(Self::from_hex("#8fbc8f")),
            "darkslateblue" => Some(Self::from_hex("#483d8b")),
            "darkslategray" | "darkslategrey" => Some(Self::from_hex("#2f4f4f")),
            "darkturquoise" => Some(Self::from_hex("#00ced1")),
            "darkviolet" => Some(Self::from_hex("#9400d3")),
            "deeppink" => Some(Self::from_hex("#ff1493")),
            "deepskyblue" => Some(Self::from_hex("#00bfff")),
            "dimgrey" | "dimgray" => Some(Self::from_hex("#696969")),
            "dodgerblue" => Some(Self::from_hex("#1e90ff")),
            "firebrick" => Some(Self::from_hex("#b22222")),
            "floralwhite" => Some(Self::from_hex("#fffaf0")),
            "forestgreen" => Some(Self::from_hex("#228b22")),
            "fuchsia" => Some(Self::from_hex("#ff00ff")),
            "gainsboro" => Some(Self::from_hex("#dcdcdc")),
            "ghostwhite" => Some(Self::from_hex("#f8f8ff")),
            "gold" => Some(Self::from_hex("#ffd700")),
            "goldenrod" => Some(Self::from_hex("#daa520")),
            "gray" | "grey" => Some(Self::from_hex("#808080")),
            "green" => Some(Self::from_hex("#008000")),
            "greenyellow" => Some(Self::from_hex("#adff2f")),
            "honeydew" => Some(Self::from_hex("#f0fff0")),
            "hotpink" => Some(Self::from_hex("#ff69b4")),
            "indianred" => Some(Self::from_hex("#cd5c5c")),
            "indigo" => Some(Self::from_hex("#4b0082")),
            "ivory" => Some(Self::from_hex("#fffff0")),
            "khaki" => Some(Self::from_hex("#f0e68c")),
            "lavender" => Some(Self::from_hex("#e6e6fa")),
            "lavenderblush" => Some(Self::from_hex("#fff0f5")),
            "lawngreen" => Some(Self::from_hex("#7cfc00")),
            "lemonchiffon" => Some(Self::from_hex("#fffacd")),
            "lightblue" => Some(Self::from_hex("#add8e6")),
            "lightcoral" => Some(Self::from_hex("#f08080")),
            "lightcyan" => Some(Self::from_hex("#e0ffff")),
            "lightgoldenrodyellow" | "lightgoldenrod" => Some(Self::from_hex("#fafad2")),
            "lightgray" | "lightgrey" => Some(Self::from_hex("#d3d3d3")),
            "lightgreen" => Some(Self::from_hex("#90ee90")),
            "lightpink" => Some(Self::from_hex("#ffb6c1")),
            "lightsalmon" => Some(Self::from_hex("#ffa07a")),
            "lightseagreen" => Some(Self::from_hex("#20b2aa")),
            "lightskyblue" => Some(Self::from_hex("#87cefa")),
            "lightslategray" | "lightslategrey" => Some(Self::from_hex("#778899")),
            "lightsteelblue" => Some(Self::from_hex("#b0c4de")),
            "lightyellow" => Some(Self::from_hex("#ffffe0")),
            "lime" => Some(Self::from_hex("#00ff00")),
            "limegreen" => Some(Self::from_hex("#32cd32")),
            "linen" => Some(Self::from_hex("#faf0e6")),
            "magenta" => Some(Self::from_hex("#ff00ff")),
            "maroon" => Some(Self::from_hex("#800000")),
            "mediumaquamarine" => Some(Self::from_hex("#66cdaa")),
            "mediumblue" => Some(Self::from_hex("#0000cd")),
            "mediumorchid" => Some(Self::from_hex("#ba55d3")),
            "mediumpurple" => Some(Self::from_hex("#9370db")),
            "mediumseagreen" => Some(Self::from_hex("#3cb371")),
            "mediumslateblue" => Some(Self::from_hex("#7b68ee")),
            "mediumspringgreen" => Some(Self::from_hex("#00fa9a")),
            "mediumturquoise" => Some(Self::from_hex("#48d1cc")),
            "mediumvioletred" => Some(Self::from_hex("#c71585")),
            "midnightblue" => Some(Self::from_hex("#191970")),
            "mintcream" => Some(Self::from_hex("#f5fffa")),
            "mistyrose" => Some(Self::from_hex("#ffe4e1")),
            "moccasin" => Some(Self::from_hex("#ffe4b5")),
            "navajowhite" => Some(Self::from_hex("#ffdead")),
            "navy" => Some(Self::from_hex("#000080")),
            "oldlace" => Some(Self::from_hex("#fdf5e6")),
            "olive" => Some(Self::from_hex("#808000")),
            "olivedrab" => Some(Self::from_hex("#6b8e23")),
            "orange" => Some(Self::from_hex("#ffa500")),
            "orangered" => Some(Self::from_hex("#ff4500")),
            "orchid" => Some(Self::from_hex("#da70d6")),
            "palegoldenrod" => Some(Self::from_hex("#eee8aa")),
            "palegreen" => Some(Self::from_hex("#98fb98")),
            "paleturquoise" => Some(Self::from_hex("#afeeee")),
            "palevioletred" => Some(Self::from_hex("#db7093")),
            "papayawhip" => Some(Self::from_hex("#ffefd5")),
            "peachpuff" => Some(Self::from_hex("#ffdab9")),
            "peru" => Some(Self::from_hex("#cd853f")),
            "pink" => Some(Self::from_hex("#ffc0cb")),
            "plum" => Some(Self::from_hex("#dda0dd")),
            "powderblue" => Some(Self::from_hex("#b0e0e6")),
            "purple" => Some(Self::from_hex("#800080")),
            "rebeccapurple" => Some(Self::from_hex("#663399")),
            "red" => Some(Self::from_hex("#ff0000")),
            "rosybrown" => Some(Self::from_hex("#bc8f8f")),
            "royalblue" => Some(Self::from_hex("#4169e1")),
            "saddlebrown" => Some(Self::from_hex("#8b4513")),
            "salmon" => Some(Self::from_hex("#fa8072")),
            "sandybrown" => Some(Self::from_hex("#f4a460")),
            "seagreen" => Some(Self::from_hex("#2e8b57")),
            "seashell" => Some(Self::from_hex("#fff5ee")),
            "sienna" => Some(Self::from_hex("#a0522d")),
            "silver" => Some(Self::from_hex("#c0c0c0")),
            "skyblue" => Some(Self::from_hex("#87ceeb")),
            "slateblue" => Some(Self::from_hex("#6a5acd")),
            "slategray" | "slategrey" => Some(Self::from_hex("#708090")),
            "snow" => Some(Self::from_hex("#fffafa")),
            "springgreen" => Some(Self::from_hex("#00ff7f")),
            "steelblue" => Some(Self::from_hex("#4682b4")),
            "tan" => Some(Self::from_hex("#d2b48c")),
            "teal" => Some(Self::from_hex("#008080")),
            "thistle" => Some(Self::from_hex("#d8bfd8")),
            "tomato" => Some(Self::from_hex("#ff6347")),
            "turquoise" => Some(Self::from_hex("#40e0d0")),
            "violet" => Some(Self::from_hex("#ee82ee")),
            "wheat" => Some(Self::from_hex("#f5deb3")),
            "white" => Some(Self::from_hex("#ffffff")),
            "whitesmoke" => Some(Self::from_hex("#f5f5f5")),
            "yellow" => Some(Self::from_hex("#ffff00")),
            "yellowgreen" => Some(Self::from_hex("#9acd32")),
            _ => None,
        }
    }

    fn from_hex(hex: &str) -> Self {
        Self::parse_hex(hex).unwrap()
    }
}

#[derive(Debug)]
pub struct InvalidColor;

impl Error for InvalidColor {}

impl Display for InvalidColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid color")
    }
}

impl FromStr for Color {
    type Err = InvalidColor;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<u8>() {
            return Ok(Self::Indexed(val));
        }

        if let Some(val) = Self::parse_named(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_hex(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_rgb(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_hsl(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_hsluv(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_hwb(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_lab(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_lch(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_lchuv(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_luv(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_okhsl(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_okhsv(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_okhwb(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_oklab(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_oklch(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_xyz(s) {
            return Ok(val);
        }
        if let Some(val) = Self::parse_yxy(s) {
            return Ok(val);
        }
        Err(InvalidColor)
    }
}

impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(val) => val.into(),
            Color::Hsl(val) => Rgb::from_color(val).into(),
            Color::Hsluv(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Hsv(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Hwb(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Lab(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Lch(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Lchuv(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Luv(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Okhsl(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Okhsv(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Okhwb(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Oklab(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Oklch(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Xyz(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Yxy(val) => Rgb::<palette::encoding::Srgb, _>::from_color(val).into(),
            Color::AnsiReset => ratatui::style::Color::Reset,
            Color::AnsiBlack => ratatui::style::Color::Black,
            Color::AnsiRed => ratatui::style::Color::Red,
            Color::AnsiGreen => ratatui::style::Color::Green,
            Color::AnsiYellow => ratatui::style::Color::Yellow,
            Color::AnsiBlue => ratatui::style::Color::Blue,
            Color::AnsiMagenta => ratatui::style::Color::Magenta,
            Color::AnsiCyan => ratatui::style::Color::Cyan,
            Color::AnsiGray => ratatui::style::Color::Gray,
            Color::AnsiDarkGray => ratatui::style::Color::DarkGray,
            Color::AnsiLightRed => ratatui::style::Color::LightRed,
            Color::AnsiLightGreen => ratatui::style::Color::LightGreen,
            Color::AnsiLightYellow => ratatui::style::Color::LightYellow,
            Color::AnsiLightBlue => ratatui::style::Color::LightBlue,
            Color::AnsiLightMagenta => ratatui::style::Color::LightMagenta,
            Color::AnsiLightCyan => ratatui::style::Color::LightCyan,
            Color::AnsiWhite => ratatui::style::Color::White,
            Color::Indexed(idx) => ratatui::style::Color::Indexed(idx),
        }
    }
}

impl From<ratatui::style::Color> for Color {
    fn from(value: ratatui::style::Color) -> Self {
        match value {
            ratatui::style::Color::Reset => Color::AnsiReset,
            ratatui::style::Color::Black => Color::AnsiBlack,
            ratatui::style::Color::Red => Color::AnsiRed,
            ratatui::style::Color::Green => Color::AnsiGreen,
            ratatui::style::Color::Yellow => Color::AnsiYellow,
            ratatui::style::Color::Blue => Color::AnsiBlue,
            ratatui::style::Color::Magenta => Color::AnsiMagenta,
            ratatui::style::Color::Cyan => Color::AnsiCyan,
            ratatui::style::Color::Gray => Color::AnsiGray,
            ratatui::style::Color::DarkGray => Color::AnsiDarkGray,
            ratatui::style::Color::LightRed => Color::AnsiLightRed,
            ratatui::style::Color::LightGreen => Color::AnsiLightGreen,
            ratatui::style::Color::LightYellow => Color::AnsiLightYellow,
            ratatui::style::Color::LightBlue => Color::AnsiLightBlue,
            ratatui::style::Color::LightMagenta => Color::AnsiLightMagenta,
            ratatui::style::Color::LightCyan => Color::AnsiLightCyan,
            ratatui::style::Color::White => Color::AnsiWhite,
            ratatui::style::Color::Rgb(r, g, b) => {
                Color::Rgb(Rgb::new(r as f32 / 255., g as f32 / 255., b as f32 / 255.))
            }
            ratatui::style::Color::Indexed(idx) => Color::Indexed(idx),
        }
    }
}

impl From<anstyle::Color> for Color {
    fn from(value: anstyle::Color) -> Self {
        match value {
            anstyle::Color::Ansi(anstyle::AnsiColor::Black) => Color::AnsiBlack,
            anstyle::Color::Ansi(anstyle::AnsiColor::Red) => Color::AnsiRed,
            anstyle::Color::Ansi(anstyle::AnsiColor::Green) => Color::AnsiGreen,
            anstyle::Color::Ansi(anstyle::AnsiColor::Yellow) => Color::AnsiYellow,
            anstyle::Color::Ansi(anstyle::AnsiColor::Blue) => Color::AnsiBlue,
            anstyle::Color::Ansi(anstyle::AnsiColor::Magenta) => Color::AnsiMagenta,
            anstyle::Color::Ansi(anstyle::AnsiColor::Cyan) => Color::AnsiCyan,
            anstyle::Color::Ansi(anstyle::AnsiColor::White) => Color::AnsiGray,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlack) => Color::AnsiDarkGray,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed) => Color::AnsiLightRed,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen) => Color::AnsiLightGreen,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow) => Color::AnsiLightYellow,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue) => Color::AnsiLightBlue,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta) => Color::AnsiLightMagenta,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan) => Color::AnsiLightCyan,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite) => Color::AnsiWhite,
            anstyle::Color::Ansi256(anstyle::Ansi256Color(index)) => Color::Indexed(index),
            anstyle::Color::Rgb(rgb_color) => Color::Rgb(Rgb::new(
                rgb_color.r() as f32 / 255.,
                rgb_color.g() as f32 / 255.,
                rgb_color.b() as f32 / 255.,
            )),
        }
    }
}

impl From<Color> for Option<anstyle::Color> {
    fn from(value: Color) -> Self {
        Some(match value {
            Color::AnsiReset => None?,
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
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Hsv(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Hwb(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Lab(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Lch(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Lchuv(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Luv(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Okhsl(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Okhsv(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Okhwb(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Oklab(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Oklch(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Xyz(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
            Color::Yxy(val) => {
                palette_to_anstyle(Rgb::<palette::encoding::Srgb, _>::from_color(val))
            }
        })
    }
}

fn palette_to_anstyle(rgb_color: Rgb) -> anstyle::Color {
    anstyle::Color::Rgb(anstyle::RgbColor(
        (rgb_color.red * 255.) as u8,
        (rgb_color.green * 255.) as u8,
        (rgb_color.blue * 255.) as u8,
    ))
}
