use std::io::{self};
use std::path::PathBuf;

use clap::{Args, Parser};
use tui_theme_util::{parse_theme_css, read_themes_from_path};

mod generate_theme;
mod open_theme;
mod print_theme;

#[derive(Args)]
struct GenerateArgs {
    #[arg(long)]
    crate_dir: PathBuf,
    #[arg(long)]
    src_dir: PathBuf,
    #[arg(long)]
    dest_dir: PathBuf,
}

#[derive(Parser)]
enum Action {
    Generate(GenerateArgs),
    Open { theme_path: PathBuf },
    Print { theme_path: PathBuf },
}

fn main() -> io::Result<()> {
    let action = Action::parse();

    match action {
        Action::Generate(GenerateArgs {
            crate_dir,
            src_dir,
            dest_dir,
        }) => generate_theme::generate(&crate_dir, &src_dir, &dest_dir),
        Action::Open { theme_path } => open_theme::open(&theme_path),
        Action::Print {
            theme_path: theme_dir,
        } => print_theme::print(&theme_dir),
    }
}
