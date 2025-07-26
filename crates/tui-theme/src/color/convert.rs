use palette::rgb::Rgb;
use palette::{
    FromColor, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Luv, Okhsl, Okhsv, Okhwb, Oklab, Oklch, Xyz,
    Yxy,
};
use termprofile::TermProfile;

use super::{Color, indexed_to_rgb, profile};

impl Color {
    pub fn to_rgb_fg(self) -> Rgb {
        self.to_rgb(true)
    }

    pub fn to_rgb_bg(self) -> Rgb {
        self.to_rgb(false)
    }

    pub fn to_hex_fg(self) -> String {
        self.to_hex(true)
    }

    pub fn to_hex_bg(self) -> String {
        self.to_hex(false)
    }

    pub fn to_hex(self, is_fg: bool) -> String {
        let rgb = self.to_rgb(is_fg);
        format!(
            "#{:02x}{:02x}{:02x}",
            (rgb.red * 255.0).round() as u8,
            (rgb.green * 255.0).round() as u8,
            (rgb.blue * 255.0).round() as u8
        )
        .to_uppercase()
    }

    fn to_rgb(self, is_fg: bool) -> Rgb {
        match self {
            Self::Rgb(val) => val,
            Self::Reset => {
                if is_fg {
                    Self::terminal_foreground().to_rgb_fg()
                } else {
                    Self::terminal_background().to_rgb_bg()
                }
            }
            Self::Black => indexed_to_rgb(0),
            Self::Red => indexed_to_rgb(1),
            Self::Green => indexed_to_rgb(2),
            Self::Yellow => indexed_to_rgb(3),
            Self::Blue => indexed_to_rgb(4),
            Self::Magenta => indexed_to_rgb(5),
            Self::Cyan => indexed_to_rgb(6),
            Self::Gray => indexed_to_rgb(7),
            Self::DarkGray => indexed_to_rgb(8),
            Self::LightRed => indexed_to_rgb(9),
            Self::LightGreen => indexed_to_rgb(10),
            Self::LightYellow => indexed_to_rgb(11),
            Self::LightBlue => indexed_to_rgb(12),
            Self::LightMagenta => indexed_to_rgb(13),
            Self::LightCyan => indexed_to_rgb(14),
            Self::White => indexed_to_rgb(15),
            Self::Indexed(idx) => indexed_to_rgb(idx),
        }
    }

    pub fn into_adaptive(self) -> Self {
        if self.is_compatible() {
            return self;
        }
        self.into_anstyle().map(Into::into).unwrap_or(Self::Reset)
    }

    fn into_anstyle(self) -> Option<anstyle::Color> {
        let value = match self {
            Color::Reset => return None,
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
        };
        let profile = profile().unwrap_or(TermProfile::TrueColor);
        profile.adapt_color(value)
    }
}

fn palette_to_anstyle<T>(val: T) -> anstyle::Color
where
    Rgb<palette::encoding::Srgb>: FromColor<T>,
{
    rgb_to_anstyle(Rgb::<::palette::encoding::Srgb, _>::from_color(val))
}

fn rgb_to_anstyle(rgb_color: Rgb) -> anstyle::Color {
    anstyle::Color::Rgb(anstyle::RgbColor(
        (rgb_color.red * 255.) as u8,
        (rgb_color.green * 255.) as u8,
        (rgb_color.blue * 255.) as u8,
    ))
}

impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        match value.into_adaptive() {
            Color::Rgb(val) => val.into(),
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
        value.into_anstyle()
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

macro_rules! from_color {
    ($type:ident) => {
        impl From<$type> for Color {
            fn from(value: $type) -> Self {
                Self::Rgb(Rgb::from_color(value))
            }
        }
    };
}
impl From<u8> for Color {
    fn from(value: u8) -> Self {
        Self::Indexed(value)
    }
}

from_color!(Hsl);
from_color!(Hsluv);
from_color!(Hsv);
from_color!(Hwb);
from_color!(Lab);
from_color!(Lch);
from_color!(Lchuv);
from_color!(Luv);
from_color!(Okhsl);
from_color!(Okhsv);
from_color!(Okhwb);
from_color!(Oklab);
from_color!(Oklch);
from_color!(Xyz);
from_color!(Yxy);
