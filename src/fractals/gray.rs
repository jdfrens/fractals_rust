use ::image::Rgb;

use super::color_scheme::ColorScheme;
use super::escape_time::Iteration;

#[derive(Debug, PartialEq)]
pub struct BlackOnWhite {}

impl ColorScheme for BlackOnWhite {
    fn color(&self, iter: Iteration) -> Rgb<u8> {
        match iter {
            Iteration::Inside { .. } => Rgb([0, 0, 0]),
            Iteration::Outside { .. } => Rgb([255, 255, 255]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Gray {}

impl ColorScheme for Gray {
    fn color(&self, iter: Iteration) -> Rgb<u8> {
        match iter {
            Iteration::Inside { .. } => Rgb([0, 0, 0]),
            Iteration::Outside { iterations } => gray_scale(iterations),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WhiteOnBlack {}

impl ColorScheme for WhiteOnBlack {
    fn color(&self, iter: Iteration) -> Rgb<u8> {
        match iter {
            Iteration::Inside { .. } => Rgb([255, 255, 255]),
            Iteration::Outside { .. } => Rgb([0, 0, 0]),
        }
    }
}

fn gray_scale(iterations: u32) -> Rgb<u8> {
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
            let cs = BlackOnWhite {};
            let color = cs.color(outside(iterations));
            prop_assert_eq!(Rgb([255, 255, 255]), color);
          }

          #[test]
          fn outside_always_white(iterations in 0u32..1024)  {
            let cs = BlackOnWhite {};
            let color = cs.color(inside(iterations));
                prop_assert_eq!(Rgb([0, 0, 0]), color);
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
                let cs = Gray {};
                let color = cs.color(inside(iterations));
                prop_assert_eq!(Rgb([0, 0, 0]), color);
            }
        }

        #[test]
        fn outside_scaled_gray() {
            let cs = Gray {};

            let color = cs.color(outside(128));
            let channels = color.channels();
            assert_eq!(127, channels[0]);
            assert_eq!(127, channels[1]);
            assert_eq!(127, channels[2]);

            let color = cs.color(outside(64));
            let channels = color.channels();
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
                let cs = WhiteOnBlack {};
                let color = cs.color(inside(iterations));
                prop_assert_eq!(Rgb([255, 255, 255]), color);
            }

            #[test]
            fn outside_always_black(iterations in 0u32..1024)  {
                let cs = WhiteOnBlack {};
                let color = cs.color(outside(iterations));
                prop_assert_eq!(Rgb([0, 0, 0]), color);
            }
        }
    }
}
