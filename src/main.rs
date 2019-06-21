use fractals::escape_time::iterate;
use fractals::parser::parse;
use image::ImageBuffer;
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
            *pixel = job.color_scheme.color(iter);
        }
    }
    image.save(&job.image.output_filename).unwrap();
    println!("wrote {}", job.image.output_filename);
}
