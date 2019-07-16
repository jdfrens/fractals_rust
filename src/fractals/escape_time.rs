use num_complex::Complex;

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

pub trait EscapeTime {
    fn iterate(&self, c: &Complex<f64>) -> Iteration;
}
