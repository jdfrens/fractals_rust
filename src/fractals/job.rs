use super::color_scheme::ColorScheme;
use super::escape_time::EscapeTime;
use super::image::Image;
use super::mandelbrot::Mandelbrot;
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
        let m = Mandelbrot {};
        let image = self.image.build(|z| {
            let iter = m.iterate(&z);
            let color = self.color_scheme.color(iter);
            color.as_rgb()
        });
        image.save(&self.image.output_filename).unwrap();
        println!("wrote {}", self.image.output_filename);
    }
}
