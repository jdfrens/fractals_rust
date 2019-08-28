use super::color_scheme::ColorScheme;
use super::escape_time::EscapeTime;
use super::image::Image;
use super::parser;

#[derive(Debug)]
pub struct Job {
    pub fractal: Box<dyn EscapeTime>,
    pub image: Image,
    pub color_scheme: Box<dyn ColorScheme>,
}

impl Job {
    pub fn parse(filename: &String) -> Self {
        parser::parse(filename)
    }

    pub fn generate(self) {
        let Job {
            fractal,
            image,
            color_scheme,
        } = self;
        let image_buffer = image.build(|z| {
            let iter = fractal.iterate(&z);
            let color = color_scheme.color(iter);
            color.as_rgb()
        });
        image_buffer.save(&image.output_filename).unwrap();
        println!("wrote {}", image.output_filename);
    }
}
