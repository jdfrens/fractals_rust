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
        Iteration::Outside { iterations } => outside_intensity(iterations),
    }
}

fn outside_intensity(iterations: u32) -> (f32, f32) {
    let half_iterations = 512 / 2 - 1;
    if iterations <= half_iterations {
        (scale(1.max(iterations)), 0.0)
    } else {
        (1.0, scale(iterations - half_iterations))
    }
}

fn scale(i: u32) -> f32 {
    let adjusted_i = (i - 1) as f32;
    let max_iterations = 512.0;
    2.0 * adjusted_i / max_iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blue_is_primary() {
        let cs = Blue {};
        assert_eq!(
            Color::new(0.6875, 0.6875, 1.0),
            cs.color(Iteration::Outside { iterations: 432 })
        );
    }

    #[test]
    fn green_is_primary() {
        let cs = Green {};
        assert_eq!(
            Color::new(0.6875, 1.0, 0.6875),
            cs.color(Iteration::Outside { iterations: 432 })
        );
    }

    #[test]
    fn red_is_primary() {
        let cs = Red {};
        assert_eq!(
            Color::new(1.0, 0.6875, 0.6875),
            cs.color(Iteration::Outside { iterations: 432 })
        );
    }

    #[test]
    fn test_scale() {
        assert_eq!(0.0, scale(1));
        assert_eq!(0.4921875, scale(127));
        assert_eq!(0.49609375, scale(128));
        assert_eq!(0.98828125, scale(254));
    }

    #[test]
    fn test_intensities() {
        assert_eq!(
            (0.0, 0.0),
            intensities(Iteration::Outside { iterations: 1 })
        );
        assert_eq!(
            (0.9921875, 0.0),
            intensities(Iteration::Outside { iterations: 255 })
        );
        assert_eq!(
            (1.0, 0.6875),
            intensities(Iteration::Outside { iterations: 432 })
        );
        assert_eq!(
            (1.0, 0.0),
            intensities(Iteration::Outside { iterations: 256 })
        );
        assert_eq!(
            (1.0, 1.0),
            intensities(Iteration::Outside { iterations: 512 })
        );
    }
}
