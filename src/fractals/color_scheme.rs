use ::image::Rgb;
use core::str::FromStr;

use super::escape_time::Iteration;
use super::warp_pov::warp_pov;

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
