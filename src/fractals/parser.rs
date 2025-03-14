use lazy_static::lazy_static;
use num_complex::Complex;
use regex::Regex;
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

#[derive(Debug, PartialEq)]
enum ParsingError {
    BadComplexNumber(String),
    BadInteger(String),
}

#[derive(Debug, PartialEq)]
enum LexingError {
    BadLexComplexNumber,
}

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
        fractal: parse_fractal(&job_yaml["fractal"]),
        image: parse_image(input_filename, &job_yaml["image"]),
        color_scheme: parse_color_scheme(&job_yaml["color_scheme"]),
    }
}

fn parse_fractal(fractal_yaml: &Yaml) -> Box<dyn EscapeTime> {
    match fractal_yaml["type"].as_str().unwrap() {
        "Julia" => {
            let max_iterations = parse_u64(&fractal_yaml["max_iterations"]).unwrap();
            let c = parse_complex(&fractal_yaml["c"]).unwrap();
            return Box::new(Julia { max_iterations, c });
        }
        "Mandelbrot" => {
            let max_iterations = parse_u64(&fractal_yaml["max_iterations"]).unwrap();
            return Box::new(Mandelbrot { max_iterations });
        }
        _ => panic!("{:?} not a valid fractal", fractal_yaml),
    }
}

fn parse_image(input_filename: &String, image_yaml: &Yaml) -> Image {
    Image {
        input_filename: input_filename.clone(),
        output_filename: build_output_filename(input_filename),
        size: parse_size(&image_yaml["size"]),
        upper_left: parse_complex(&image_yaml["upperLeft"]).unwrap(),
        lower_right: parse_complex(&image_yaml["lowerRight"]).unwrap(),
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

fn parse_u64(i64_value: &Yaml) -> Result<i64, ParsingError> {
    if let Some(poop) = i64_value.as_i64() {
        return Ok(poop);
    }
    Err(ParsingError::BadInteger(
        i64_value.as_str().unwrap().to_string(),
    ))
}

fn parse_complex(complex_value: &Yaml) -> Result<Complex<f64>, ParsingError> {
    let input = complex_value.as_str().unwrap().to_string();
    if let Ok((real, rest)) = lex_number_from_complex(input) {
        if let Ok((sign, rest)) = lex_operator_from_complex(rest) {
            if let Ok((imag, rest)) = lex_number_from_complex(rest) {
                if let Ok(rest) = lex_i_from_complex(rest) {
                    if rest.as_str() == "" {
                        return Ok(Complex::new(real, sign * imag));
                    }
                }
            }
        }
    }
    Err(ParsingError::BadComplexNumber(
        complex_value.as_str().unwrap().to_string(),
    ))
}

fn lex_number_from_complex(input: String) -> Result<(f64, String), LexingError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*((\+|\-)?(\d|\.)+)(.*)$").unwrap();
    }
    for cap in RE.captures_iter(input.as_str()) {
        if let Ok(number) = cap[1].parse::<f64>() {
            return Ok((number, cap[4].to_string()));
        } else {
            return Err(LexingError::BadLexComplexNumber);
        }
    }
    Err(LexingError::BadLexComplexNumber)
}

fn lex_operator_from_complex(input: String) -> Result<(f64, String), LexingError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*(\+|\-)(.*)$").unwrap();
    }
    for cap in RE.captures_iter(input.as_str()) {
        match &cap[1] {
            "+" => return Ok((1.0, cap[2].to_string())),
            "-" => return Ok((-1.0, cap[2].to_string())),
            _ => return Err(LexingError::BadLexComplexNumber),
        }
    }
    Err(LexingError::BadLexComplexNumber)
}

fn lex_i_from_complex(input: String) -> Result<String, LexingError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*i\s*(.*)$").unwrap();
    }
    for cap in RE.captures_iter(input.as_str()) {
        return Ok(cap[1].to_string());
    }
    Err(LexingError::BadLexComplexNumber)
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
mod parser_tests {
    use super::super::color_scheme::Color;
    use super::super::escape_time::Iteration;
    use super::*;

    #[test]
    fn test_lex_number_from_complex() {
        assert_eq!(
            Ok((321.0, "".to_string())),
            lex_number_from_complex("321".to_string())
        );
        assert_eq!(
            Ok((321.0, " ".to_string())),
            lex_number_from_complex("   321 ".to_string())
        );
        assert_eq!(
            Ok((321.0, " ".to_string())),
            lex_number_from_complex("+321 ".to_string())
        );
        assert_eq!(
            Ok((-321.0, " ".to_string())),
            lex_number_from_complex("-321 ".to_string())
        );
        assert_eq!(
            Ok((321.0, "+2i".to_string())),
            lex_number_from_complex("321+2i".to_string())
        );
        assert_eq!(
            Ok((3.14, " + 2.0i".to_string())),
            lex_number_from_complex("  3.14 + 2.0i".to_string())
        );
    }

