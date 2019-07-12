use ::image::Rgb;

use super::color_scheme::ColorScheme;
use super::escape_time::Iteration;

// Computations taken from http://warp.povusers.org/Mandelbrot/

#[derive(Debug, PartialEq)]
pub struct Blue {}

#[derive(Debug, PartialEq)]
pub struct Green {}

#[derive(Debug, PartialEq)]
pub struct Red {}

impl ColorScheme for Blue {
    fn color(&self, iter: Iteration) -> Rgb<u8> {
        let (primary, secondary) = intensities(iter);
        Rgb([secondary, secondary, primary])
    }
}

impl ColorScheme for Green {
    fn color(&self, iter: Iteration) -> Rgb<u8> {
        let (primary, secondary) = intensities(iter);
        Rgb([secondary, primary, secondary])
    }
}

impl ColorScheme for Red {
    fn color(&self, iter: Iteration) -> Rgb<u8> {
        let (primary, secondary) = intensities(iter);
        Rgb([primary, secondary, secondary])
    }
}

fn intensities(iter: Iteration) -> (u8, u8) {
    match iter {
        Iteration::Inside { .. } => (0, 0),
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
    fn blue_is_primary() {
        let cs = Blue {};
        assert_eq!(
            Rgb([175, 175, 255]),
            cs.color(Iteration::Outside { iterations: 432 })
        );
    }

    #[test]
    fn green_is_primary() {
        let cs = Green {};
        assert_eq!(
            Rgb([175, 255, 175]),
            cs.color(Iteration::Outside { iterations: 432 })
        );
    }

    #[test]
    fn red_is_primary() {
        let cs = Red {};
        assert_eq!(
            Rgb([255, 175, 175]),
            cs.color(Iteration::Outside { iterations: 432 })
        );
    }

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
            (255, 175),
            intensities(Iteration::Outside { iterations: 432 })
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
