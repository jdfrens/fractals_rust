use ::image::Rgb;

use super::escape_time::Iteration;

pub trait ColorScheme: std::fmt::Debug {
    fn color(&self, iter: Iteration) -> Color;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(0.5, 0.6, 0.7);
        assert_eq!(color.red, 0.5);
        assert_eq!(color.green, 0.6);
        assert_eq!(color.blue, 0.7);
    }

    #[test]
    fn test_color_equality() {
        let color1 = Color::new(1.0, 0.0, 0.5);
        let color2 = Color::new(1.0, 0.0, 0.5);
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_color_inequality() {
        let color1 = Color::new(1.0, 0.0, 0.0);
        let color2 = Color::new(0.0, 1.0, 0.0);
        assert_ne!(color1, color2);
    }

    #[test]
    fn test_as_rgb_black() {
        let color = Color::new(0.0, 0.0, 0.0);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([0, 0, 0]));
    }

    #[test]
    fn test_as_rgb_white() {
        let color = Color::new(1.0, 1.0, 1.0);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([255, 255, 255]));
    }

    #[test]
    fn test_as_rgb_red() {
        let color = Color::new(1.0, 0.0, 0.0);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([255, 0, 0]));
    }

    #[test]
    fn test_as_rgb_green() {
        let color = Color::new(0.0, 1.0, 0.0);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([0, 255, 0]));
    }

    #[test]
    fn test_as_rgb_blue() {
        let color = Color::new(0.0, 0.0, 1.0);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([0, 0, 255]));
    }

    #[test]
    fn test_as_rgb_gray() {
        let color = Color::new(0.5, 0.5, 0.5);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([127, 127, 127]));
    }

    #[test]
    fn test_as_rgb_mixed_values() {
        let color = Color::new(0.25, 0.5, 0.75);
        let rgb = color.as_rgb();
        assert_eq!(rgb, Rgb([63, 127, 191]));
    }

    #[test]
    fn test_color_debug_format() {
        let color = Color::new(0.1, 0.2, 0.3);
        let debug_string = format!("{:?}", color);
        assert_eq!(debug_string, "Color { red: 0.1, green: 0.2, blue: 0.3 }");
    }
}
