use num_complex::Complex;
use serde_yaml;
use std::fs::File;
use std::io::Read;

pub fn parse(input_filename: &String) -> super::Job {
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

    super::Job {
        image: super::Image {
            size: super::Size {
                width: size3[0] as u32,
                height: size3[1] as u32,
            },
            upper_left: upper_left,
            lower_right: lower_right,
        },

    }
}
