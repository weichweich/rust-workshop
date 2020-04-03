#![warn(clippy::all)]
#![deny(unsafe_code)]

use regex::Regex;
use std::{error::Error, path::Path};
use structopt::StructOpt;

mod manipulator;
mod read_file;
mod scan;

const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, StructOpt)]
#[structopt(name = NAME)]
pub struct Opt {
    #[structopt(
        short,
        long,
        parse(from_occurrences),
        help = "Sets the level of verbosity."
    )]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    // Configure logger
    stderrlog::new().verbosity(opt.verbose as usize).init()?;

    println!("Hello, world!");

    let re_file = Regex::new(r".*\.txt").unwrap();
    let re_line = Regex::new(r".*").unwrap();
    let dir = Path::new(".");

    let changer: manipulator::Printer = Default::default();
    let scan = scan::RegexFilter::new(re_line, Box::new(changer));

    read_file::find_files(&dir, re_file, Box::new(scan));

    Ok(())
}
