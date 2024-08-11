use raylib::color::Color;

pub const BACKGROUND_COLOR: &str = "002430";
pub const BORDER_COLOR: &str = "9c0000";

pub fn get_color(hex: &str) -> Color {
    Color::from_hex(hex).unwrap()
}
