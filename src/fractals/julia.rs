use super::escape_time::{EscapeTime, Iteration};
use num_complex::Complex;

#[cfg(test)]
use std::any::Any;

#[derive(Debug)]
pub struct Julia {
    pub max_iterations: i64,
    pub c: Complex<f64>,
}

impl EscapeTime for Julia {
    fn iterate(&self, z0: &Complex<f64>) -> Iteration {
        let Julia { max_iterations, c } = self;
        let mut z = *z0;
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
        let m = Julia {
            max_iterations: 512,
            c: Complex::new(0.0, 0.0),
        };

        assert_eq!(
            Iteration::Inside {
                iterations: 512,
                max_iterations: 512,
            },
            m.iterate(&Complex::new(0.0, 0.0))
        );
        assert_eq!(
            Iteration::Inside {
                iterations: 512,
                max_iterations: 512,
            },
            m.iterate(&Complex::new(0.2, 0.5))
        );
    }

    #[test]
    fn test_iterate_outside() {
        let m = Julia {
            max_iterations: 512,
            c: Complex::new(0.0, 0.0),
        };

        assert_eq!(
            Iteration::Outside {
                iterations: 0,
                max_iterations: 512
            },
            m.iterate(&Complex::new(2.0, 2.0))
        );
        assert_eq!(
            Iteration::Outside {
                iterations: 55,
                max_iterations: 512
            },
            m.iterate(&Complex::new(0.8, 0.6))
        );
    }
}
