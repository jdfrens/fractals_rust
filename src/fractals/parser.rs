use num_complex::Complex;
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

pub fn parse(input_filename: &String) -> super::Job {
  let mut file = File::open(input_filename).expect("Unable to open file");
  let mut contents = String::new();

  file
    .read_to_string(&mut contents)
    .expect("Unable to read file");
  let docs = YamlLoader::load_from_str(&contents).unwrap();
  parse_job(&docs[0])
}

pub fn parse_job(job_yaml: &Yaml) -> super::Job {
  super::Job {
    image: parse_image(&job_yaml["image"]),
  }
}

pub fn parse_image(image_yaml: &Yaml) -> super::Image {
  let size = parse_size(&image_yaml["size"]);
  let upper_left = parse_complex(&image_yaml["upperLeft"]);
  let lower_right = parse_complex(&image_yaml["lowerRight"]);

  super::Image {
    size: size,
    upper_left: upper_left,
    lower_right: lower_right,
  }
}

fn parse_size(size: &Yaml) -> super::Size {
  let size_vec: Vec<u32> = size
    .as_str()
    .unwrap()
    .split('x')
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  super::Size {
    width: size_vec[0],
    height: size_vec[1],
  }
}

fn parse_complex(complex_value: &Yaml) -> Complex<f64> {
  let complex_vec: Vec<f64> = complex_value
    .as_str()
    .unwrap()
    .replace("i", "")
    .split('+')
    .map(|x| x.parse::<f64>().unwrap())
    .collect();

  Complex::new(complex_vec[0], complex_vec[1])
}

#[cfg(test)]
mod tests {
  use super::super::*;
  use super::*;

  #[test]
  fn test_parse_complex() {
    let parse = |s: &str| -> Complex<f64> { parse_complex(&Yaml::String(s.to_string())) };

    assert_eq!(Complex::new(5.2, 3.8), parse("5.2+3.8i"));
    assert_eq!(Complex::new(111.5, 876.222), parse("111.5+876.222i"));
    assert_eq!(Complex::new(1.0, 2.0), parse("1+2i"));
    assert_eq!(Complex::new(-5.2, 3.8), parse("-5.2+3.8i"));
    assert_eq!(Complex::new(5.2, -3.8), parse("5.2+-3.8i"));
    assert_eq!(Complex::new(-5.2, -3.8), parse("-5.2+-3.8i"));
  }


  #[test]
  fn test_parse_size() {
    let parse = |s: &str| -> Size { parse_size(&Yaml::String(s.to_string())) };

    assert_eq!(
      Size {
        width: 100,
        height: 333
      },
      parse("100x333")
    );
    assert_eq!(
      Size {
        width: 9,
        height: 12_345
      },
      parse("9x12345")
    );
  }
}
