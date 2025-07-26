use std::borrow::Cow;
use std::io::{self, stdout};

use anstyle_crossterm::to_crossterm;
use crossterm::style::Stylize;
use indexmap::{IndexMap, IndexSet};
use palette::color_difference::Wcag21RelativeContrast;
use tui_theme::palette::Gruvbox;
use tui_theme::{Color, NamedColor};

fn main() -> io::Result<()> {
    tui_theme::load_color_palette();
    tui_theme::load_profile(&stdout());
    let colors = Gruvbox::ALL_COLORS;
    let headers: IndexSet<&Cow<'_, str>> = colors.iter().map(|c| &c.group).collect();

    let max_header_len = headers.iter().map(|h| h.len()).max().unwrap() + 2;
    for header in headers {
        let header = format!("{header:^max_header_len$} ");
        print!("{}", header.bold());
    }
    println!();
    let grouped_colors = group(colors.iter().map(|c| (&c.variant, c)));
    for variant in grouped_colors.keys() {
        for named_color in grouped_colors[variant].iter() {
            let NamedColor {
                color,
                group: _,
                variant,
            } = named_color;

            let color_luminance = named_color.color.to_rgb_fg().relative_luminance().luma;
            let anstyle_color: Option<anstyle::Color> = (*color).into();
            if let Some(anstyle_color) = anstyle_color {
                let fg_color: Color = if color_luminance < 0.179 {
                    "rgb(220 220 220)".parse().unwrap()
                } else {
                    "rgb(20 20 20)".parse().unwrap()
                };
                let style = to_crossterm(
                    anstyle::Style::new()
                        .fg_color(fg_color.into())
                        .bg_color(Some(anstyle_color)),
                );
                print!("{} ", style.apply(format!("{variant:^max_header_len$}")));
            }
        }
        println!()
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
