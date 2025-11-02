//! # [Ratatui] Demo2 example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

mod app;
mod tabs;
mod theme;

use std::io::stdout;

use app::App;
use color_eyre::Result;
use tui_theme::profile::DetectorSettings;

fn main() -> Result<()> {
    color_eyre::install()?;
    tui_theme::load_color_palette();
    tui_theme::load_profile(&stdout(), DetectorSettings::with_query()?);

    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
