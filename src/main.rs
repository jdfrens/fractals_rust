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
            let mut z = Complex::new(0.0, 0.0);
            let c = job.image.complex_at(col, row);
            let mut iter = 0;

            while z.norm_sqr() < 4.0 && iter < 512 {
                z = z * z + c;
                iter = iter + 1;
            }

            let pixel = image.get_pixel_mut(col, row);
            if iter >= 512 {
                *pixel = image::Rgb([255, 255, 255]);
            } else {
                *pixel = image::Rgb([0, 0, 0]);
            }
        }
    }
    image.save("images/fractal.png").unwrap();
}
