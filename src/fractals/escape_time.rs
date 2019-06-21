use num_complex::Complex;

#[derive(Debug, PartialEq)]
pub enum Iteration {
  Inside { iterations: u32 },
  Outside { iterations: u32 },
}
