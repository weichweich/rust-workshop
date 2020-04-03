#![warn(clippy::all)]

use regex::Regex;
use std::path::Path;
mod manipulator;
mod read_file;
mod scan;

fn main() {
    println!("Hello, world!");

    let re_file = Regex::new(r".*\.txt").unwrap();
    let re_line = Regex::new(r".*").unwrap();
    let dir = Path::new(".");

    let changer: manipulator::Printer = Default::default();
    let scan = scan::RegexFilter::new(re_line, Box::new(changer));

    read_file::find_files(&dir, &re_file, &Box::new(scan));
}
