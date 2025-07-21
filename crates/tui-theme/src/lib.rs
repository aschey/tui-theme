mod color;
pub mod palette;
mod style;
mod theme;

// hack for referencing the current crate in proc macros https://github.com/bkchr/proc-macro-crate/issues/14#issuecomment-1742071768
extern crate self as tui_theme;

pub use color::*;
pub use style::*;
use terminal_colorsaurus::ColorScheme;
pub use theme::*;
pub use tui_theme_derive::*;
pub mod profile {
    pub use termprofile::*;
}

pub enum ThemeChoice {
    Dark,
    Light,
}

pub struct Adaptive<T> {
    dark: T,
    light: T,
    choice: ThemeChoice,
}

impl<T> Adaptive<T> {
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

    pub fn adapt(&self) -> &T {
        match self.choice {
            ThemeChoice::Dark => &self.dark,
            ThemeChoice::Light => &self.light,
        }
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
