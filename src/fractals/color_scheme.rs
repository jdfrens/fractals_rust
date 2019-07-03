use ::image::Rgb;
use core::str::FromStr;

use super::escape_time::Iteration;
use super::gray::gray;
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
      ColorScheme::BlackOnWhite => gray(self, iter),
      ColorScheme::Blue => warp_pov(self, iter),
      ColorScheme::Gray => gray(self, iter),
      ColorScheme::Green => warp_pov(self, iter),
      ColorScheme::Red => warp_pov(self, iter),
      ColorScheme::WhiteOnBlack => gray(self, iter),
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
  use ::image::Pixel;
  use proptest::prelude::*;

  fn inside(iterations: u32) -> Iteration {
    Iteration::Inside {
      iterations: iterations,
    }
  }

  fn outside(iterations: u32) -> Iteration {
    Iteration::Outside {
      iterations: iterations,
    }
  }

  proptest! {
    #[test]
    fn gray_colors_have_same_rgb_outside(iterations in 0u32..1024, index in 0usize..2)  {
      let colors_schemes = [
        ColorScheme::BlackOnWhite,
        ColorScheme::Gray,
        ColorScheme::WhiteOnBlack
      ];
      let color_scheme = &colors_schemes[index];
      let color = color_scheme.color(outside(iterations));
      let channels = color.channels();
      prop_assert_eq!(channels[0], channels[1]);
      prop_assert_eq!(channels[0], channels[2]);
    }

    #[test]
    fn gray_colors_have_same_rgb_inside(iterations in 0u32..1024, index in 0usize..2)  {
      let colors_schemes = [
        ColorScheme::BlackOnWhite,
        ColorScheme::Gray,
        ColorScheme::WhiteOnBlack
      ];
      let color_scheme = &colors_schemes[index];
      let color = color_scheme.color(inside(iterations));
      let channels = color.channels();
      prop_assert_eq!(channels[0], channels[1]);
      prop_assert_eq!(channels[0], channels[2]);
    }
  }
}
