use num_complex::Complex;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};

use super::color_scheme::ColorScheme;
use super::escape_time::EscapeTime;
use super::gray::{BlackOnWhite, Gray, WhiteOnBlack};
use super::image::Image;
use super::julia::Julia;
use super::mandelbrot::Mandelbrot;
use super::size::Size;
use super::warp_pov::{Blue, Green, Red};
use super::Job;

pub fn parse(input_filename: &String) -> Job {
    let mut file = File::open(input_filename).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    parse_job(input_filename, &docs[0])
}

fn parse_job(input_filename: &String, job_yaml: &Yaml) -> Job {
    Job {
        fractal: parse_fractal(input_filename, &job_yaml["fractal"]),
        image: parse_image(input_filename, &job_yaml["image"]),
        color_scheme: parse_color_scheme(&job_yaml["color_scheme"]),
    }
}

fn parse_fractal(_input_filename: &String, fractal_yaml: &Yaml) -> Box<dyn EscapeTime> {
    match fractal_yaml["type"].as_str().unwrap() {
        "Julia" => {
            let c = parse_complex(&fractal_yaml["c"]);
            return Box::new(Julia { c: c })
            },
        "Mandelbrot" => return Box::new(Mandelbrot {}),
        _ => panic!("{:?} not a valid fractal", fractal_yaml),
    }
}

fn parse_image(input_filename: &String, image_yaml: &Yaml) -> Image {
    Image {
        input_filename: input_filename.clone(),
        output_filename: build_output_filename(input_filename),
        size: parse_size(&image_yaml["size"]),
        upper_left: parse_complex(&image_yaml["upperLeft"]),
        lower_right: parse_complex(&image_yaml["lowerRight"]),
    }
}

fn build_output_filename(input_filename: &String) -> String {
    let file_stem = Path::new(input_filename).file_stem().unwrap();
    let mut output_filename = PathBuf::new();
    output_filename.push("images");
    output_filename.push(file_stem);
    output_filename.set_extension("png");
    output_filename.as_os_str().to_str().unwrap().to_string()
}

fn parse_size(size: &Yaml) -> Size {
    let size_str: &str = match size.as_str() {
        Some(s) => s,
        None => &"1024x768",
    };

    let size_vec: Vec<u32> = size_str
        .split('x')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    Size {
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

fn parse_color_scheme(color_scheme_yaml: &Yaml) -> Box<dyn ColorScheme> {
    match color_scheme_yaml["type"].as_str().unwrap() {
        "BlackOnWhite" => return Box::new(BlackOnWhite {}),
        "Blue" => return Box::new(Blue {}),
        "Gray" => return Box::new(Gray {}),
        "Green" => return Box::new(Green {}),
        // "Random" => Ok(Random {}),
        "Red" => return Box::new(Red {}),
        "WhiteOnBlack" => return Box::new(WhiteOnBlack {}),
        _ => panic!("{:?} not a valid color scheme", color_scheme_yaml),
    }
}

#[cfg(test)]
mod tests {
    use super::super::color_scheme::Color;
    use super::super::escape_time::Iteration;
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

    #[test]
    fn test_parse_image() {
        let input = r#"
      image:
        size: 512x384
        upperLeft: -2.0+1.2i
        lowerRight: 1.2+-1.2i
    "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        assert_eq!(
            Image {
                input_filename: "data/foobar.yml".to_string(),
                output_filename: "images/foobar.png".to_string(),
                size: Size {
                    width: 512,
                    height: 384
                },
                upper_left: Complex::new(-2.0, 1.2),
                lower_right: Complex::new(1.2, -1.2),
            },
            parse_image(&String::from("data/foobar.yml"), &docs[0]["image"])
        );
    }

    #[test]
    fn test_parse_image_without_size() {
        let input = r#"
      image:
        upperLeft: -2.0+1.2i
        lowerRight: 1.2+-1.2i
    "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        assert_eq!(
            Size {
                width: 1024,
                height: 768
            },
            parse_image(&String::from("data/foobar.yml"), &docs[0]["image"]).size
        );
    }

    #[test]
    fn test_parse_color_scheme() {
        let input = r#"
        color_scheme:
          type: BlackOnWhite
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let cs = parse_color_scheme(&docs[0]["color_scheme"]);
        assert_eq!(
            Color::new(0.0, 0.0, 0.0),
            cs.color(Iteration::Inside {
                iterations: 200,
                max_iterations: 512
            })
        );

        let input = r#"
            color_scheme:
              type: Green
          "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let cs = parse_color_scheme(&docs[0]["color_scheme"]);
        assert_eq!(
            Color::new(0.6875, 1.0, 0.6875),
            cs.color(Iteration::Outside {
                iterations: 432,
                max_iterations: 512
            })
        );
    }
}
