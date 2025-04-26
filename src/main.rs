#![cfg_attr(feature = "strict", deny(warnings))]

use fractals::Job;
use std::env;

mod fractals;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    match Job::parse(input_filename) {
        Ok(job) => job.generate(),
        Err(e) => panic!("Error: {:?}", e),
    }
}
