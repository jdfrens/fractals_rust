#![cfg_attr(feature = "strict", deny(warnings))]

use fractals::Job;
use std::env;

mod fractals;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        std::process::exit(1);
    }
    
    let input_filename = &args[1];
    match Job::parse(input_filename) {
        Ok(job) => job.generate(),
        Err(e) => {
            eprintln!("Error parsing input file: {:?}", e);
            std::process::exit(1);
        }
    }
}
