use bitflags::bitflags;
use termprofile::TermProfile;

use crate::{Color, profile};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    underline: Option<Color>,
    modifiers: Modifiers,
    reset: bool,
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Modifiers: u16 {
        const BOLD              = 1<<1;
        const DIM               = 1<<2;
        const ITALIC            = 1<<3;
        const UNDERLINE         = 1<<4;
        const DOUBLE_UNDERLINE  = 1<<5;
        const CURLY_UNDERLINE   = 1<<6;
        const DOTTED_UNDERLINE  = 1<<7;
        const DASHED_UNDERLINE  = 1<<8;
        const SLOW_BLINK        = 1<<9;
        const RAPID_BLINK       = 1<<10;
        const INVERT            = 1<<11;
        const HIDE              = 1<<12;
        const STRIKETHROUGH     = 1<<13;
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn underline(mut self, color: Color) -> Self {
        self.underline = Some(color);
        self
    }

    pub fn modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = self.modifiers.union(modifiers);
        self
    }

    pub fn reset(mut self) -> Self {
        self.reset = true;
        self
    }

    pub fn into_adaptive(self) -> Self {
        let profile = profile().unwrap_or(TermProfile::TrueColor);
        if profile == TermProfile::NoTty {
            return Self::new();
        }
        Self {
            fg: self.fg.map(|c| c.into_adaptive()),
            bg: self.bg.map(|c| c.into_adaptive()),
            underline: self.underline.map(|c| c.into_adaptive()),
            modifiers: self.modifiers,
            reset: self.reset,
        }
    }
}

impl From<Style> for ratatui::style::Style {
    fn from(val: Style) -> Self {
        let mut ratatui_style = if val.reset {
            ratatui::style::Style::reset()
        } else {
            ratatui::style::Style::new()
        };
        if let Some(fg) = val.fg {
            ratatui_style = ratatui_style.fg(fg.into());
        }
        if let Some(bg) = val.bg {
            ratatui_style = ratatui_style.bg(bg.into());
        }
        if let Some(underline) = val.underline {
            ratatui_style = ratatui_style.underline_color(underline.into());
        }
        ratatui_style.add_modifier(val.modifiers.into())
    }
}

impl From<ratatui::style::Style> for Style {
    fn from(value: ratatui::style::Style) -> Self {
        Style {
            fg: value.fg.map(Into::into),
            bg: value.bg.map(Into::into),
            underline: value.underline_color.map(Into::into),
            modifiers: value.add_modifier.into(),
            reset: value.sub_modifier == ratatui::style::Modifier::all(),
        }
    }
}

impl From<Style> for anstyle::Style {
    fn from(val: Style) -> Self {
        let profile = profile().unwrap_or(TermProfile::TrueColor);

        let style = anstyle::Style::new()
            .fg_color(val.fg.and_then(Into::into))
            .bg_color(val.bg.and_then(Into::into))
            .underline_color(val.underline.and_then(Into::into));
        profile.adapt_style(style)
    }
}

impl From<anstyle::Style> for Style {
    fn from(value: anstyle::Style) -> Self {
        Style {
            fg: value.get_fg_color().map(Into::into),
            bg: value.get_fg_color().map(Into::into),
            underline: value.get_underline_color().map(Into::into),
            modifiers: value.get_effects().into(),
            reset: false,
        }
    }
}

