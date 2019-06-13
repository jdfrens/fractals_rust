use image;
use num_complex::Complex;
use serde_yaml;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct ImageConfig {
    size: Size,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
}
// struct Job {
//     image: ImageConfig,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let mut file = File::open(input_filename).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let config: serde_yaml::Mapping = serde_yaml::from_str(&contents).expect("unable to parse");
    let image_config = config.get(&serde_yaml::Value::String("image".to_string()));
    let image_config = match image_config {
        Some(ic) => ic,
        None => panic!("no image!"),
    };
    let size = image_config.get(&serde_yaml::Value::String("size".to_string()));
    let size = match size {
        Some(s) => s.as_str().unwrap(),
        None => panic!("no size"),
    };
    let size2: Vec<&str> = size.split('x').collect();
    let size3: Vec<i32> = size2
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let upper_left = image_config.get(&serde_yaml::Value::String("upperLeft".to_string()));
    let upper_left = match upper_left {
        Some(ul) => ul.as_str().unwrap(),
        None => panic!("no upper left"),
    };
    let upper_left2: Vec<&str> = upper_left.split('+').collect();
    let real: f64 = upper_left2[0].parse().unwrap();
    let imag_length = upper_left2[1].len();
    let imag_str = &upper_left2[1].to_string()[0..(imag_length - 1)];
    let imag: f64 = imag_str.parse().unwrap();
    let upper_left = Complex::new(real, imag);

    let lower_right = image_config.get(&serde_yaml::Value::String("lowerRight".to_string()));
    let lower_right = match lower_right {
        Some(lr) => lr.as_str().unwrap(),
        None => panic!("no lower right"),
    };
    let lower_right2: Vec<&str> = lower_right.split('+').collect();
    let real: f64 = lower_right2[0].parse().unwrap();
    let imag_length = lower_right2[1].len();
    let imag_str = &lower_right2[1].to_string()[0..(imag_length - 1)];
    let imag: f64 = imag_str.parse().unwrap();
    let lower_right = Complex::new(real, imag);

    let real_image_config = ImageConfig {
        size: Size {
            width: size3[0] as u32,
            height: size3[1] as u32,
        },
        upper_left: upper_left,
        lower_right: lower_right,
    };

    let x_width = (real_image_config.upper_left.re - real_image_config.lower_right.re).abs();
    let y_height = (real_image_config.lower_right.im - real_image_config.upper_left.im).abs();

    let left = real_image_config.upper_left.re;
    let top = real_image_config.upper_left.im;

    let x_delta = x_width / ((real_image_config.size.width - 1) as f64);
    let y_delta = y_height / ((real_image_config.size.height - 1) as f64);

    let mut image = image::ImageBuffer::new(
        real_image_config.size.width as u32,
        real_image_config.size.height as u32,
    );

    for row in 0..real_image_config.size.height {
        for col in 0..real_image_config.size.width {
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
    image.save("images/fractal.png").unwrap();
}
