use super::color_scheme::ColorScheme;
use super::image::Image;

#[derive(Debug)]
pub struct Job {
    pub image: Image,
    pub color_scheme: Box<ColorScheme>,
}
