use super::escape_time::{escape_time, EscapeTime, Iteration};
use num_complex::Complex;

#[cfg(test)]
use std::any::Any;

#[derive(Debug)]
pub struct Mandelbrot {
    pub max_iterations: i64,
    pub escape_length: f64,
}

impl EscapeTime for Mandelbrot {
    fn iterate(&self, c: &Complex<f64>) -> Iteration {
        escape_time(Complex::new(0.0, 0.0), *c, self.escape_length, self.max_iterations, |z| z)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_inside() {
        let m = Mandelbrot {
            max_iterations: 512,
            escape_length: 2.0,
        };

        assert_eq!(
            Iteration::Inside {
                iterations: 512,
                max_iterations: 512
            },
            m.iterate(&Complex::new(0.0, 0.0))
        );
        assert_eq!(
            Iteration::Inside {
                iterations: 512,
                max_iterations: 512
            },
            m.iterate(&Complex::new(0.2, 0.5))
        );
    }

    #[test]
    fn test_iterate_outside() {
        let m = Mandelbrot {
            max_iterations: 512,
            escape_length: 2.0,
        };

        assert_eq!(
            Iteration::Outside {
                iterations: 1,
                max_iterations: 512
            },
            m.iterate(&Complex::new(2.0, 2.0))
        );
        assert_eq!(
            Iteration::Outside {
                iterations: 12,
                max_iterations: 512
            },
            m.iterate(&Complex::new(0.2, 0.6))
        );
    }
}
