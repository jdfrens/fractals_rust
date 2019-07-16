use ::image::Rgb;

use super::escape_time::Iteration;

#[derive(Debug, PartialEq)]
pub struct Random {}

#[derive(Debug, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        let Color { red, green, blue } = self;
        Rgb([
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
        ])
    }
}

pub trait ColorScheme: std::fmt::Debug {
    fn color(&self, iter: Iteration) -> Color;
}
