mod color;
pub mod palette;
mod theme;

use color::color_scheme;
pub use color::*;
use terminal_colorsaurus::ColorScheme;
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
        let theme = color_scheme();
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
