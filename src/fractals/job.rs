use super::color_scheme::ColorScheme;
use super::escape_time::iterate;
use super::image::Image;
use super::parser;

#[derive(Debug)]
pub struct Job {
  pub image: Image,
  pub color_scheme: Box<ColorScheme>,
}

impl Job {
  pub fn parse(filename: &String) -> Self {
    parser::parse(filename)
  }

  pub fn generate(self) {
    let image = self.image.build(|z| {
      let iter = iterate(&z);
      let color = self.color_scheme.color(iter);
      color.as_rgb()
    });
    image.save(&self.image.output_filename).unwrap();
    println!("wrote {}", self.image.output_filename);
  }
}
