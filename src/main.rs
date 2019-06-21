use fractals::parser::parse;
use image::{ImageBuffer, Rgb};
use num_complex::Complex;
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

fn iterate(c: &Complex<f64>) -> Iteration {
    let mut z = Complex::new(0.0, 0.0);
    let mut iter = 0;

    while z.norm_sqr() < 4.0 && iter < 512 {
        z = z * z + c;
        iter = iter + 1;
    }
    iter
}

fn set_pixel(pixel: &mut Rgb<u8>, iter: u32) {
    if iter >= 512 {
        *pixel = Rgb([255, 255, 255]);
    } else {
        *pixel = Rgb([0, 0, 0]);
    }
}
