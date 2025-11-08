use super::size::Size;
use ::image::{ImageBuffer, Rgb, RgbImage};
use num_complex::Complex;

#[derive(Debug, PartialEq)]
pub struct Image {
    pub input_filename: String,
    pub output_filename: String,
    pub size: Size,
    pub upper_left: Complex<f64>,
    pub lower_right: Complex<f64>,
}

impl Image {
    pub fn build(&self, fun: impl Fn(Complex<f64>) -> Rgb<u8>) -> RgbImage {
        let mut image = ImageBuffer::new(self.size.width as u32, self.size.height as u32);
        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let z = self.complex_at(col, row);
                let pixel_color = fun(z);
                let pixel = image.get_pixel_mut(col, row);
                *pixel = pixel_color;
            }
        }
        image
    }

    pub fn view_width(&self) -> f64 {
        (self.upper_left.re - self.lower_right.re).abs()
    }
    pub fn view_height(&self) -> f64 {
        (self.lower_right.im - self.upper_left.im).abs()
    }

    pub fn left(&self) -> f64 {
        self.upper_left.re
    }
    pub fn top(&self) -> f64 {
        self.upper_left.im
    }

    pub fn x_delta(&self) -> f64 {
        self.view_width() / ((self.size.width - 1) as f64)
    }
    pub fn y_delta(&self) -> f64 {
        self.view_height() / ((self.size.height - 1) as f64)
    }

    pub fn complex_at(&self, col: u32, row: u32) -> Complex<f64> {
        Complex::new(
            self.left() + col as f64 * self.x_delta(),
            self.top() - row as f64 * self.y_delta(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_eq_float::*;

    fn image() -> Image {
        Image {
            input_filename: "".to_string(),
            output_filename: "".to_string(),
            size: Size {
                width: 512,
                height: 384,
            },
            upper_left: Complex::new(-2.0, 1.2),
            lower_right: Complex::new(1.2, -1.2),
        }
    }

    #[test]
    fn test_view_width() {
        assert_eq_float!(3.2, image().view_width());
    }

    #[test]
    fn test_view_height() {
        assert_eq_float!(2.4, image().view_height());
    }

    #[test]
    fn test_left() {
        assert_eq_float!(-2.0, image().left());
    }

    #[test]
    fn test_top() {
        assert_eq_float!(1.2, image().top());
    }

    #[test]
    fn test_x_delta() {
        assert_eq_float!(0.0062622309197651665, image().x_delta());
    }

    #[test]
    fn test_y_delta() {
        assert_eq_float!(0.006266318537859007, image().y_delta());
    }

    #[test]
    fn test_complex_at() {
        let result1 = image().complex_at(5, 128);
        assert_eq_float!(-1.9686888454011742, result1.re);
        assert_eq_float!(0.39791122715404703, result1.im);

        let result2 = image().complex_at(300, 19);
        assert_eq_float!(-0.12133072407044998, result2.re);
        assert_eq_float!(1.0809399477806787, result2.im);
    }
}
