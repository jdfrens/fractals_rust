use super::color_scheme::ColorScheme;
use super::image::Image;

#[derive(Debug, PartialEq)]
pub struct Job {
  pub image: Image,
  pub color_scheme: ColorScheme,
}
