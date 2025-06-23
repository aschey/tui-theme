use ::palette::{
    Darken, FromColor, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Lighten, Luv, Okhsl, Okhsv, Okhwb,
    Oklab, Oklch, Srgb, Xyz, Yxy, rgb::Rgb, white_point::D50,
};
use regex::{Captures, Regex};
use std::{
    error::Error,
    fmt::Display,
    io::IsTerminal,
    str::FromStr,
    sync::{LazyLock, RwLock},
};
use terminal_colorsaurus::{ColorPalette, ColorScheme, QueryOptions, color_palette};
use termprofile::TermProfile;
pub use tui_theme_derive::*;

pub mod palette;
mod theme;

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

static HEX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#[a-fA-F0-9]{6};?$").unwrap());

static RGB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^rgb\\(({DIGITS}){SEP}({DIGITS}){SEP}({DIGITS})\\);?$"
    ))
    .unwrap()
});

static HSL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^hsl\\(({DIGITS}){SEP}({PCT}){SEP}({PCT})\\);?$")).unwrap()
});

static HSLUV_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^hsluv\\(({DIGITS}){SEP}({PCT}){SEP}({PCT})\\);?$"
    ))
    .unwrap()
});

static HWB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^hwb\\(({DIGITS}){SEP}({PCT}){SEP}({PCT});?$\\)")).unwrap()
});

static LAB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^lab\\(({PCT}){SEP}(-?{PCT}){SEP}(-?{PCT})\\);?$")).unwrap()
});

static LCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^lch\\({PCT}{SEP}{PCT}{SEP}{DIGITS}{DEC}\\);?$")).unwrap()
});

static LCHUV_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "lchuv\\(({PCT}){SEP}({PCT}){SEP}({DIGITS}{DEC})\\);?$"
    ))
    .unwrap()
});

static LUV_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^luv\\(({PCT}{SEP})(-?{PCT}){SEP}(-?{PCT})\\);?$")).unwrap()
});

static OKHSL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^okhsl\\(({DIGITS}{DEC}){SEP}({PCT}){SEP}({PCT})\\);?$"
    ))
    .unwrap()
});

static OKHSV_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^okhsv\\(({DIGITS}{DEC}){SEP}({PCT}){SEP}({PCT})\\);?$"
    ))
    .unwrap()
});

static OKHWB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^okhwb\\(({DIGITS}{DEC}){SEP}({PCT}){SEP}({PCT})\\);?$"
    ))
    .unwrap()
});

static OKLAB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^oklab\\(({PCT}){SEP}(-?{PCT}){SEP}(-?{PCT})\\);?$"
    ))
    .unwrap()
});

static OKLCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^oklch\\(({PCT}){SEP}({PCT}){SEP}({DIGITS}{DEC})\\);?$"
    ))
    .unwrap()
});

static XYZ_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!("^xyz\\(({PCT}){SEP}({PCT}){SEP}({PCT})\\);?$")).unwrap());

static YXY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!("^yxy\\(({PCT}){SEP}({PCT}){SEP}({PCT})\\);?$")).unwrap());

impl Color {
    fn parse_hex(s: &str) -> Option<Self> {
        if HEX_RE.is_match(s) {
            let rgb: Srgb<u8> = s.parse().unwrap();
            Some(Self::Rgb(rgb.into()))
        } else {
            None
        }
    }

    fn parse_rgb(s: &str) -> Option<Self> {
        RGB_RE.captures(s).and_then(|captures| {
            Some(Self::Rgb(Rgb::new(
                parse_capture(1, None, &captures)? / 255.0,
                parse_capture(2, None, &captures)? / 255.0,
                parse_capture(3, None, &captures)? / 255.0,
            )))
        })
    }

    fn parse_hsl(s: &str) -> Option<Self> {
        HSL_RE.captures(s).and_then(|captures| {
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
        HSLUV_RE.captures(s).and_then(|captures| {
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
        HWB_RE.captures(s).and_then(|captures| {
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
        LAB_RE.captures(s).and_then(|captures| {
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
        LCH_RE.captures(s).and_then(|captures| {
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
        LCHUV_RE.captures(s).and_then(|captures| {
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
        LUV_RE.captures(s).and_then(|captures| {
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
        OKHSL_RE.captures(s).and_then(|captures| {
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
        OKHSV_RE.captures(s).and_then(|captures| {
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
        OKHWB_RE.captures(s).and_then(|captures| {
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
        OKLAB_RE.captures(s).and_then(|captures| {
            Some(Self::Oklab(Oklab::new(
                parse_capture(1, Bounds::new(Oklab::min_l(), Oklab::max_l()), &captures)?,
                parse_capture(2, None, &captures)?,
                parse_capture(3, None, &captures)?,
            )))
        })
    }

    fn parse_oklch(s: &str) -> Option<Self> {
        OKLCH_RE.captures(s).and_then(|captures| {
            Some(Self::Oklch(Oklch::new(
                parse_capture(1, Bounds::new(Oklch::min_l(), Oklch::max_l()), &captures)?,
                parse_capture(2, None, &captures)?,
                parse_capture(3, None, &captures)?,
            )))
        })
    }

    fn parse_xyz(s: &str) -> Option<Self> {
        XYZ_RE.captures(s).and_then(|captures| {
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
        YXY_RE.captures(s).and_then(|captures| {
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
            s => ::palette::named::from_str(s).map(|c| Self::Rgb(c.into())),
        }
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
        let s = s.trim();
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
            Color::Hsluv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Hsv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Hwb(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Lab(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Lch(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Lchuv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Luv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Okhsl(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Okhsv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Okhwb(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Oklab(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Oklch(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Xyz(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
            Color::Yxy(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val).into(),
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