    #[test]
    fn test_lex_number_from_complex_errors() {
        assert_eq!(
            Err(LexingError::BadLexComplexNumber),
            lex_number_from_complex("abc3".to_string())
        );
        assert_eq!(
            Err(LexingError::BadLexComplexNumber),
            lex_number_from_complex("3.14.159+2i".to_string())
        );
    }

    #[test]
    fn test_lex_operator_from_complex() {
        assert_eq!(
            Ok((1.0, "".to_string())),
            lex_operator_from_complex("+".to_string())
        );
        assert_eq!(
            Ok((-1.0, "".to_string())),
            lex_operator_from_complex("-".to_string())
        );
        assert_eq!(
            Ok((1.0, " remainder".to_string())),
            lex_operator_from_complex("   + remainder".to_string())
        );
    }

    #[test]
    fn test_lex_operator_from_complex_errors() {
        assert_eq!(
            Err(LexingError::BadLexComplexNumber),
            lex_operator_from_complex("".to_string())
        );
        assert_eq!(
            Err(LexingError::BadLexComplexNumber),
            lex_operator_from_complex("anything".to_string())
        );
        assert_eq!(
            Err(LexingError::BadLexComplexNumber),
            lex_operator_from_complex("*".to_string())
        );
        assert_eq!(
            Err(LexingError::BadLexComplexNumber),
            lex_operator_from_complex("^".to_string())
        );
    }

    #[test]
    fn test_lex_i_from_complex_errors() {
        assert_eq!(Ok("".to_string()), lex_i_from_complex("i".to_string()));
        assert_eq!(
            Ok("rest".to_string()),
            lex_i_from_complex("irest".to_string())
        );
        assert_eq!(
            Ok("".to_string()),
            lex_i_from_complex("   i    ".to_string())
        );
    }

    #[test]
    fn test_parse_complex() {
        let parse = |s: &str| -> Result<Complex<f64>, ParsingError> {
            parse_complex(&Yaml::String(s.to_string()))
        };

        assert_eq!(Ok(Complex::new(5.2, 3.8)), parse("5.2+3.8i"));
        assert_eq!(
            Ok(Complex::new(111.5, 876.222)),
            parse("   111.5   +   876.222   i    ")
        );
        assert_eq!(Ok(Complex::new(1.0, 2.0)), parse("+1+2i"));
        assert_eq!(Ok(Complex::new(1.0, -2.0)), parse("+1-2i"));
        assert_eq!(Ok(Complex::new(-1.0, 2.0)), parse("-1+2i"));
        assert_eq!(Ok(Complex::new(-1.0, -2.0)), parse("-1-2i"));
        assert_eq!(Ok(Complex::new(1.0, 2.0)), parse("1++2i"));
        assert_eq!(Ok(Complex::new(1.0, -2.0)), parse("1+-2i"));
        assert_eq!(Ok(Complex::new(1.0, -2.0)), parse("1-+2i"));
        assert_eq!(Ok(Complex::new(1.0, 2.0)), parse("1--2i"));

        assert_eq!(
            Err(ParsingError::BadComplexNumber("ab 1+2i".to_string())),
            parse("ab 1+2i")
        );
        assert_eq!(
            Err(ParsingError::BadComplexNumber("".to_string())),
            parse("")
        );
        assert_eq!(
            Err(ParsingError::BadComplexNumber("1+2i  ab".to_string())),
            parse("1+2i  ab")
        );
    }

    #[test]
    fn test_parse_u64() {
        let parse = |i| -> Result<i64, ParsingError> { parse_u64(&Yaml::Integer(i)) };

        assert_eq!(Ok(5), parse(5i64));
        assert_eq!(Ok(50), parse(50i64));
        assert_eq!(Ok(1234567890), parse(1234567890i64));
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

    #[test]
    fn test_parse_fractal() {
        let input = r#"
        fractal:
          type: Julia
          max_iterations: 376
          c: 1.0+2.0i
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let fractal = parse_fractal(&docs[0]["fractal"]);
        assert_eq!(
            format!("{fractal:?}"),
            "Julia { max_iterations: 376, c: Complex { re: 1.0, im: 2.0 } }"
        );
    }
}
