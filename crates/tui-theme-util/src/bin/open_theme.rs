use std::fs::File;
use std::io::{self, Read};

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use tui_theme::Color;

fn main() -> io::Result<()> {
    let mut file = File::open("themes/catppuccin.css").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n").filter_map(|l| {
        let l = l.trim();
        if !l.starts_with("--") {
            return None;
        }
        Some(l)
    });
    let formatted: Vec<String> = lines
        .into_iter()
        .filter_map(|line| {
            let parts: Vec<_> = line.split(": ").collect();
            let [name, val] = parts.as_slice() else {
                panic!("invalid format");
            };

            let color_num = "500";
            let color_suffix = "-".to_string() + color_num;
            if name.ends_with(&color_suffix) {
                let name = name.replace("--color-", "").replace(&color_suffix, "");
                let color: Color = val.parse().unwrap();
                println!("color {color:?}");
                let hex = color.to_hex_fg();
                println!("hex {hex}");
                Some(format_color(&name, &hex, color_num))
            } else {
                None
            }
        })
        .collect();
    ::open::that_detached(format!(
        "https://www.tints.dev/palette/v1:{}",
        BASE64_URL_SAFE_NO_PAD.encode(formatted.join("~"))
    ))
    .unwrap();
    Ok(())
}

fn format_color(name: &str, hex: &str, number: &str) -> String {
    format!("{name}|{}|{number}|p|0|0|0|100|a", &hex[1..])
}
