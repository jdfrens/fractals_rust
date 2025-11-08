#[derive(Debug, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_creation() {
        let size = Size {
            width: 800,
            height: 600,
        };
        assert_eq!(size.width, 800);
        assert_eq!(size.height, 600);
    }

    #[test]
    fn test_size_equality() {
        let size1 = Size {
            width: 1024,
            height: 768,
        };
        let size2 = Size {
            width: 1024,
            height: 768,
        };
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_size_inequality() {
        let size1 = Size {
            width: 800,
            height: 600,
        };
        let size2 = Size {
            width: 1024,
            height: 768,
        };
        assert_ne!(size1, size2);
    }

    #[test]
    fn test_size_with_zero_dimensions() {
        let size = Size {
            width: 0,
            height: 0,
        };
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }

    #[test]
    fn test_size_debug_format() {
        let size = Size {
            width: 640,
            height: 480,
        };
        let debug_string = format!("{:?}", size);
        assert_eq!(debug_string, "Size { width: 640, height: 480 }");
    }
}
