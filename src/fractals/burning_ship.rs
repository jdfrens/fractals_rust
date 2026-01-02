use super::escape_time::{escape_time, EscapeTime, Iteration};
use num_complex::Complex;

#[cfg(test)]
use std::any::Any;

#[derive(Debug)]
pub struct BurningShip {
    pub max_iterations: i64,
    pub escape_length: f64,
}

impl EscapeTime for BurningShip {
    fn iterate(&self, c: &Complex<f64>) -> Iteration {
        escape_time(
            Complex::new(0.0, 0.0),
            *c,
            self.escape_length,
            self.max_iterations,
            |z| Complex::new(z.re.abs(), -z.im.abs()),
        )
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
        let bs = BurningShip {
            max_iterations: 512,
            escape_length: 2.0,
        };

        assert_eq!(
            Iteration::Inside {
                iterations: 512,
                max_iterations: 512
            },
            bs.iterate(&Complex::new(0.0, 0.0))
        );
    }

    #[test]
    fn test_iterate_outside() {
        let bs = BurningShip {
            max_iterations: 512,
            escape_length: 2.0,
        };

        assert_eq!(
            Iteration::Outside {
                iterations: 1,
                max_iterations: 512
            },
            bs.iterate(&Complex::new(2.0, 2.0))
        );
    }

    #[test]
    fn test_orientation_negative_imaginary() {
        // This test ensures the burning ship uses the correct formula:
        // z_{n+1} = (|Re(z)| - i|Im(z)|)^2 + c
        // Note the negative sign on the imaginary component.
        //
        // Test with c = 0.0 + 1.0i:
        // - With correct formula (negative imaginary): stays bounded (cycles between -1+i and 0-i)
        // - With incorrect formula (positive imaginary): escapes quickly
        let bs = BurningShip {
            max_iterations: 50,
            escape_length: 2.0,
        };

        // With correct formula (negative imaginary), this point should stay inside
        let result = bs.iterate(&Complex::new(0.0, 1.0));
        match result {
            Iteration::Inside { .. } => {
                // Good! Point stays inside with correct formula
            },
            Iteration::Outside { iterations, .. } => panic!(
                "Expected point (0.0, 1.0) to stay inside with correct formula (negative imaginary), \
                 but it escaped at iteration {}. This likely means the burning ship formula is using \
                 the wrong sign (positive imaginary), which would flip the fractal upside-down.",
                iterations
            ),
        }
    }
}
