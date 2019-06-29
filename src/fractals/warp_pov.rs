use ::image::Rgb;

use super::color_scheme::ColorScheme;
use super::escape_time::Iteration;

// Taken from http://warp.povusers.org/Mandelbrot/
pub fn warp_pov(color_scheme: &ColorScheme, iter: Iteration) -> Result<Rgb<u8>, ()> {
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


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scale() {
    assert_eq!(0, scale(1));
    assert_eq!(125, scale(127));
    assert_eq!(126, scale(128));
    // maybe a bit off
    assert_eq!(253, scale(512));
  }

  #[test]
  fn test_intensities() {
    assert_eq!((0, 0), intensities(Iteration::Outside { iterations: 1 }));
    assert_eq!(
      (253, 0),
      intensities(Iteration::Outside { iterations: 255 })
    );
    assert_eq!(
      (255, 0),
      intensities(Iteration::Outside { iterations: 256 })
    );
    assert_eq!(
      (255, 255),
      intensities(Iteration::Outside { iterations: 512 })
    );
  }
}
