use palette::rgb::Rgb;
use palette::{
    FromColor, Hsl, Hsluv, Hsv, Hwb, Lab, Lch, Lchuv, Luv, Okhsl, Okhsv, Okhwb, Oklab, Oklch, Xyz,
    Yxy,
};
use termprofile::TermProfile;

use super::{
    ANSI_HEX, Color, indexed_to_color, indexed_to_rgb, profile, terminal_background_rgb,
    terminal_foreground_rgb,
};

impl Color {
    pub fn to_rgb_fg(self) -> Rgb {
        self.to_rgb(true)
    }

    pub fn to_rgb_bg(self) -> Rgb {
        self.to_rgb(false)
    }

    fn to_rgb(self, is_fg: bool) -> Rgb {
        match self {
            Color::Rgb(val) => val,
            Color::Hsl(val) => Rgb::from_color(val),
            Color::Hsluv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Hsv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Hwb(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Lab(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Lch(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Lchuv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Luv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Okhsl(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Okhsv(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Okhwb(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Oklab(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Oklch(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Xyz(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::Yxy(val) => Rgb::<::palette::encoding::Srgb, _>::from_color(val),
            Color::AnsiReset => {
                if is_fg {
                    terminal_foreground_rgb()
                } else {
                    terminal_background_rgb()
                }
            }
            Color::AnsiBlack => indexed_to_rgb(0),
            Color::AnsiRed => indexed_to_rgb(1),
            Color::AnsiGreen => indexed_to_rgb(2),
            Color::AnsiYellow => indexed_to_rgb(3),
            Color::AnsiBlue => indexed_to_rgb(4),
            Color::AnsiMagenta => indexed_to_rgb(5),
            Color::AnsiCyan => indexed_to_rgb(6),
            Color::AnsiGray => indexed_to_rgb(7),
            Color::AnsiDarkGray => indexed_to_rgb(8),
            Color::AnsiLightRed => indexed_to_rgb(9),
            Color::AnsiLightGreen => indexed_to_rgb(10),
            Color::AnsiLightYellow => indexed_to_rgb(11),
            Color::AnsiLightBlue => indexed_to_rgb(12),
            Color::AnsiLightMagenta => indexed_to_rgb(13),
            Color::AnsiLightCyan => indexed_to_rgb(14),
            Color::AnsiWhite => indexed_to_rgb(15),
            Color::Indexed(idx) => indexed_to_rgb(idx),
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
            Color::Hsluv(val) => palette_to_anstyle(val),
            Color::Hsv(val) => palette_to_anstyle(val),
            Color::Hwb(val) => palette_to_anstyle(val),
            Color::Lab(val) => palette_to_anstyle(val),
            Color::Lch(val) => palette_to_anstyle(val),
            Color::Lchuv(val) => palette_to_anstyle(val),
            Color::Luv(val) => palette_to_anstyle(val),
            Color::Okhsl(val) => palette_to_anstyle(val),
            Color::Okhsv(val) => palette_to_anstyle(val),
            Color::Okhwb(val) => palette_to_anstyle(val),
            Color::Oklab(val) => palette_to_anstyle(val),
            Color::Oklch(val) => palette_to_anstyle(val),
            Color::Xyz(val) => palette_to_anstyle(val),
            Color::Yxy(val) => palette_to_anstyle(val),
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
        value.into_anstyle()
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl From<Hsl> for Color {
    fn from(value: Hsl) -> Self {
        Self::Hsl(value)
    }
}

impl From<Hsluv> for Color {
    fn from(value: Hsluv) -> Self {
        Self::Hsluv(value)
    }
}

impl From<Hsv> for Color {
    fn from(value: Hsv) -> Self {
        Self::Hsv(value)
    }
}

impl From<Hwb> for Color {
    fn from(value: Hwb) -> Self {
        Self::Hwb(value)
    }
}

impl From<Lab> for Color {
    fn from(value: Lab) -> Self {
        Self::Lab(value)
    }
}

impl From<Lch> for Color {
    fn from(value: Lch) -> Self {
        Self::Lch(value)
    }
}

impl From<Lchuv> for Color {
    fn from(value: Lchuv) -> Self {
        Self::Lchuv(value)
    }
}

impl From<Luv> for Color {
    fn from(value: Luv) -> Self {
        Self::Luv(value)
    }
}

impl From<Okhsl> for Color {
    fn from(value: Okhsl) -> Self {
        Self::Okhsl(value)
    }
}

impl From<Okhsv> for Color {
    fn from(value: Okhsv) -> Self {
        Self::Okhsv(value)
    }
}

impl From<Okhwb> for Color {
    fn from(value: Okhwb) -> Self {
        Self::Okhwb(value)
    }
}

impl From<Oklab> for Color {
    fn from(value: Oklab) -> Self {
        Self::Oklab(value)
    }
}

impl From<Oklch> for Color {
    fn from(value: Oklch) -> Self {
        Self::Oklch(value)
    }
}

impl From<Xyz> for Color {
    fn from(value: Xyz) -> Self {
        Self::Xyz(value)
    }
}

impl From<Yxy> for Color {
    fn from(value: Yxy) -> Self {
        Self::Yxy(value)
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        Self::Indexed(value)
    }
}
