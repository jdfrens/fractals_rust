use num_complex::Complex;

#[derive(Debug, PartialEq)]
pub enum Iteration {
  Inside { iterations: u32 },
  Outside { iterations: u32 },
}

pub fn iterate(c: &Complex<f64>) -> Iteration {
  let mut z = Complex::new(0.0, 0.0);
  let mut iter = 0;

  while z.norm_sqr() < 4.0 && iter < 512 {
    z = z * z + c;
    iter = iter + 1;
  }
  if iter >= 512 {
    Iteration::Inside { iterations: iter }
  } else {
    Iteration::Outside { iterations: iter }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_iterate_inside() {
    assert_eq!(
      Iteration::Inside { iterations: 512 },
      iterate(&Complex::new(0.0, 0.0))
    );
    assert_eq!(
      Iteration::Inside { iterations: 512 },
      iterate(&Complex::new(0.2, 0.5))
    );
  }

  #[test]
  fn test_iterate_outside() {
    assert_eq!(
      Iteration::Outside { iterations: 1 },
      iterate(&Complex::new(2.0, 2.0))
    );
    assert_eq!(
      Iteration::Outside { iterations: 12 },
      iterate(&Complex::new(0.2, 0.6))
    );
  }
}
