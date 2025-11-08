use super::color_scheme::{Color, ColorScheme};
use super::escape_time::Iteration;

// Computations taken from http://warp.povusers.org/Mandelbrot/

#[derive(Debug, PartialEq)]
pub struct Blue {}

#[derive(Debug, PartialEq)]
pub struct Green {}

#[derive(Debug, PartialEq)]
pub struct Red {}

impl ColorScheme for Blue {
    fn color(&self, iter: Iteration) -> Color {
        let (primary, secondary) = intensities(iter);
        Color::new(secondary, secondary, primary)
    }
}

impl ColorScheme for Green {
    fn color(&self, iter: Iteration) -> Color {
        let (primary, secondary) = intensities(iter);
        Color::new(secondary, primary, secondary)
    }
}

impl ColorScheme for Red {
    fn color(&self, iter: Iteration) -> Color {
        let (primary, secondary) = intensities(iter);
        Color::new(primary, secondary, secondary)
    }
}

fn intensities(iter: Iteration) -> (f32, f32) {
    match iter {
        Iteration::Inside { .. } => (0.0, 0.0),
        Iteration::Outside {
            iterations,
            max_iterations,
        } => outside_intensity(iterations, max_iterations),
    }
}

fn outside_intensity(iterations: i64, max_iterations: i64) -> (f32, f32) {
    let half_iterations = max_iterations / 2 - 1;
    if iterations <= half_iterations {
        (scale(1.max(iterations), max_iterations), 0.0)
    } else {
        (1.0, scale(iterations - half_iterations, max_iterations))
    }
}

fn scale(i: i64, max_iterations: i64) -> f32 {
    2.0 * (i - 1) as f32 / max_iterations as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blue_is_primary() {
        let cs = Blue {};
        assert_eq!(
            Color::new(0.6875, 0.6875, 1.0),
            cs.color(Iteration::Outside {
                iterations: 432,
                max_iterations: 512
            })
        );
    }

    #[test]
    fn test_green_is_primary() {
        let cs = Green {};
        assert_eq!(
            Color::new(0.6875, 1.0, 0.6875),
            cs.color(Iteration::Outside {
                iterations: 432,
                max_iterations: 512
            })
        );
    }

    #[test]
    fn test_red_is_primary() {
        let cs = Red {};
        assert_eq!(
            Color::new(1.0, 0.6875, 0.6875),
            cs.color(Iteration::Outside {
                iterations: 432,
                max_iterations: 512
            })
        );
    }

    #[test]
    fn test_scale() {
        assert_eq!(0.0, scale(1, 512));
        assert_eq!(0.4921875, scale(127, 512));
        assert_eq!(0.49609375, scale(128, 512));
        assert_eq!(0.98828125, scale(254, 512));
    }

    #[test]
    fn test_intensities() {
        assert_eq!(
            (0.0, 0.0),
            intensities(Iteration::Outside {
                iterations: 1,
                max_iterations: 512
            })
        );
        assert_eq!(
            (0.9921875, 0.0),
            intensities(Iteration::Outside {
                iterations: 255,
                max_iterations: 512
            })
        );
        assert_eq!(
            (1.0, 0.6875),
            intensities(Iteration::Outside {
                iterations: 432,
                max_iterations: 512
            })
        );
        assert_eq!(
            (1.0, 0.0),
            intensities(Iteration::Outside {
                iterations: 256,
                max_iterations: 512
            })
        );
        assert_eq!(
            (1.0, 1.0),
            intensities(Iteration::Outside {
                iterations: 512,
                max_iterations: 512
            })
        );
    }
}
