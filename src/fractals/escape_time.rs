use num_complex::Complex;

pub trait EscapeTime: std::fmt::Debug {
    fn iterate(&self, c: &Complex<f64>) -> Iteration;
}

#[derive(Debug, PartialEq)]
pub enum Iteration {
    Inside {
        iterations: u32,
        max_iterations: u32,
    },
    Outside {
        iterations: u32,
        max_iterations: u32,
    },
}
