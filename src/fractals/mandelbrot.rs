use super::escape_time::{EscapeTime, Iteration};
use num_complex::Complex;

#[derive(Debug)]
pub struct Mandelbrot {
    pub max_iterations: i64,
}

impl EscapeTime for Mandelbrot {
    fn iterate(&self, c: &Complex<f64>) -> Iteration {
        let Mandelbrot { max_iterations } = self;
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while z.norm_sqr() < 4.0 && iterations < *max_iterations {
            z = z * z + c;
            iterations = iterations + 1;
        }
        if iterations >= *max_iterations {
            Iteration::Inside {
                iterations,
                max_iterations: *max_iterations,
            }
        } else {
            Iteration::Outside {
                iterations,
                max_iterations: *max_iterations,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_inside() {
        let m = Mandelbrot {
            max_iterations: 512,
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
