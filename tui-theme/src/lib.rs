use std::{
    io::IsTerminal,
    str::FromStr,
    sync::{LazyLock, RwLock},
};

use palette::{
    Darken, FromColor, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Lighten, Luv, Okhsl, Okhsv, Okhwb,
    Oklab, Oklch, Srgb, Xyz, Yxy, rgb::Rgb, white_point::D50,
};
use regex::{Captures, Regex};
use term_color_adapter::ColorSupport;
use terminal_colorsaurus::{ColorPalette, ColorScheme, QueryOptions, color_palette};
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
        let theme = COLOR_PALETTE.read().unwrap().color_scheme();
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

static COLOR_SUPPORT: LazyLock<RwLock<ColorSupport>> =
    LazyLock::new(|| RwLock::new(ColorSupport::TrueColor));

static COLOR_PALETTE: LazyLock<RwLock<ColorPalette>> =
    LazyLock::new(|| RwLock::new(color_palette(QueryOptions::default()).unwrap()));

pub fn load_color_support<T>(stream: &T)
where
    T: IsTerminal,
{
    *COLOR_SUPPORT.write().unwrap() = ColorSupport::detect(stream)
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

impl Color {
    pub fn terminal_foreground() -> Self {
        let fg = &COLOR_PALETTE.read().unwrap().foreground;
        Self::Rgb(Rgb::new(
            fg.r as f32 / 255.,
            fg.g as f32 / 255.,
            fg.b as f32 / 255.,
        ))
    }

    pub fn terminal_background() -> Self {
        let bg = &COLOR_PALETTE.read().unwrap().background;
        Self::Rgb(Rgb::new(
            bg.r as f32 / 255.,
            bg.g as f32 / 255.,
            bg.b as f32 / 255.,
        ))
    }

    pub fn is_compatible(&self) -> bool {
        let color_support = COLOR_SUPPORT.read().unwrap();
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
            | Self::DarkGray => *color_support >= ColorSupport::Ansi16,
            Self::Indexed(index) if *index < 16 => *color_support >= ColorSupport::Ansi16,
            Self::Indexed(_) => *color_support >= ColorSupport::Ansi256,
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
            | Self::Yxy(_) => *color_support >= ColorSupport::TrueColor,
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
        if let Some(adapted) = COLOR_SUPPORT.read().unwrap().adapt(color) {
            adapted.into()
        } else {
            Self::Reset
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
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "reset" => return Ok(Self::Reset),
            "black" => return Ok(Self::Black),
            "red" => return Ok(Self::Red),
            "green" => return Ok(Self::Green),
            "yellow" => return Ok(Self::Green),
            "blue" => return Ok(Self::Blue),
            "magenta" => return Ok(Self::Magenta),
            "cyan" => return Ok(Self::Cyan),
            "gray" | "grey" => return Ok(Self::Gray),
            "darkgray" | "darkgrey" => return Ok(Self::DarkGray),
            "lightred" => return Ok(Self::LightRed),
            "lightgreen" => return Ok(Self::LightGreen),
            "lightyellow" => return Ok(Self::LightYellow),
            "lightblue" => return Ok(Self::LightBlue),
            "lightmagenta" => return Ok(Self::LightMagenta),
            "lightcyan" => return Ok(Self::LightCyan),
            _ => {}
        }

        if let Ok(val) = s.parse::<u8>() {
            return Ok(Self::Indexed(val));
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
        Err(())
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
            Color::Reset => ratatui::style::Color::Reset,
            Color::Black => ratatui::style::Color::Black,
            Color::Red => ratatui::style::Color::Red,
            Color::Green => ratatui::style::Color::Green,
            Color::Yellow => ratatui::style::Color::Yellow,
            Color::Blue => ratatui::style::Color::Blue,
            Color::Magenta => ratatui::style::Color::Magenta,
            Color::Cyan => ratatui::style::Color::Cyan,
            Color::Gray => ratatui::style::Color::Gray,
            Color::DarkGray => ratatui::style::Color::DarkGray,
            Color::LightRed => ratatui::style::Color::LightRed,
            Color::LightGreen => ratatui::style::Color::LightGreen,
            Color::LightYellow => ratatui::style::Color::LightYellow,
            Color::LightBlue => ratatui::style::Color::LightBlue,
            Color::LightMagenta => ratatui::style::Color::LightMagenta,
            Color::LightCyan => ratatui::style::Color::LightCyan,
            Color::White => ratatui::style::Color::White,
            Color::Indexed(idx) => ratatui::style::Color::Indexed(idx),
        }
    }
}

impl From<ratatui::style::Color> for Color {
    fn from(value: ratatui::style::Color) -> Self {
        match value {
            ratatui::style::Color::Reset => Color::Reset,
            ratatui::style::Color::Black => Color::Black,
            ratatui::style::Color::Red => Color::Red,
            ratatui::style::Color::Green => Color::Green,
            ratatui::style::Color::Yellow => Color::Yellow,
            ratatui::style::Color::Blue => Color::Blue,
            ratatui::style::Color::Magenta => Color::Magenta,
            ratatui::style::Color::Cyan => Color::Cyan,
            ratatui::style::Color::Gray => Color::Gray,
            ratatui::style::Color::DarkGray => Color::DarkGray,
            ratatui::style::Color::LightRed => Color::LightRed,
            ratatui::style::Color::LightGreen => Color::LightGreen,
            ratatui::style::Color::LightYellow => Color::LightYellow,
            ratatui::style::Color::LightBlue => Color::LightBlue,
            ratatui::style::Color::LightMagenta => Color::LightMagenta,
            ratatui::style::Color::LightCyan => Color::LightCyan,
            ratatui::style::Color::White => Color::White,
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
            anstyle::Color::Ansi(anstyle::AnsiColor::Black) => Color::Black,
            anstyle::Color::Ansi(anstyle::AnsiColor::Red) => Color::Red,
            anstyle::Color::Ansi(anstyle::AnsiColor::Green) => Color::Green,
            anstyle::Color::Ansi(anstyle::AnsiColor::Yellow) => Color::Yellow,
            anstyle::Color::Ansi(anstyle::AnsiColor::Blue) => Color::Blue,
            anstyle::Color::Ansi(anstyle::AnsiColor::Magenta) => Color::Magenta,
            anstyle::Color::Ansi(anstyle::AnsiColor::Cyan) => Color::Cyan,
            anstyle::Color::Ansi(anstyle::AnsiColor::White) => Color::Gray,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlack) => Color::DarkGray,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed) => Color::LightRed,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen) => Color::LightGreen,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow) => Color::LightYellow,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue) => Color::LightBlue,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta) => Color::LightMagenta,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan) => Color::LightCyan,
            anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite) => Color::White,
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
            Color::Reset => None?,
            Color::Black => anstyle::Color::Ansi(anstyle::AnsiColor::Black),
            Color::Red => anstyle::Color::Ansi(anstyle::AnsiColor::Red),
            Color::Green => anstyle::Color::Ansi(anstyle::AnsiColor::Green),
            Color::Yellow => anstyle::Color::Ansi(anstyle::AnsiColor::Yellow),
            Color::Blue => anstyle::Color::Ansi(anstyle::AnsiColor::Blue),
            Color::Magenta => anstyle::Color::Ansi(anstyle::AnsiColor::Magenta),
            Color::Cyan => anstyle::Color::Ansi(anstyle::AnsiColor::Cyan),
            Color::Gray => anstyle::Color::Ansi(anstyle::AnsiColor::White),
            Color::DarkGray => anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlack),
            Color::LightRed => anstyle::Color::Ansi(anstyle::AnsiColor::BrightRed),
            Color::LightGreen => anstyle::Color::Ansi(anstyle::AnsiColor::BrightGreen),
            Color::LightYellow => anstyle::Color::Ansi(anstyle::AnsiColor::BrightYellow),
            Color::LightBlue => anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue),
            Color::LightMagenta => anstyle::Color::Ansi(anstyle::AnsiColor::BrightMagenta),
            Color::LightCyan => anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan),
            Color::White => anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite),
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