impl From<Modifiers> for ratatui::style::Modifier {
    fn from(value: Modifiers) -> Self {
        let mut modifier = ratatui::style::Modifier::empty();

        if value.intersects(Modifiers::BOLD) {
            modifier |= ratatui::style::Modifier::BOLD;
        }
        if value.intersects(Modifiers::DIM) {
            modifier |= ratatui::style::Modifier::DIM;
        }
        if value.intersects(Modifiers::ITALIC) {
            modifier |= ratatui::style::Modifier::ITALIC;
        }
        if value.intersects(
            Modifiers::UNDERLINE
                | Modifiers::DOUBLE_UNDERLINE
                | Modifiers::CURLY_UNDERLINE
                | Modifiers::DOTTED_UNDERLINE
                | Modifiers::DASHED_UNDERLINE,
        ) {
            modifier |= ratatui::style::Modifier::UNDERLINED;
        }
        if value.intersects(Modifiers::SLOW_BLINK) {
            modifier |= ratatui::style::Modifier::SLOW_BLINK;
        }
        if value.intersects(Modifiers::RAPID_BLINK) {
            modifier |= ratatui::style::Modifier::RAPID_BLINK;
        }
        if value.intersects(Modifiers::INVERT) {
            modifier |= ratatui::style::Modifier::REVERSED;
        }
        if value.intersects(Modifiers::HIDE) {
            modifier |= ratatui::style::Modifier::HIDDEN;
        }
        if value.intersects(Modifiers::STRIKETHROUGH) {
            modifier |= ratatui::style::Modifier::CROSSED_OUT;
        }

        modifier
    }
}

impl From<ratatui::style::Modifier> for Modifiers {
    fn from(value: ratatui::style::Modifier) -> Self {
        let mut modifiers = Modifiers::empty();
        if matches!(profile(), Ok(TermProfile::NoTty)) {
            return modifiers;
        }

        if value.intersects(ratatui::style::Modifier::BOLD) {
            modifiers |= Modifiers::BOLD;
        }
        if value.intersects(ratatui::style::Modifier::DIM) {
            modifiers |= Modifiers::DIM;
        }
        if value.intersects(ratatui::style::Modifier::ITALIC) {
            modifiers |= Modifiers::ITALIC;
        }
        if value.intersects(ratatui::style::Modifier::UNDERLINED) {
            modifiers |= Modifiers::UNDERLINE;
        }
        if value.intersects(ratatui::style::Modifier::SLOW_BLINK) {
            modifiers |= Modifiers::SLOW_BLINK;
        }
        if value.intersects(ratatui::style::Modifier::RAPID_BLINK) {
            modifiers |= Modifiers::RAPID_BLINK;
        }
        if value.intersects(ratatui::style::Modifier::REVERSED) {
            modifiers |= Modifiers::INVERT;
        }
        if value.intersects(ratatui::style::Modifier::HIDDEN) {
            modifiers |= Modifiers::HIDE;
        }
        if value.intersects(ratatui::style::Modifier::CROSSED_OUT) {
            modifiers |= Modifiers::STRIKETHROUGH;
        }

        modifiers
    }
}

impl From<Modifiers> for anstyle::Effects {
    fn from(value: Modifiers) -> Self {
        let mut modifier = anstyle::Effects::new();
        if value.intersects(Modifiers::BOLD) {
            modifier |= anstyle::Effects::BOLD;
        }
        if value.intersects(Modifiers::DIM) {
            modifier |= anstyle::Effects::DIMMED;
        }
        if value.intersects(Modifiers::ITALIC) {
            modifier |= anstyle::Effects::ITALIC;
        }
        if value.intersects(Modifiers::UNDERLINE) {
            modifier |= anstyle::Effects::UNDERLINE;
        }
        if value.intersects(Modifiers::DOUBLE_UNDERLINE) {
            modifier |= anstyle::Effects::DOUBLE_UNDERLINE;
        }
        if value.intersects(Modifiers::CURLY_UNDERLINE) {
            modifier |= anstyle::Effects::CURLY_UNDERLINE;
        }
        if value.intersects(Modifiers::DOTTED_UNDERLINE) {
            modifier |= anstyle::Effects::DASHED_UNDERLINE;
        }
        if value.intersects(Modifiers::SLOW_BLINK | Modifiers::RAPID_BLINK) {
            modifier |= anstyle::Effects::BLINK;
        }
        if value.intersects(Modifiers::INVERT) {
            modifier |= anstyle::Effects::INVERT;
        }
        if value.intersects(Modifiers::HIDE) {
            modifier |= anstyle::Effects::HIDDEN;
        }
        if value.intersects(Modifiers::STRIKETHROUGH) {
            modifier |= anstyle::Effects::STRIKETHROUGH;
        }

        modifier
    }
}

