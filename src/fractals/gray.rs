use super::color_scheme::{Color, ColorScheme};
use super::escape_time::Iteration;

#[derive(Debug, PartialEq)]
pub struct BlackOnWhite {}

impl ColorScheme for BlackOnWhite {
    fn color(&self, iter: Iteration) -> Color {
        match iter {
            Iteration::Inside { .. } => Color::new(0.0, 0.0, 0.0),
            Iteration::Outside { .. } => Color::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Gray {}

impl ColorScheme for Gray {
    fn color(&self, iter: Iteration) -> Color {
        match iter {
            Iteration::Inside { .. } => Color::new(0.0, 0.0, 0.0),
            Iteration::Outside { iterations } => gray_scale(iterations),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WhiteOnBlack {}

impl ColorScheme for WhiteOnBlack {
    fn color(&self, iter: Iteration) -> Color {
        match iter {
            Iteration::Inside { .. } => Color::new(1.0, 1.0, 1.0),
            Iteration::Outside { .. } => Color::new(0.0, 0.0, 0.0),
        }
    }
}

fn gray_scale(iterations: u32) -> Color {
    let intensity = (iterations as f32 / 512.0).sqrt();
    Color::new(intensity, intensity, intensity)
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
            prop_assert_eq!(Color::new(1.0, 1.0, 1.0), color);
          }

          #[test]
          fn outside_always_white(iterations in 0u32..1024)  {
            let cs = BlackOnWhite {};
            let color = cs.color(inside(iterations));
                prop_assert_eq!(Color::new(0.0, 0.0, 0.0), color);
          }
        }
    }

    mod gray {
        use super::super::*;
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn inside_always_black(iterations in 0u32..1024)  {
                let cs = Gray {};
                let color = cs.color(inside(iterations));
                prop_assert_eq!(Color::new(0.0, 0.0, 0.0), color);
            }
        }

        #[test]
        fn outside_scaled_gray() {
            let cs = Gray {};

            let color = cs.color(outside(128));
            assert_eq!(Color::new(0.5, 0.5, 0.5), color);

            let color = cs.color(outside(64));
            assert_eq!(Color::new(0.35355338, 0.35355338, 0.35355338), color);
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
                prop_assert_eq!(Color::new(1.0, 1.0, 1.0), color);
            }

            #[test]
            fn outside_always_black(iterations in 0u32..1024)  {
                let cs = WhiteOnBlack {};
                let color = cs.color(outside(iterations));
                prop_assert_eq!(Color::new(0.0, 0.0, 0.0), color);
            }
        }
    }
}
