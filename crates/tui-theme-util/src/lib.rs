use std::ffi::OsStr;
use std::io::{self, Read};
use std::path::PathBuf;

use fs::File;
use fs_err as fs;
use tui_theme::{Color, NamedColor};

pub fn read_themes_from_dir<P>(dir: P) -> Vec<String>
where
    P: Into<PathBuf>,
{
    let theme_files = fs::read_dir(dir)
        .unwrap()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_file() && f.path().extension() == Some(OsStr::new("css")))
        .map(|f| {
            f.file_name()
                .to_string_lossy()
                .to_string()
                .replace(".css", "")
        });
    theme_files.collect()
}

pub fn parse_theme_css<P>(path: P) -> io::Result<Vec<NamedColor<'static>>>
where
    P: Into<PathBuf>,
{
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents
        .split("\n")
        .filter_map(|l| {
            let l = l.trim();
            if !l.starts_with("--") {
                return None;
            }
            Some(l)
        })
        .map(|line| {
            let line = line.to_ascii_lowercase();
            let parts: Vec<_> = line.split(": ").collect();
            let [name, val] = parts.as_slice() else {
                panic!("invalid format");
            };
            let name = name
                .replacen("--", "", 1)
                .replace("-", "_")
                .replacen("color_", "", 1);
            let color: Color = val.parse().unwrap();
            let group = name.rsplitn(2, "_").last().unwrap();

            let variant = name.rsplit("_").next().unwrap();

            NamedColor {
                variant: variant.to_ascii_lowercase().into(),
                group: group.to_ascii_lowercase().into(),
                color,
            }
        });
    Ok(lines.collect())
}
