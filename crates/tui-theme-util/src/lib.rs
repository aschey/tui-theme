use std::ffi::OsStr;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use fs::File;
use fs_err as fs;
use tui_theme::{Color, NamedColor};

pub struct ThemeFile {
    pub path: PathBuf,
    pub name: String,
}

impl ThemeFile {
    fn from_file(path: PathBuf) -> Self {
        Self {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .replace(".css", ""),
            path,
        }
    }
}

pub fn read_themes_from_path<P>(path: P) -> Vec<ThemeFile>
where
    P: Into<PathBuf>,
{
    let path = path.into();
    if is_css_file(&path) {
        vec![ThemeFile::from_file(path)]
    } else {
        fs::read_dir(&path)
            .unwrap()
            .filter_map(|f| f.ok().map(|f| f.path()))
            .filter(|f| is_css_file(f))
            .map(ThemeFile::from_file)
            .collect()
    }
}

fn is_css_file(path: &Path) -> bool {
    path.is_file() && path.extension() == Some(OsStr::new("css"))
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

pub fn theme_selector(theme_files: &[ThemeFile]) -> io::Result<usize> {
    match theme_files.len() {
        0 => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no themes found",
        )),
        1 => Ok(0),
        _ => Ok(Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select theme")
            .default(0)
            .items(theme_files.iter().map(|t| &t.name))
            .interact()
            .unwrap()),
    }
}
