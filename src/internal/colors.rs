use raylib::color::Color;

pub const BACKGROUND_COLOR: &str = "002430";
pub const FPS_COLOR: &str = "ffffff";

pub fn get_color(hex: &str) -> Color {
    Color::from_hex(hex).unwrap()
}
