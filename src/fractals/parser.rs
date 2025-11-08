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
pub enum ParsingError {
    BadComplexNumber(String),
    BadInteger(String),
    BadFractal(String),
    IoError(String),
    YamlError(String),
    BadColorScheme(String),
    BadSize(String),
    MissingField(String),
}

#[derive(Debug, PartialEq)]
pub enum LexingError {
    BadLexComplexNumber,
}

pub fn parse(input_filename: &String) -> Result<Job, ParsingError> {
    let mut file = File::open(input_filename)
        .map_err(|e| ParsingError::IoError(format!("Unable to open file: {}", e)))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| ParsingError::IoError(format!("Unable to read file: {}", e)))?;
    let docs = YamlLoader::load_from_str(&contents)
        .map_err(|e| ParsingError::YamlError(format!("YAML parsing error: {:?}", e)))?;

    if docs.is_empty() {
        return Err(ParsingError::YamlError("Empty YAML document".to_string()));
    }

    parse_job(input_filename, &docs[0])
}

fn parse_job(input_filename: &String, job_yaml: &Yaml) -> Result<Job, ParsingError> {
    let fractal = parse_fractal(&job_yaml["fractal"])?;
    let image = parse_image(input_filename, &job_yaml["image"])?;
    let color_scheme = parse_color_scheme(&job_yaml["color_scheme"])?;

    Ok(Job {
        fractal,
        image,
        color_scheme,
    })
}

fn parse_fractal(fractal_yaml: &Yaml) -> Result<Box<dyn EscapeTime>, ParsingError> {
    let fractal_type = fractal_yaml["type"]
        .as_str()
        .ok_or_else(|| ParsingError::MissingField("fractal type".to_string()))?;

    match fractal_type {
        "Julia" => {
            let max_iterations_result = match fractal_yaml["max_iterations"] {
                Yaml::Integer(i) => Ok(i),
                Yaml::BadValue => Ok(128),
                _ => Err(ParsingError::BadInteger(format!(
                    "{:?}",
                    fractal_yaml["max_iterations"]
                ))),
            };
            let c = parse_complex(&fractal_yaml["c"])?;
            match max_iterations_result {
                Ok(max_iterations) => Ok(Box::new(Julia { max_iterations, c })),
                Err(e) => Err(e),
            }
        }
        "Mandelbrot" => {
            let max_iterations = match fractal_yaml["max_iterations"] {
                Yaml::Integer(i) => Ok(i),
                Yaml::BadValue => Ok(128),
                _ => Err(ParsingError::BadInteger(format!(
                    "{:?}",
                    fractal_yaml["max_iterations"]
                ))),
            }?;
            Ok(Box::new(Mandelbrot { max_iterations }))
        }
        _ => Err(ParsingError::BadFractal(format!(
            "{:?} is not a valid fractal",
            fractal_yaml
        ))),
    }
}

fn parse_image(input_filename: &String, image_yaml: &Yaml) -> Result<Image, ParsingError> {
    Ok(Image {
        input_filename: input_filename.clone(),
        output_filename: build_output_filename(input_filename)?,
        size: parse_size(&image_yaml["size"])?,
        upper_left: parse_complex(&image_yaml["upperLeft"])?,
        lower_right: parse_complex(&image_yaml["lowerRight"])?,
    })
}

fn build_output_filename(input_filename: &String) -> Result<String, ParsingError> {
    let file_stem = Path::new(input_filename).file_stem().ok_or_else(|| {
        ParsingError::IoError(format!(
            "Cannot extract filename stem from {}",
            input_filename
        ))
    })?;
    let mut output_filename = PathBuf::new();
    output_filename.push("images");
    output_filename.push(file_stem);
    output_filename.set_extension("png");
    let result = output_filename
        .as_os_str()
        .to_str()
        .ok_or_else(|| ParsingError::IoError("Cannot convert path to string".to_string()))?;
    Ok(result.to_string())
}

fn parse_size(size: &Yaml) -> Result<Size, ParsingError> {
    let size_str: &str = match size.as_str() {
        Some(s) => s,
        None => "1024x768",
    };

    let size_vec: Result<Vec<u32>, _> = size_str.split('x').map(|x| x.parse::<u32>()).collect();

    let size_vec =
        size_vec.map_err(|e| ParsingError::BadSize(format!("Invalid size number: {}", e)))?;

    if size_vec.len() != 2 {
        return Err(ParsingError::BadSize(format!(
            "Size must be in format WIDTHxHEIGHT, got: {}",
            size_str
        )));
    }

    Ok(Size {
        width: size_vec[0],
        height: size_vec[1],
    })
}

#[cfg(test)]
fn parse_u64(i64_value: &Yaml) -> Result<i64, ParsingError> {
    if let Some(i) = i64_value.as_i64() {
        Ok(i)
    } else {
        let value_str = i64_value
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{:?}", i64_value));
        Err(ParsingError::BadInteger(value_str))
    }
}

