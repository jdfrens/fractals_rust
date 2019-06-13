use num_complex::Complex;

#[derive(Debug)]
pub struct Job {
  pub image: Image,
}

#[derive(Debug)]
pub struct Size {
  pub width: u32,
  pub height: u32,
}

#[derive(Debug)]
pub struct Image {
  pub size: Size,
  pub upper_left: Complex<f64>,
  pub lower_right: Complex<f64>,
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
}

#[cfg(test)]
mod tests {
  use super::*;

  fn image() -> Image {
    Image {
      size: Size {
        width: 5,
        height: 5,
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
    assert_eq!(1.2, image().top())
  }
}