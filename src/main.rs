#![warn(clippy::all)]
#![deny(unsafe_code)]

use regex::Regex;
use std::{error::Error, path::PathBuf};
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

    #[structopt(short = "f", long = "file", default_value = ".*")]
    file_re: String,

    #[structopt(short = "l", long = "line", default_value = ".*")]
    line_re: String,

    // impossible to match r"a^" -> nothing will be highlighted
    #[structopt(short = "h", long = "highlight", default_value = "a^")]
    highlight_re: String,

    #[structopt(short = "d", long = "dir", default_value = ".")]
    dir: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    // Configure logger
    stderrlog::new().verbosity(opt.verbose as usize).init()?;

    println!("Hello, world!");

    let re_file = Regex::new(&opt.file_re)?;
    let re_line = Regex::new(&opt.line_re)?;
    // let re_highlight = Regex::new(&opt.highlight_re)?;

    let changer: manipulator::Printer = Default::default();
    let scan = scan::RegexFilter::new(re_line, Box::new(changer));
    let mut b: Box<dyn scan::Scan> = Box::new(scan);
    read_file::find_files(opt.dir.as_path(), &re_file, &mut b)?;

    Ok(())
}
