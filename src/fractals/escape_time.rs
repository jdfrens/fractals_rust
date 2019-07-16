use num_complex::Complex;

#[derive(Debug, PartialEq)]
pub enum Iteration {
    Inside { iterations: u32, max_iterations: u32 },
    Outside { iterations: u32, max_iterations: u32 },
}

pub fn iterate(c: &Complex<f64>) -> Iteration {
    let max_iterations  = 512;
    let mut z = Complex::new(0.0, 0.0);
    let mut iterations = 0;

    while z.norm_sqr() < 4.0 && iterations < max_iterations {
        z = z * z + c;
        iterations = iterations + 1;
    }
    if iterations >= 512 {
        Iteration::Inside { iterations, max_iterations }
    } else {
        Iteration::Outside { iterations, max_iterations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate_inside() {
        assert_eq!(
            Iteration::Inside { iterations: 512, max_iterations: 512 },
            iterate(&Complex::new(0.0, 0.0))
        );
        assert_eq!(
            Iteration::Inside { iterations: 512, max_iterations: 512 },
            iterate(&Complex::new(0.2, 0.5))
        );
    }

    #[test]
    fn test_iterate_outside() {
        assert_eq!(
            Iteration::Outside { iterations: 1, max_iterations: 512 },
            iterate(&Complex::new(2.0, 2.0))
        );
        assert_eq!(
            Iteration::Outside { iterations: 12, max_iterations: 512 },
            iterate(&Complex::new(0.2, 0.6))
        );
    }
}
