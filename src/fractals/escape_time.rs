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
