use std::borrow::Cow;
use std::io::{self, stdout};

use anstyle_crossterm::to_crossterm;
use crossterm::style::Stylize;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use indexmap::{IndexMap, IndexSet};
use palette::color_difference::Wcag21RelativeContrast;
use tui_theme::{Color, NamedColor, Style};
use tui_theme_util::{parse_theme_css, read_themes_from_dir};

struct PrintableTheme {
    column_width: usize,
    sections: Vec<Section>,
}

struct Row {
    label: String,
    cells: Vec<Cell>,
}

impl Row {
    fn new(variant: &str, colors: Vec<&NamedColor<'_>>) -> Self {
        Row {
            label: variant.to_string(),
            cells: colors.iter().map(|c| Cell::new(&c.color)).collect(),
        }
    }
}

struct Cell {
    fg: Color,
    bg: Color,
}

impl Cell {
    fn new(color: &Color) -> Self {
        let rgb_color = color.to_rgb_bg();
        let color_luminance = rgb_color.into_linear::<f32>().relative_luminance().luma;
        let fg = if color_luminance < 0.179 {
            Color::Rgb(220, 220, 220)
        } else {
            Color::Rgb(20, 20, 20)
        };
        Cell {
            fg,
            bg: rgb_color.into(),
        }
    }
}

struct Section {
    headers: Vec<String>,
    rows: Vec<Row>,
}

impl Section {
    fn new(header_group: &[&Cow<'_, str>], colors: &[NamedColor]) -> Self {
        let headers = header_group.iter().map(|h| h.to_string()).collect();
        let grouped_colors = group(
            colors
                .iter()
                .filter(|c| header_group.contains(&&c.group))
                .map(|c| (&c.variant, c)),
        );
        let rows = grouped_colors
            .into_iter()
            .map(|(variant, colors)| Row::new(variant, colors))
            .collect();

        Self { headers, rows }
    }
}

impl PrintableTheme {
    fn new<'a, I>(colors: I, width: usize) -> Self
    where
        I: Into<Vec<NamedColor<'a>>>,
    {
        let colors = colors.into();
        let headers: IndexSet<&Cow<'_, str>> = colors.iter().map(|c| &c.group).collect();
        let headers: Vec<_> = headers.into_iter().collect();

        let max_header_len = (headers.iter().map(|h| h.len()).max().unwrap() + 2).max(9);
        let columns_per_section = width / (max_header_len + 1);

        let sections = headers
            .chunks(columns_per_section)
            .map(|header_group| Section::new(header_group, &colors))
            .collect();
        PrintableTheme {
            column_width: max_header_len,
            sections,
        }
    }
}

fn main() -> io::Result<()> {
    tui_theme::load_color_palette();
    tui_theme::load_profile(&stdout());
    let columns = crossterm::terminal::window_size().unwrap().columns;

    let theme_files = read_themes_from_dir("themes");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select theme")
        .default(0)
        .items(&theme_files[..])
        .interact()
        .unwrap();

    let file = format!("themes/{}.css", theme_files[selection]);
    let colors = parse_theme_css(file)?;

    let theme = PrintableTheme::new(colors, columns as usize);
    let column_width = theme.column_width;
    println!();
    for section in theme.sections {
        for header in section.headers {
            let formatted_header = format!("{header:^column_width$}");
            print!("{} ", formatted_header.bold());
        }
        println!();
        for row in section.rows {
            let label = row.label;
            for cell in row.cells {
                let an: anstyle::Style = Style::new().fg(cell.fg).bg(cell.bg).into();
                let style = to_crossterm(an);
                print!("{} ", style.apply(format!("{label:^column_width$}")));
            }
            println!();
        }
        println!();
    }
    Ok(())
}

fn group<K: Eq + std::hash::Hash + Ord, V, I: Iterator<Item = (K, V)>>(
    iter: I,
) -> IndexMap<K, Vec<V>> {
    iter.fold(IndexMap::new(), |mut map, (k, v)| {
        map.entry(k).or_insert_with(|| Vec::new()).push(v);
        map
    })
}
