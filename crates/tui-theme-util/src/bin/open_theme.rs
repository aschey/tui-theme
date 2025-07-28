use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read};

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use tui_theme::Color;
use tui_theme_util::{parse_theme_css, read_themes_from_dir};

fn main() -> io::Result<()> {
    let theme_files = read_themes_from_dir("themes");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select theme")
        .default(0)
        .items(&theme_files[..])
        .interact()
        .unwrap();

    let file = format!("themes/{}.css", theme_files[selection]);
    let colors = parse_theme_css(file)?;
    let formatted: Vec<String> = colors
        .into_iter()
        .filter_map(|line| {
            let color_num = "500";
            if line.variant == color_num {
                let hex = line.color.to_hex_fg();
                Some(format_color(&line.group, &hex, color_num))
            } else {
                None
            }
        })
        .collect();
    open::that(format!(
        "https://www.tints.dev/palette/v1:{}",
        BASE64_URL_SAFE_NO_PAD.encode(formatted.join("~"))
    ))
    .unwrap();
    Ok(())
}

fn format_color(name: &str, hex: &str, number: &str) -> String {
    format!("{name}|{}|{number}|p|0|0|0|100|a", &hex[1..])
}
