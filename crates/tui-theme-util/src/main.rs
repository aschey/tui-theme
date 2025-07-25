use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::Command;

use convert_case::{Case, Casing};
use tui_theme::Color;

fn main() -> io::Result<()> {
    // palettes created with https://www.tints.dev
    let palette_dir = Path::new("../tui-theme/src/palette");
    fs::remove_dir_all(palette_dir)?;
    fs::create_dir_all(palette_dir)?;
    let mut mod_file = File::create(palette_dir.join("mod.rs"))?;
    for path in fs::read_dir("themes")? {
        let path = path.unwrap().path();
        let name = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .replace(".css", "");

        read_theme(&name, &path).unwrap();
        let mod_name = name.to_case(Case::Snake);
        writeln!(mod_file, "mod {mod_name};")?;
        writeln!(mod_file, "pub use {mod_name}::*;")?;
    }

    Command::new("cargo")
        .args(["+nightly", "fmt"])
        .current_dir("../..")
        .output()
        .unwrap();

    Ok(())
}

fn read_theme(name: &str, path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n").filter_map(|l| {
        let l = l.trim();
        if !l.starts_with("--") {
            return None;
        }
        Some(l)
    });

    let mut out = File::create(format!(
        "../tui-theme/src/palette/{}.rs",
        name.to_case(Case::Snake)
    ))?;
    let name_caps = name.to_case(Case::UpperCamel);
    writeln!(out, "use crate::Color;")?;
    writeln!(out, "use crate::ThemeArray;\n")?;
    writeln!(out, "// Auto-generated file. Do not edit.\n")?;
    writeln!(out, "pub struct {name_caps} {{}}\n")?;
    writeln!(out, "impl {name_caps} {{")?;
    let mut color_groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in lines {
        let parts: Vec<_> = line.split(": ").collect();
        let [name, val] = parts.as_slice() else {
            panic!("invalid format");
        };
        let name = name
            .replacen("--", "", 1)
            .replace("-", "_")
            .replacen("color_", "", 1)
            .to_ascii_uppercase();

        let name_base = name.rsplitn(2, "_").last().unwrap();
        if let Some(colors) = color_groups.get_mut(name_base) {
            colors.push(name.clone());
        } else {
            color_groups.insert(name_base.to_string(), vec![name.clone()]);
        }

        let color: Color = val.parse().unwrap();
        let Color::Oklch(color) = color else {
            panic!("invalid color");
        };
        writeln!(
            out,
            "    #[allow(clippy::excessive_precision, clippy::approx_constant)]"
        )?;
        writeln!(out, "{}", generate_const(&name, color))?;
    }

    let mut colors_len = 0;
    for (color_group, colors) in &color_groups {
        let color_array_vals: Vec<_> = colors.iter().map(|c| format!("Self::{c}")).collect();
        colors_len = colors.len();
        writeln!(
            out,
            "    pub const {color_group}: ThemeArray<{}> = ThemeArray([{}]);\n",
            colors_len,
            color_array_vals.join(",")
        )?;
    }
    let group_keys: Vec<_> = color_groups.keys().map(|k| format!("Self::{k}")).collect();
    writeln!(
        out,
        "  pub const ALL_COLORS: [ThemeArray<{colors_len}>;{}] = [{}];",
        group_keys.len(),
        group_keys.join(",")
    )?;
    writeln!(out, "}}")?;

    Ok(())
}

fn generate_const(name: &str, color: ::palette::Oklch) -> String {
    format!(
        "    pub const {name}: Color = Color::Oklch(::palette::Oklch::new_const({:.4}, {:.4}, \
         ::palette::OklabHue::new({:.4})));\n",
        color.l,
        color.chroma,
        color.hue.into_raw_degrees()
    )
}
