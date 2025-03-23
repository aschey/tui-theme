use std::{
    cell::RefCell,
    str::FromStr,
    sync::{Arc, LazyLock, OnceLock, RwLock},
};

use palette::{
    FromColor, Hsl, Hsluv, Hsv, Hwb, IntoColor, Lab, Lch, Lchuv, Lighten, LinSrgb, Luv, Okhsl,
    Okhsv, Okhwb, Oklab, Oklch, Srgb, Xyz, Yxy, color_difference::EuclideanDistance,
    convert::FromColorUnclamped, encoding::Linear, rgb::Rgb, white_point::D50,
};
use ratatui::{
    palette::Darken,
    style::{Style, Stylize},
};
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use term_color_support::colors::ColorSupportLevel;
pub use tui_theme_derive::*;

enum ThemeChoice {
    Dark,
    Light,
}

struct AdaptiveTheme<T> {
    dark: T,
    light: T,
    choice: ThemeChoice,
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
}

static COLOR_SUPPORT: LazyLock<RwLock<ColorSupportLevel>> =
    LazyLock::new(|| RwLock::new(ColorSupportLevel::TrueColor));

pub fn load_color_support_stdout() {
    *COLOR_SUPPORT.write().unwrap() = term_color_support::ColorSupport::stdout().level;
}

pub fn load_color_support_stderr() {
    *COLOR_SUPPORT.write().unwrap() = term_color_support::ColorSupport::stderr().level;
}

pub fn adapt_color(color: ratatui::style::Color) -> ratatui::style::Color {
    let color_support = COLOR_SUPPORT.read().unwrap();
    // NO_COLOR should be handled upstream
    match color {
        ratatui::style::Color::Indexed(index) => {
            if *color_support == ColorSupportLevel::Basic && index > 15 {
                ansi256_to_ansi(index)
            } else {
                color
            }
        }
        ratatui::style::Color::Rgb(r, g, b) => {
            if *color_support == ColorSupportLevel::TrueColor {
                color
            } else {
                let ansi256_index = rgb_to_ansi256(r, g, b);
                if *color_support == ColorSupportLevel::Colors256 {
                    ratatui::style::Color::Indexed(ansi256_index)
                } else {
                    ansi256_to_ansi(ansi256_index)
                }
            }
        }
        _ => color,
    }
}

fn ansi256_to_lab(ansi256_index: u8) -> Lab {
    let srgb: Srgb<u8> = ANSI_HEX[ansi256_index as usize].parse().unwrap();
    Lab::from_color(srgb.into_linear())
}

fn ansi256_to_ansi(ansi256_index: u8) -> ratatui::style::Color {
    let reference_lab = ansi256_to_lab(ansi256_index);
    let mut min_distance = f32::MAX;
    let mut closest_ansi = 0u8;
    for i in 0..16u8 {
        let lab = ansi256_to_lab(i);
        let distance = reference_lab.distance(lab);
        if distance < min_distance {
            closest_ansi = i;
            min_distance = distance;
        }
    }
    ratatui::style::Color::Indexed(closest_ansi)
}

fn value_to_color_index(value: u8) -> usize {
    if value < 48 {
        0
    } else if value < 115 {
        1
    } else {
        ((value - 35) / 40) as usize
    }
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let color = Srgb::new(r, g, b);
    let r = value_to_color_index(r);
    let g = value_to_color_index(g);
    let b = value_to_color_index(b);

    let color_index = (36 * r + 6 * g + b) as u8;
    let index_to_color_value: [u8; 5] = [0x0, 0x5f, 0x87, 0xaf, 0xff];

    let cr = index_to_color_value[r];
    let cg = index_to_color_value[g];
    let cb = index_to_color_value[b];

    let average = (r + g + b) / 3;
    let gray_index = if average > 238 {
        23
    } else {
        (average - 3) / 10
    };
    let gray_value = 8 + 10 * gray_index as u8;

    let color2 = Srgb::new(cr, cg, cb);
    let gray2 = Srgb::new(gray_value, gray_value, gray_value);

    let lab_color: Lab = Lab::from_color(color.into_linear());
    let color_distance = lab_color.distance(Lab::from_color(color2.into_linear()));
    let gray_distance = lab_color.distance(Lab::from_color(gray2.into_linear()));
    if color_distance <= gray_distance {
        16 + color_index
    } else {
        232 + gray_index as u8
    }
}