fn parse_complex(complex_value: &Yaml) -> Result<Complex<f64>, ParsingError> {
    let input = complex_value
        .as_str()
        .ok_or_else(|| ParsingError::BadComplexNumber(format!("{:?}", complex_value)))?
        .to_string();

    if let Ok((real, rest)) = lex_number_from_complex(input.clone()) {
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
    Err(ParsingError::BadComplexNumber(input))
}

fn lex_number_from_complex(input: String) -> Result<(f64, String), LexingError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*((\+|\-)?(\d|\.)+)(.*)$").unwrap();
    }
    if let Some(cap) = RE.captures(input.as_str()) {
        if let Ok(number) = cap[1].parse::<f64>() {
            Ok((number, cap[4].to_string()))
        } else {
            Err(LexingError::BadLexComplexNumber)
        }
    } else {
        Err(LexingError::BadLexComplexNumber)
    }
}

fn lex_operator_from_complex(input: String) -> Result<(f64, String), LexingError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*(\+|\-)(.*)$").unwrap();
    }
    if let Some(cap) = RE.captures(input.as_str()) {
        match &cap[1] {
            "+" => Ok((1.0, cap[2].to_string())),
            "-" => Ok((-1.0, cap[2].to_string())),
            _ => Err(LexingError::BadLexComplexNumber),
        }
    } else {
        Err(LexingError::BadLexComplexNumber)
    }
}

