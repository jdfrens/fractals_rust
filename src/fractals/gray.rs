use ::image::Rgb;

use super::color_scheme::ColorScheme;
use super::escape_time::Iteration;

pub fn gray(color_scheme: &ColorScheme, iter: Iteration) -> Result<Rgb<u8>, ()> {
  match color_scheme {
    ColorScheme::BlackOnWhite => match iter {
      Iteration::Inside { iterations: _ } => Ok(Rgb([0, 0, 0])),
      Iteration::Outside { iterations: _ } => Ok(Rgb([255, 255, 255])),
    },
    ColorScheme::Gray => match iter {
      Iteration::Inside { iterations: _ } => Ok(Rgb([0, 0, 0])),
      Iteration::Outside { iterations } => Ok(sqrt_scale(iterations)),
    },
    ColorScheme::WhiteOnBlack => match iter {
      Iteration::Inside { iterations: _ } => Ok(Rgb([255, 255, 255])),
      Iteration::Outside { iterations: _ } => Ok(Rgb([0, 0, 0])),
    },
    &_ => Err(()),
  }
}

fn sqrt_scale(iterations: u32) -> Rgb<u8> {
  let factor = (iterations as f64 / 512.0).sqrt();
  let intensity = (factor * 255.0) as u8;
  Rgb([intensity, intensity, intensity])
}

#[cfg(test)]
mod tests {
  use super::*;

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

  mod black_on_white {
    use super::super::*;
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn inside_always_black(iterations in 0u32..1024)  {
        let color = gray(&ColorScheme::BlackOnWhite, outside(iterations));
        prop_assert_eq!(Ok(Rgb([255, 255, 255])), color);
      }

      #[test]
      fn outside_always_white(iterations in 0u32..1024)  {
        let color = gray(&ColorScheme::BlackOnWhite, inside(iterations));
            prop_assert_eq!(Ok(Rgb([0, 0, 0])), color);
      }
    }
  }

  mod gray {
    use super::super::*;
    use super::*;
    use ::image::Pixel;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn inside_always_black(iterations in 0u32..1024)  {
        let color = gray(&ColorScheme::Gray, inside(iterations));
        prop_assert_eq!(Ok(Rgb([0, 0, 0])), color);
      }
    }

    #[test]
    fn outside_scaled_gray() {
      let color_result = gray(&ColorScheme::Gray, outside(128)).unwrap();
      let channels = color_result.channels();
      assert_eq!(127, channels[0]);
      assert_eq!(127, channels[1]);
      assert_eq!(127, channels[2]);

      let color_result = gray(&ColorScheme::Gray, outside(64)).unwrap();
      let channels = color_result.channels();
      assert_eq!(90, channels[0]);
      assert_eq!(90, channels[1]);
      assert_eq!(90, channels[2]);
    }
  }

  mod white_on_black {
    use super::super::*;
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn inside_always_white(iterations in 0u32..1024)  {
        let color = gray(&ColorScheme::WhiteOnBlack, outside(iterations));
        prop_assert_eq!(Ok(Rgb([0, 0, 0])), color);
      }

      #[test]
      fn outside_always_black(iterations in 0u32..1024)  {
        let color = gray(&ColorScheme::WhiteOnBlack, inside(iterations));
            prop_assert_eq!(Ok(Rgb([255, 255, 255])), color);
      }
    }
  }
}
