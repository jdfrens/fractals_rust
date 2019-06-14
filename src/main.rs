use num_complex::Complex;
use std::env;

mod fractals;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let job = fractals::parser::parse(input_filename);

    let mut image =
        image::ImageBuffer::new(job.image.size.width as u32, job.image.size.height as u32);

    for row in 0..job.image.size.height {
        for col in 0..job.image.size.width {
            let iter = iterate(&job.image, col, row);
            let pixel = image.get_pixel_mut(col, row);
            set_pixel(pixel, iter);
        }
    }
    image.save("images/fractal.png").unwrap();
}

fn iterate(image: &fractals::Image, col: u32, row: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let c = image.complex_at(col, row);
    let mut iter = 0;

    while z.norm_sqr() < 4.0 && iter < 512 {
        z = z * z + c;
        iter = iter + 1;
    }
    iter
}

fn set_pixel(pixel: &mut image::Rgb<u8>, iter: u32) {
    if iter >= 512 {
        *pixel = image::Rgb([255, 255, 255]);
    } else {
        *pixel = image::Rgb([0, 0, 0]);
    }
}