fn lex_i_from_complex(input: String) -> Result<String, LexingError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*i\s*(.*)$").unwrap();
    }
    if let Some(cap) = RE.captures(input.as_str()) {
        Ok(cap[1].to_string())
    } else {
        Err(LexingError::BadLexComplexNumber)
    }
}
fn parse_color_scheme(color_scheme_yaml: &Yaml) -> Result<Box<dyn ColorScheme>, ParsingError> {
    let scheme_type = color_scheme_yaml["type"]
        .as_str()
        .ok_or_else(|| ParsingError::MissingField("color_scheme type".to_string()))?;

    match scheme_type {
        "BlackOnWhite" => Ok(Box::new(BlackOnWhite {})),
        "Blue" => Ok(Box::new(Blue {})),
        "Gray" => Ok(Box::new(Gray {})),
        "Green" => Ok(Box::new(Green {})),
        // "Random" => Ok(Box::new(Random {})),
        "Red" => Ok(Box::new(Red {})),
        "WhiteOnBlack" => Ok(Box::new(WhiteOnBlack {})),
        _ => Err(ParsingError::BadColorScheme(format!(
            "{} is not a valid color scheme",
            scheme_type
        ))),
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
        assert_eq!(
            Err(ParsingError::BadInteger("Foo".to_string())),
            parse_u64(&Yaml::String("Foo".to_string()))
        );
    }

    #[test]
    fn test_parse_size() {
        let parse =
            |s: &str| -> Result<Size, ParsingError> { parse_size(&Yaml::String(s.to_string())) };

        assert_eq!(
            Ok(Size {
                width: 100,
                height: 333
            }),
            parse("100x333")
        );
        assert_eq!(
            Ok(Size {
                width: 9,
                height: 12_345
            }),
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
            Ok(Image {
                input_filename: "data/foobar.yml".to_string(),
                output_filename: "images/foobar.png".to_string(),
                size: Size {
                    width: 512,
                    height: 384
                },
                upper_left: Complex::new(-2.0, 1.2),
                lower_right: Complex::new(1.2, -1.2),
            }),
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
            parse_image(&String::from("data/foobar.yml"), &docs[0]["image"])
                .unwrap()
                .size
        );
    }

    #[test]
    fn test_parse_color_scheme() {
        let input = r#"
        color_scheme:
          type: BlackOnWhite
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let cs = parse_color_scheme(&docs[0]["color_scheme"]).unwrap();
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
        let cs = parse_color_scheme(&docs[0]["color_scheme"]).unwrap();
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
        let fractal = parse_fractal(&docs[0]["fractal"]).unwrap();
        
        // Downcast to Julia to verify the values
        let julia = fractal.as_any().downcast_ref::<Julia>().unwrap();
        assert_eq!(julia.max_iterations, 376);
        assert_eq!(julia.c, Complex::new(1.0, 2.0));

        // test default max_iterations
        let input = r#"
        fractal:
          type: Julia
          c: 1.0+2.0i
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let fractal = parse_fractal(&docs[0]["fractal"]).unwrap();
        
        let julia = fractal.as_any().downcast_ref::<Julia>().unwrap();
        assert_eq!(julia.max_iterations, 128);
        assert_eq!(julia.c, Complex::new(1.0, 2.0));
    }

    // Error handling tests

    #[test]
    fn test_parse_fractal_missing_type() {
        let input = r#"
        fractal:
          max_iterations: 376
          c: 1.0+2.0i
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_fractal(&docs[0]["fractal"]);
        assert!(matches!(
            result,
            Err(ParsingError::MissingField(ref msg)) if msg == "fractal type"
        ));
    }

    #[test]
    fn test_parse_fractal_invalid_type() {
        let input = r#"
        fractal:
          type: InvalidFractal
          max_iterations: 376
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_fractal(&docs[0]["fractal"]);
        assert!(matches!(result, Err(ParsingError::BadFractal(_))));
    }

    #[test]
    fn test_parse_fractal_julia_missing_c() {
        let input = r#"
        fractal:
          type: Julia
          max_iterations: 376
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_fractal(&docs[0]["fractal"]);
        assert!(matches!(result, Err(ParsingError::BadComplexNumber(_))));
    }

    #[test]
    fn test_parse_fractal_julia_invalid_max_iterations() {
        let input = r#"
        fractal:
          type: Julia
          max_iterations: "not a number"
          c: 1.0+2.0i
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_fractal(&docs[0]["fractal"]);
        assert!(matches!(result, Err(ParsingError::BadInteger(_))));
    }

    #[test]
    fn test_parse_fractal_mandelbrot_default_max_iterations() {
        let input = r#"
        fractal:
          type: Mandelbrot
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let fractal = parse_fractal(&docs[0]["fractal"]).unwrap();
        
        let mandelbrot = fractal.as_any().downcast_ref::<Mandelbrot>().unwrap();
        assert_eq!(mandelbrot.max_iterations, 128);
    }

    #[test]
    fn test_parse_fractal_mandelbrot_with_max_iterations() {
        let input = r#"
        fractal:
          type: Mandelbrot
          max_iterations: 512
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let fractal = parse_fractal(&docs[0]["fractal"]).unwrap();
        
        let mandelbrot = fractal.as_any().downcast_ref::<Mandelbrot>().unwrap();
        assert_eq!(mandelbrot.max_iterations, 512);
    }

    #[test]
    fn test_parse_complex_invalid_format() {
        let result = parse_complex(&Yaml::String("invalid".to_string()));
        assert_eq!(
            result,
            Err(ParsingError::BadComplexNumber("invalid".to_string()))
        );
    }

    #[test]
    fn test_parse_complex_not_a_string() {
        let result = parse_complex(&Yaml::Integer(42));
        assert!(matches!(result, Err(ParsingError::BadComplexNumber(_))));
    }

    #[test]
    fn test_parse_size_invalid_format() {
        let result = parse_size(&Yaml::String("100".to_string()));
        assert!(matches!(result, Err(ParsingError::BadSize(_))));
    }

    #[test]
    fn test_parse_size_invalid_numbers() {
        let result = parse_size(&Yaml::String("100xabc".to_string()));
        assert!(matches!(result, Err(ParsingError::BadSize(_))));
    }

    #[test]
    fn test_parse_size_too_many_dimensions() {
        let result = parse_size(&Yaml::String("100x200x300".to_string()));
        assert!(matches!(result, Err(ParsingError::BadSize(_))));
    }

    #[test]
    fn test_parse_color_scheme_missing_type() {
        let input = r#"
        color_scheme:
          foo: bar
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_color_scheme(&docs[0]["color_scheme"]);
        assert!(matches!(
            result,
            Err(ParsingError::MissingField(ref msg)) if msg == "color_scheme type"
        ));
    }

    #[test]
    fn test_parse_color_scheme_invalid_type() {
        let input = r#"
        color_scheme:
          type: InvalidScheme
      "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_color_scheme(&docs[0]["color_scheme"]);
        assert!(matches!(result, Err(ParsingError::BadColorScheme(_))));
    }

    #[test]
    fn test_parse_image_missing_upper_left() {
        let input = r#"
      image:
        size: 512x384
        lowerRight: 1.2+-1.2i
    "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_image(&String::from("data/foobar.yml"), &docs[0]["image"]);
        assert!(matches!(result, Err(ParsingError::BadComplexNumber(_))));
    }

    #[test]
    fn test_parse_image_missing_lower_right() {
        let input = r#"
      image:
        size: 512x384
        upperLeft: -2.0+1.2i
    "#;
        let docs = YamlLoader::load_from_str(input).unwrap();
        let result = parse_image(&String::from("data/foobar.yml"), &docs[0]["image"]);
        assert!(matches!(result, Err(ParsingError::BadComplexNumber(_))));
    }

    #[test]
    fn test_parse_u64_string_value() {
        let result = parse_u64(&Yaml::String("not a number".to_string()));
        assert_eq!(
            result,
            Err(ParsingError::BadInteger("not a number".to_string()))
        );
    }

    #[test]
    fn test_parse_nonexistent_file() {
        let result = parse(&String::from("/nonexistent/file.yml"));
        assert!(matches!(result, Err(ParsingError::IoError(_))));
    }
}
