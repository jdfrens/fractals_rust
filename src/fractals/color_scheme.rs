use ::image::Rgb;

use super::escape_time::Iteration;

#[derive(Debug, PartialEq)]
pub struct Random {}

pub trait ColorScheme: std::fmt::Debug {
  fn color(&self, iter: Iteration) -> Rgb<u8>;
}
