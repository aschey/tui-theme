use itertools::Itertools;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap};
use tui_theme::Color;

use crate::theme::{AppThemeStyle, LogoColorExt};

const RATATUI_LOGO: [&str; 32] = [
    "               ███              ",
    "             ██████             ",
    "            ███████             ",
    "           ████████             ",
    "          █████████             ",
    "         ██████████             ",
    "        ████████████            ",
    "        █████████████           ",
    "        █████████████     ██████",
    "         ███████████    ████████",
    "              █████ ███████████ ",
    "               ███ ██ee████████ ",
    "                █ █████████████ ",
    "            ████ █████████████  ",
    "           █████████████████    ",
    "           ████████████████     ",
    "           ████████████████     ",
    "            ███ ██████████      ",
    "          ▒▒    █████████       ",
    "         ▒xx▒   █████████       ",
    "        ▒xxxx▒ ██████████       ",
    "       ▒xx█xxx▒ █████████       ",
    "      ▒xx██xxxx▒ ████████       ",
    "     ▒xxxxxxxxxx▒ ██████████    ",
    "    ▒xxxxxxxxxxxx▒ ██████████   ",
    "   ▒xxxxxxx██xxxxx▒ █████████   ",
    "  ▒xxxxxxxxx██xxxxx▒ ████  ███  ",
    " ▒xxxxxxxxxxxxxxxxxx▒ ██   ███  ",
    "▒xxxxxxxxxxxxxxxxxxxx▒ █   ███  ",
    "▒xxxxxxxxxxxxxxxxxxxxx▒   ███   ",
    " ▒xxxxxxxxxxxxxxxxxxxxx▒ ███    ",
    "  ▒xxxxxxxxxxxxxxxxxxxxx▒ █     ",
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AboutTab {
    row_index: usize,
}

impl AboutTab {
    pub fn prev_row(&mut self) {
        self.row_index = self.row_index.saturating_sub(1);
    }

    pub fn next_row(&mut self) {
        self.row_index = self.row_index.saturating_add(1);
    }
}

impl Widget for AboutTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Length(34), Constraint::Min(0)]);
        let [description, logo] = horizontal.areas(area);
        render_crate_description(description, buf);
        render_logo(self.row_index, logo, buf);
    }
}

fn render_crate_description(area: Rect, buf: &mut Buffer) {
    let area = area.inner(Margin {
        vertical: 4,
        horizontal: 2,
    });
    Clear.render(area, buf); // clear out the color swatches
    Block::new().style_content().render(area, buf);
    let area = area.inner(Margin {
        vertical: 1,
        horizontal: 2,
    });
    let text = "- cooking up terminal user interfaces -

    Ratatui is a Rust crate that provides widgets (e.g. Paragraph, Table) and draws them to the \
                screen efficiently every frame.";
    Paragraph::new(text)
        .style_description()
        .block(
            Block::new()
                .title(" Ratatui ")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP)
                .style_description_title()
                .padding(Padding::new(0, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .render(area, buf);
}

/// Use half block characters to render a logo based on the `RATATUI_LOGO` const.
///
/// The logo is rendered in three colors, one for the rat, one for the terminal, and one for the
/// rat's eye. The eye color alternates between two colors based on the selected row.
#[allow(clippy::cast_possible_truncation)]
pub fn render_logo(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let eye_color = if selected_row.is_multiple_of(2) {
        Color::rat_eye().into()
    } else {
        Color::rat_eye_alt().into()
    };
    let area = area.inner(Margin {
        vertical: 0,
        horizontal: 2,
    });
    for (y, (line1, line2)) in RATATUI_LOGO.iter().tuples().enumerate() {
        for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
            let x = area.left() + x as u16;
            let y = area.top() + y as u16;
            let cell = &mut buf[(x, y)];
            let rat_color = Color::rat().into();
            let term_color = Color::term().into();
            let border_color = Color::term_border().into();
            match (ch1, ch2) {
                ('█', '█') => {
                    cell.set_char('█');
                    cell.fg = rat_color;
                    cell.bg = rat_color;
                }
                ('█', ' ') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                }
                (' ', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                }
                ('█', 'x') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                    cell.bg = term_color;
                }
                ('x', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                    cell.bg = term_color;
                }
                ('x', 'x') => {
                    cell.set_char(' ');
                    cell.fg = term_color;
                    cell.bg = term_color;
                }
                ('█', 'e') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                    cell.bg = eye_color;
                }
                ('e', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                    cell.bg = eye_color;
                }
                ('▒', '▒') => {
                    cell.set_char('█');
                    cell.fg = border_color;
                    cell.bg = border_color;
                }
                ('▒', ' ') => {
                    cell.set_char('▀');
                    cell.fg = border_color;
                }
                (' ', '▒') => {
                    cell.set_char('▄');
                    cell.fg = border_color;
                }
                ('▒', 'x') => {
                    cell.set_char('▀');
                    cell.fg = border_color;
                    cell.bg = term_color;
                }
                ('x', '▒') => {
                    cell.set_char('▄');
                    cell.fg = border_color;
                    cell.bg = term_color;
                }
                (_, _) => {}
            };
        }
    }
}
