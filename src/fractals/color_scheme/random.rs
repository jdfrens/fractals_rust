use super::{Color, ColorScheme};
use crate::fractals::escape_time::Iteration;
use rand::Rng;

#[derive(Debug)]
pub struct Random {
    colors: [Color; 2048],
}

impl Random {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let colors = std::array::from_fn(|_| {
            Color::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
            )
        });
        Random { colors }
    }
}

impl ColorScheme for Random {
    fn color(&self, iter: Iteration) -> Color {
        match iter {
            Iteration::Inside { .. } => Color::new(0.0, 0.0, 0.0),
            Iteration::Outside { iterations, .. } => {
                let index = (iterations as usize) % self.colors.len();
                self.colors[index].clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inside(iterations: i64) -> Iteration {
        Iteration::Inside {
            iterations,
            max_iterations: 512,
        }
    }

    fn outside(iterations: i64) -> Iteration {
        Iteration::Outside {
            iterations,
            max_iterations: 512,
        }
    }

    #[test]
    fn test_random_creates_2048_colors() {
        let random = Random::new();
        assert_eq!(random.colors.len(), 2048);
    }

    #[test]
    fn test_random_color_returns_color_at_index() {
        let random = Random::new();
        
        let color0 = random.color(outside(0));
        let color2048 = random.color(outside(2048));
        assert_eq!(color0, color2048);
    }

    #[test]
    fn test_inside_always_black() {
        let random = Random::new();
        
        assert_eq!(random.color(inside(0)), Color::new(0.0, 0.0, 0.0));
        assert_eq!(random.color(inside(100)), Color::new(0.0, 0.0, 0.0));
        assert_eq!(random.color(inside(512)), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_random_color_wraps_around() {
        let random = Random::new();
        
        let color_at_100 = random.color(outside(100));
        let color_at_2148 = random.color(outside(2148)); // 2148 % 2048 = 100
        assert_eq!(color_at_100, color_at_2148);
    }
}
