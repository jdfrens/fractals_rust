use self::escape_time::Iteration;
use core::str::FromStr;
use image::Rgb;
use num_complex::Complex;

pub mod escape_time;
pub mod parser;

#[derive(Debug, PartialEq)]
pub struct Job {
  pub image: Image,
  pub color_scheme: ColorScheme,
}

#[derive(Debug, PartialEq)]
pub struct Size {
  pub width: u32,
  pub height: u32,
}

#[derive(Debug, PartialEq)]
pub struct Image {
  pub input_filename: String,
  pub output_filename: String,
  pub size: Size,
  pub upper_left: Complex<f64>,
  pub lower_right: Complex<f64>,
}

#[derive(Debug, PartialEq)]
pub enum ColorScheme {
  BlackOnWhite,
  Blue,
  Gray,
  Green,
  Random,
  Red,
  WhiteOnBlack,
}

impl Image {
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

impl ColorScheme {
  pub fn color(&self, iter: Iteration) -> Rgb<u8> {
    match self {
      ColorScheme::BlackOnWhite => match iter {
        Iteration::Inside { iterations: _ } => Ok(Rgb([0, 0, 0])),
        Iteration::Outside { iterations: _ } => Ok(Rgb([255, 255, 255])),
      },
      ColorScheme::WhiteOnBlack => match iter {
        Iteration::Inside { iterations: _ } => Ok(Rgb([255, 255, 255])),
        Iteration::Outside { iterations: _ } => Ok(Rgb([0, 0, 0])),
      },
      &_ => Err(()),
    }
    .unwrap()
  }
}

impl FromStr for ColorScheme {
  type Err = ();

  fn from_str(s: &str) -> Result<ColorScheme, ()> {
    match s {
      "BlackOnWhite" => Ok(ColorScheme::BlackOnWhite),
      "Blue" => Ok(ColorScheme::Blue),
      "Gray" => Ok(ColorScheme::Gray),
      "Green" => Ok(ColorScheme::Green),
      "Random" => Ok(ColorScheme::Random),
      "Red" => Ok(ColorScheme::Red),
      "WhiteOnBlack" => Ok(ColorScheme::WhiteOnBlack),
      _ => Err(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
    assert_eq!(3.2, image().view_width());
  }


  #[test]
  fn test_view_height() {
    assert_eq!(2.4, image().view_height());
  }

  #[test]
  fn test_left() {
    assert_eq!(-2.0, image().left());
  }

  #[test]
  fn test_top() {
    assert_eq!(1.2, image().top());
  }

  #[test]
  fn test_x_delta() {
    assert_eq!(0.0062622309197651665, image().x_delta());
  }

  #[test]
  fn test_y_delta() {
    assert_eq!(0.006266318537859007, image().y_delta());
  }

  #[test]
  fn test_complex_at() {
    assert_eq!(
      Complex::new(-1.9686888454011742, 0.39791122715404703),
      image().complex_at(5, 128)
    );
    assert_eq!(
      Complex::new(-0.12133072407044998, 1.0809399477806787),
      image().complex_at(300, 19)
    );
  }
}
