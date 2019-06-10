use image;
use num_complex::Complex;

fn main() {
    let x_width = 3.0;
    let y_height = 2.0;

    let left = -0.7 - x_width / 2.0;
    let top = 0.0 + y_height / 2.0;

    let width = 768;
    let height = 512;

    let x_delta = x_width / ((width - 1) as f64);
    let y_delta = y_height / ((height - 1) as f64);

    let mut image = image::ImageBuffer::new(width as u32, height as u32);

    for row in 0..height {
        for col in 0..width {
            let mut z = Complex::new(0.0, 0.0);
            let c = Complex::new(left + col as f64 * x_delta, top - row as f64 * y_delta);
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
    image.save("fractal.png").unwrap();
}
