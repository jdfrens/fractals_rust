use num_complex::Complex;
use serde_yaml::{Mapping, Value};
use std::fs::File;
use std::io::Read;

pub fn parse(input_filename: &String) -> super::Job {
  let mut file = File::open(input_filename).expect("Unable to open file");
  let mut contents = String::new();

  file
    .read_to_string(&mut contents)
    .expect("Unable to read file");
  let config: Mapping = serde_yaml::from_str(&contents).expect("unable to parse");
  parse_job(config)
}

pub fn parse_job(config: Mapping) -> super::Job {
  let image_config = config.get(&Value::String("image".to_string()));
  let image_config = match image_config {
    Some(ic) => ic,
    None => panic!("no image!"),
  };
  super::Job {
    image: parse_image(image_config),
  }
}

pub fn parse_image(config: &Value) -> super::Image {
  let size_value = config.get(&Value::String("size".to_string()));
  let size = parse_size(size_value);
  let upper_left_value = config.get(&Value::String("upperLeft".to_string()));
  let upper_left = parse_complex(upper_left_value);
  let lower_right_value = config.get(&Value::String("lowerRight".to_string()));
  let lower_right = parse_complex(lower_right_value);

  super::Image {
    size: size,
    upper_left: upper_left,
    lower_right: lower_right,
  }
}

fn parse_size(size_value: Option<&Value>) -> super::Size {
  let size = match size_value {
    Some(s) => s.as_str().unwrap(),
    None => panic!("no size"),
  };
  let size2: Vec<&str> = size.split('x').collect();
  let size3: Vec<i32> = size2
    .into_iter()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

  super::Size {
    width: size3[0] as u32,
    height: size3[1] as u32,
  }
}

fn parse_complex(complex_value: Option<&Value>) -> Complex<f64> {
  let complex_string = match complex_value {
    Some(ul) => ul.as_str().unwrap(),
    None => panic!("no upper left"),
  };
  let vector: Vec<&str> = complex_string.split('+').collect();
  let real: f64 = vector[0].parse().unwrap();
  let imag_length = vector[1].len();
  let imag_str = &vector[1].to_string()[0..(imag_length - 1)];
  let imag: f64 = imag_str.parse().unwrap();
  Complex::new(real, imag)
}
