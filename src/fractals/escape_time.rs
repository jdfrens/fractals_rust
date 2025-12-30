use num_complex::Complex;

#[cfg(test)]
use std::any::Any;

pub trait EscapeTime: std::fmt::Debug {
    fn iterate(&self, c: &Complex<f64>) -> Iteration;
    
    #[cfg(test)]
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, PartialEq)]
pub enum Iteration {
    Inside {
        iterations: i64,
        max_iterations: i64,
    },
    Outside {
        iterations: i64,
        max_iterations: i64,
    },
}

/// Escape-time algorithm: iterates z = transform(z)² + c until |z| > escape_length or max_iterations reached
pub fn escape_time<F>(
    z0: Complex<f64>,
    c: Complex<f64>,
    escape_length: f64,
    max_iterations: i64,
    transform: F,
) -> Iteration
where
    F: Fn(Complex<f64>) -> Complex<f64>,
{
    let mut z = z0;
    let mut iterations = 0;
    let escape_threshold = escape_length * escape_length;

    while z.norm_sqr() < escape_threshold && iterations < max_iterations {
        z = transform(z);
        z = z * z + c;
        iterations += 1;
    }

    if iterations >= max_iterations {
        Iteration::Inside {
            iterations,
            max_iterations,
        }
    } else {
        Iteration::Outside {
            iterations,
            max_iterations,
        }
    }
}
