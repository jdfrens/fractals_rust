use num_complex::Complex;
use serde_yaml;
use std::fs::File;
use std::io::Read;

pub fn parse(input_filename: &String) -> super::Job {
  let mut file = File::open(input_filename).expect("Unable to open file");
  let mut contents = String::new();

  file
    .read_to_string(&mut contents)
    .expect("Unable to read file");
  let config: serde_yaml::Mapping = serde_yaml::from_str(&contents).expect("unable to parse");
  parse_job(config)
}

pub fn parse_job(config: serde_yaml::Mapping) -> super::Job {
  let image_config = config.get(&serde_yaml::Value::String("image".to_string()));
  let image_config = match image_config {
    Some(ic) => ic,
    None => panic!("no image!"),
  };
  super::Job {
    image: parse_image(image_config),
  }
}

pub fn parse_image(config: &serde_yaml::Value) -> super::Image {
  let size = parse_size(config);
  let upper_left = config.get(&serde_yaml::Value::String("upperLeft".to_string()));
  let upper_left = parse_complex(upper_left);
  let lower_right = config.get(&serde_yaml::Value::String("lowerRight".to_string()));
  let lower_right = parse_complex(lower_right);

  super::Image {
    size: size,
    upper_left: upper_left,
    lower_right: lower_right,
  }
}

fn parse_size(config: &serde_yaml::Value) -> super::Size {
  let size = config.get(&serde_yaml::Value::String("size".to_string()));
  let size = match size {
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

fn parse_complex(s: Option<&serde_yaml::Value>) -> Complex<f64> {
  let upper_left = match s {
    Some(ul) => ul.as_str().unwrap(),
    None => panic!("no upper left"),
  };
  let upper_left2: Vec<&str> = upper_left.split('+').collect();
  let real: f64 = upper_left2[0].parse().unwrap();
  let imag_length = upper_left2[1].len();
  let imag_str = &upper_left2[1].to_string()[0..(imag_length - 1)];
  let imag: f64 = imag_str.parse().unwrap();
  Complex::new(real, imag)
}