// #[derive(Default, Clone, Copy)]
// pub struct AppTheme {
//     primary: ratatui::style::Color,
//     secondary: ratatui::style::Color,
// }

pub trait SetTheme {
    type Theme;

    fn set_local(&self);

    fn unset_local();

    fn set_global(&self);

    fn current() -> Self::Theme;
}

// impl SetTheme for AppTheme {
//     type Theme = Self;

//     fn set_local(&self) {
//         __LOCAL_APPTHEME.with(|t| *t.borrow_mut() = Some(self.clone()));
//     }

//     fn unset_local() {
//         __LOCAL_APPTHEME.with(|t| *t.borrow_mut() = None);
//     }

//     fn set_global(&self) {
//         *__GLOBAL_APPTHEME.write().unwrap() = self.clone();
//     }

//     fn current() -> Self {
//         __LOCAL_APPTHEME
//             .with(|t| *t.borrow())
//             .unwrap_or_else(|| *__GLOBAL_APPTHEME.read().unwrap())
//     }
// }

// static __GLOBAL_APPTHEME: LazyLock<Arc<RwLock<AppTheme>>> = LazyLock::new(Default::default);

// thread_local! {
//     static __LOCAL_APPTHEME: RefCell<Option<AppTheme>> = Default::default();
// }

// pub trait ThemeStyle<T> {
//     fn primary(self) -> T;
//     fn secondary(self) -> T;

//     fn on_primary(self) -> T;
//     fn on_secondary(self) -> T;
// }

// impl<'a, T, U> ThemeStyle<T> for U
// where
//     U: Stylize<'a, T>,
// {
//     fn primary(self) -> T {
//         let theme = AppTheme::current();
//         self.fg(theme.primary)
//     }

//     fn secondary(self) -> T {
//         let theme = AppTheme::current();
//         self.fg(theme.secondary)
//     }

//     fn on_primary(self) -> T {
//         let theme = AppTheme::current();
//         self.bg(theme.primary)
//     }

//     fn on_secondary(self) -> T {
//         let theme = AppTheme::current();
//         self.bg(theme.secondary)
//     }
// }

// pub trait AppThemeColor {
//     fn primary() -> Self;
//     fn secondary() -> Self;
// }

// impl AppThemeColor for ratatui::style::Color {
//     fn primary() -> Self {
//         AppTheme::current().primary
//     }

//     fn secondary() -> Self {
//         AppTheme::current().secondary
//     }
// }

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
        let tui_color = match value {
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
            Color::LightGreen => todo!(),
            Color::LightYellow => todo!(),
            Color::LightBlue => todo!(),
            Color::LightMagenta => todo!(),
            Color::LightCyan => todo!(),
            Color::White => todo!(),
            Color::Indexed(_) => todo!(),
        };
        adapt_color(tui_color)
    }
}

// #[derive(Theme, Default, Clone)]
// #[variants("a", "b")]
// struct AppTheme2 {
//     #[variants("a", "b")]
//     //#[color(variants("a", "b"))]
//     primary: Color,
//     secondary: Color,
//     //borders: Borders,
//     //borders_focused: Borders,
// }
//
// #[derive(ColorTheme, Default, Clone)]
// #[variants("a", "b")]
// struct ColorTheme2 {
//     #[variants("a", "b")]
//     primary: Color,
//     secondary: Color,
// }
//
// #[derive(SubTheme, Default, Clone)]
// struct BorderTheme {
//     default: Borders,
//     focused: Borders,
// }
//
// struct AppTheme2 {
//     colors: ColorTheme2,
//     borders: BorderTheme,
//     //borders: Borders,
//     //borders_focused: Borders,
// }
//
// fn test() {
//     let a = "a".primary();
//     "a".on_primary();
//     // Borders::primary()
// }

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
