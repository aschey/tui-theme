mod color;
mod local_override;
pub mod palette;
mod style;
mod theme;

// hack for referencing the current crate in proc macros
// https://github.com/bkchr/proc-macro-crate/issues/14#issuecomment-1742071768
extern crate self as tui_theme;

use std::ops::{Deref, DerefMut, Index};

pub use color::*;
pub use style::*;
use termprofile::TermProfile;
pub use theme::*;
pub use tui_theme_derive::*;
pub mod profile {
    pub use termprofile::*;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScheme {
    Dark,
    Light,
}

impl From<terminal_colorsaurus::ThemeMode> for ColorScheme {
    fn from(value: terminal_colorsaurus::ThemeMode) -> Self {
        match value {
            terminal_colorsaurus::ThemeMode::Dark => ColorScheme::Dark,
            terminal_colorsaurus::ThemeMode::Light => ColorScheme::Light,
        }
    }
}

pub struct Adaptive<T> {
    dark: T,
    light: T,
    choice: ColorScheme,
}

impl<T> Adaptive<T> {
    pub fn auto(dark: T, light: T) -> Self {
        let theme = color_scheme();
        Self::new(dark, light, theme)
    }

    pub fn new(dark: T, light: T, theme: ColorScheme) -> Self {
        Self {
            dark,
            light,
            choice: theme,
        }
    }

    pub fn adapt(&self) -> &T {
        match self.choice {
            ColorScheme::Dark => &self.dark,
            ColorScheme::Light => &self.light,
        }
    }
}

pub struct ProfileVariant<T> {
    default_value: T,
    ansi_256: Option<T>,
    ansi_16: Option<T>,
    ascii: Option<T>,
    no_tty: Option<T>,
}

impl<T> ProfileVariant<T> {
    pub fn new(default_value: T) -> Self {
        Self {
            default_value,
            ansi_256: None,
            ansi_16: None,
            ascii: None,
            no_tty: None,
        }
    }

    pub fn ansi_16(mut self, value: T) -> Self {
        self.ansi_16 = Some(value);
        self
    }

    pub fn adapt(self) -> T {
        let current_profile = term_profile();
        if current_profile <= TermProfile::NoTty
            && let Some(no_tty) = self.no_tty
        {
            return no_tty;
        }
        if current_profile <= TermProfile::NoColor
            && let Some(ascii) = self.ascii
        {
            return ascii;
        }
        if current_profile <= TermProfile::Ansi16
            && let Some(ansi_16) = self.ansi_16
        {
            return ansi_16;
        }
        if current_profile <= TermProfile::Ansi256
            && let Some(ansi_256) = self.ansi_256
        {
            return ansi_256;
        }

        self.default_value
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeArray<const N: usize>(pub [Color; N]);

#[derive(Debug, Clone, Copy)]
pub struct Dark(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct Light(pub usize);

impl<const N: usize> Deref for ThemeArray<N> {
    type Target = [Color; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ThemeArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Index<(Light, Dark)> for ThemeArray<N> {
    type Output = Color;

    fn index(&self, (light, dark): (Light, Dark)) -> &Self::Output {
        if color_scheme() == ColorScheme::Light {
            &self.0[light.0]
        } else {
            &self.0[dark.0]
        }
    }
}

impl<const N: usize> Index<(Dark, Light)> for ThemeArray<N> {
    type Output = Color;

    fn index(&self, (dark, light): (Dark, Light)) -> &Self::Output {
        if color_scheme() == ColorScheme::Light {
            &self.0[light.0]
        } else {
            &self.0[dark.0]
        }
    }
}

impl From<ProfileVariant<Color>> for Color {
    fn from(value: ProfileVariant<Color>) -> Self {
        value.adapt()
    }
}

impl From<ProfileVariant<Style>> for Style {
    fn from(value: ProfileVariant<Style>) -> Self {
        value.adapt()
    }
}

impl<T> SetTheme for Adaptive<T>
where
    T: SetTheme,
{
    type Theme = T::Theme;

    fn set_global(&self) {
        self.adapt().set_global();
    }

    fn unset_local() {
        T::unset_local();
    }

    fn set_local(&self) {
        self.adapt().set_local();
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
