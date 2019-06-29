use ::image::Rgb;
use core::str::FromStr;

use super::escape_time::Iteration;

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

impl ColorScheme {
  pub fn color(&self, iter: Iteration) -> Rgb<u8> {
    match self {
      ColorScheme::BlackOnWhite => match iter {
        Iteration::Inside { iterations: _ } => Ok(Rgb([0, 0, 0])),
        Iteration::Outside { iterations: _ } => Ok(Rgb([255, 255, 255])),
      },
      ColorScheme::Blue => warp_pov(self, iter),
      ColorScheme::Green => warp_pov(self, iter),
      ColorScheme::Red => warp_pov(self, iter),
      ColorScheme::WhiteOnBlack => match iter {
        Iteration::Inside { iterations: _ } => Ok(Rgb([255, 255, 255])),
        Iteration::Outside { iterations: _ } => Ok(Rgb([0, 0, 0])),
      },
      &_ => Err(()),
    }
    .unwrap()
  }
}

// Taken from http://warp.povusers.org/Mandelbrot/
fn warp_pov(color_scheme: &ColorScheme, iter: Iteration) -> Result<Rgb<u8>, ()> {
  let intensities = intensities(iter);
  match color_scheme {
    ColorScheme::Blue => Ok(permute_blue(intensities)),
    ColorScheme::Green => Ok(permute_green(intensities)),
    ColorScheme::Red => Ok(permute_red(intensities)),
    &_ => Err(()),
  }
}

fn permute_blue((primary, secondary): (u8, u8)) -> Rgb<u8> {
  Rgb([secondary, secondary, primary])
}
fn permute_green((primary, secondary): (u8, u8)) -> Rgb<u8> {
  Rgb([secondary, primary, secondary])
}
fn permute_red((primary, secondary): (u8, u8)) -> Rgb<u8> {
  Rgb([primary, secondary, secondary])
}

fn intensities(iter: Iteration) -> (u8, u8) {
  match iter {
    Iteration::Inside { iterations: _ } => (0, 0),
    Iteration::Outside { iterations } => {
      let half_iterations = 512 / 2 - 1;
      if iterations <= half_iterations {
        (scale(1.max(iterations)), 0)
      } else {
        (1 * 255, scale(iterations - half_iterations))
      }
    }

  }
}

fn scale(i: u32) -> u8 {
  (2.0 * (i - 1) as f64 / 512 as f64 * 255.0) as u8
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
