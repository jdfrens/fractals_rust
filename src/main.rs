use fractals::escape_time::{iterate, Iteration};
use fractals::parser::parse;
use image::{ImageBuffer, Rgb};
use std::env;

mod fractals;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let job = parse(input_filename);

    let mut image = ImageBuffer::new(job.image.size.width as u32, job.image.size.height as u32);

    for row in 0..job.image.size.height {
        for col in 0..job.image.size.width {
            let iter = iterate(&job.image.complex_at(col, row));
            let pixel = image.get_pixel_mut(col, row);
            set_pixel(pixel, iter);
        }
    }
    image.save(&job.image.output_filename).unwrap();
    println!("wrote {}", job.image.output_filename);
}

fn set_pixel(pixel: &mut Rgb<u8>, iter: Iteration) {
    let color = match iter {
        Iteration::Inside { iterations: _ } => Rgb([255, 255, 255]),
        Iteration::Outside { iterations: _ } => Rgb([0, 0, 0]),
    };
    *pixel = color;
}
