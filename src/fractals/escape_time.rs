use num_complex::Complex;

pub trait EscapeTime: std::fmt::Debug {
    fn iterate(&self, c: &Complex<f64>) -> Iteration;
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