impl From<anstyle::Effects> for Modifiers {
    fn from(value: anstyle::Effects) -> Self {
        let mut modifier = Modifiers::empty();
        if value.contains(anstyle::Effects::BOLD) {
            modifier |= Modifiers::BOLD;
        }
        if value.contains(anstyle::Effects::DIMMED) {
            modifier |= Modifiers::DIM;
        }
        if value.contains(anstyle::Effects::ITALIC) {
            modifier |= Modifiers::ITALIC;
        }
        if value.contains(anstyle::Effects::UNDERLINE) {
            modifier |= Modifiers::UNDERLINE;
        }
        if value.contains(anstyle::Effects::DOUBLE_UNDERLINE) {
            modifier |= Modifiers::DOUBLE_UNDERLINE;
        }
        if value.contains(anstyle::Effects::DOTTED_UNDERLINE) {
            modifier |= Modifiers::DOTTED_UNDERLINE;
        }
        if value.contains(anstyle::Effects::DASHED_UNDERLINE) {
            modifier |= Modifiers::DASHED_UNDERLINE;
        }

        if value.contains(anstyle::Effects::BLINK) {
            modifier |= Modifiers::SLOW_BLINK;
        }
        if value.contains(anstyle::Effects::INVERT) {
            modifier |= Modifiers::INVERT;
        }
        if value.contains(anstyle::Effects::HIDDEN) {
            modifier |= Modifiers::HIDE;
        }
        if value.contains(anstyle::Effects::STRIKETHROUGH) {
            modifier |= Modifiers::STRIKETHROUGH;
        }

        modifier
    }
}

pub trait Stylize<T> {
    fn fg<C>(self, color: C) -> T
    where
        C: Into<Color>;

    fn bg<C>(self, color: C) -> T
    where
        C: Into<Color>;

    fn underline<C>(self, color: C) -> T
    where
        C: Into<Color>;
}

impl<T, U> Stylize<T> for U
where
    U: ratatui::style::Styled<Item = T>,
{
    fn fg<C>(self, color: C) -> T
    where
        C: Into<Color>,
    {
        let color: Color = color.into();
        let style = self.style().fg(color.into());
        self.set_style(style)
    }

    fn bg<C>(self, color: C) -> T
    where
        C: Into<Color>,
    {
        let color: Color = color.into();
        let style = self.style().bg(color.into());
        self.set_style(style)
    }

    fn underline<C>(self, color: C) -> T
    where
        C: Into<Color>,
    {
        let color: Color = color.into();
        let style = self.style().underline_color(color.into());
        self.set_style(style)
    }
}

impl Stylize<Self> for Style {
    fn fg<C>(self, color: C) -> Self
    where
        C: Into<Color>,
    {
        self.fg(color.into())
    }

    fn bg<C>(self, color: C) -> Self
    where
        C: Into<Color>,
    {
        self.bg(color.into())
    }

    fn underline<C>(self, color: C) -> Self
    where
        C: Into<Color>,
    {
        self.underline(color.into())
    }
}

pub trait Styled {
    type Item;

    fn style(&self) -> Style;

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item;
}

impl Styled for Style {
    type Item = Self;
    fn style(&self) -> Style {
        *self
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        style.into()
    }
}

impl<T> Styled for T
where
    T: ratatui::style::Styled,
{
    type Item = <T as ratatui::style::Styled>::Item;

    fn style(&self) -> Style {
        <Self as ratatui::style::Styled>::style(self).into()
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Self::Item {
        <Self as ratatui::style::Styled>::set_style(self, style.into())
    }
}
