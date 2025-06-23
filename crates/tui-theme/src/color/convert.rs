use palette::FromColor;
use palette::rgb::Rgb;

use super::Color;

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
