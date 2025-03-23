use palette::rgb::Srgb;
use tui_theme::Color;

fn main() {
    let col: Color = "lab(50.0 100% 40%)".parse().unwrap();
    println!("{col:?}");
}
