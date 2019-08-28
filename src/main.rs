#![cfg_attr(feature = "strict", deny(warnings))]

use fractals::Job;
use std::env;

mod fractals;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let job = Job::parse(input_filename);
    job.generate();
}
