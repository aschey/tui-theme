use itertools::Itertools;
use palette::{FromColor, Okhsv};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::symbols;
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Clear, LineGauge, Padding, Widget};
use time::OffsetDateTime;
use tui_theme::{Color, Style};

use crate::colors::ThemeColorsColorThemeExt;
use crate::theme::{
    AppThemeStyle, WeatherColorTheme, WeatherColorThemeExt, WeatherStyleExt as _,
    enhanced_color_support,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WeatherTab {
    pub download_progress: usize,
}

impl WeatherTab {
    /// Simulate a download indicator by decrementing the row index.
    pub fn decrease(&mut self) {
        self.download_progress = self.download_progress.saturating_sub(1);
    }

    /// Simulate a download indicator by incrementing the row index.
    pub fn increase(&mut self) {
        self.download_progress = (self.download_progress + 1).min(34);
    }
}

impl Widget for WeatherTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        Block::new().style_content().render(area, buf);

        let area = area.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        let [main, _, gauges] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(area);
        let [calendar, charts] =
            Layout::horizontal([Constraint::Length(23), Constraint::Min(0)]).areas(main);
        let [simple, _] =
            Layout::vertical([Constraint::Length(29), Constraint::Min(0)]).areas(charts);

        render_calendar(calendar, buf);
        render_simple_barchart(simple, buf);
        render_gauge(self.download_progress, gauges, buf);
    }
}

fn render_calendar(area: Rect, buf: &mut Buffer) {
    let date = OffsetDateTime::now_utc().date();
    Monthly::new(date, CalendarEventStore::today(Style::calendar_day()))
        .block(Block::new().padding(Padding::new(0, 0, 2, 0)))
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic())
        .render(area, buf);
}

fn render_simple_barchart(area: Rect, buf: &mut Buffer) {
    let data = [
        ("Sat", 76),
        ("Sun", 69),
        ("Mon", 65),
        ("Tue", 67),
        ("Wed", 65),
        ("Thu", 69),
        ("Fri", 73),
    ];
    let data = data
        .into_iter()
        .map(|(label, value)| {
            Bar::default()
                .value(value)
                // This doesn't actually render correctly as the text is too wide for the bar
                // See https://github.com/ratatui/ratatui/issues/513 for more info
                // (the demo GIFs hack around this by hacking the calculation in bars.rs)
                .text_value(format!("{value}°"))
                .style(if value > 70 {
                    Style::bar1()
                } else {
                    Style::bar2()
                })
                .value_style(if value > 70 {
                    Style::bar_value1()
                } else {
                    Style::bar_value2()
                })
                .label(label.into())
        })
        .collect_vec();
    let group = BarGroup::default().bars(&data);
    BarChart::default()
        .data(group)
        .bar_width(3)
        .bar_gap(1)
        .render(area, buf);
}

#[allow(clippy::cast_precision_loss)]
pub fn render_gauge(progress: usize, area: Rect, buf: &mut Buffer) {
    let percent = (progress * 3).min(100) as f64;

    render_line_gauge(percent, area, buf);
}

fn render_line_gauge(percent: f64, area: Rect, buf: &mut Buffer) {
    let label = if percent < 100.0 {
        format!("Downloading: {percent}%")
    } else {
        "Download Complete!".into()
    };

    let (filled_color, unfilled_color) = if enhanced_color_support() {
        let progress_value = Color::progress_value().to_rgb_fg();
        let color = Okhsv::from_color(progress_value.into_linear());
        let hue = color.hue - (percent as f32 * 0.6);
        let luma = Color::base1().luminance_bg();
        let value: f32 = Okhsv::max_value();
        // for light schemes, ensure the value isn't too bright
        let theme_mod: f32 = if luma <= 0.01 { 1.0 } else { 0.75 };
        let filled_color = Okhsv::new(hue, Okhsv::max_saturation(), value * theme_mod);
        let unfilled_color = Okhsv::new(
            filled_color.hue,
            filled_color.saturation,
            filled_color.value * 0.5,
        );
        (filled_color.into(), unfilled_color.into())
    } else {
        (Color::Magenta, Color::Reset)
    };
    LineGauge::default()
        .ratio(percent / 100.0)
        .label(label)
        .fg_line_gauge()
        .filled_style(Style::new().fg(filled_color))
        .unfilled_style(Style::new().fg(unfilled_color))
        .line_set(symbols::line::THICK)
        .render(area, buf);
}
