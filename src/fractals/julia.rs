use super::escape_time::{escape_time, EscapeTime, Iteration};
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
        escape_time(*z0, self.c, 2.0, self.max_iterations)
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
