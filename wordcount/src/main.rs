#![warn(clippy::all)]
#![deny(unsafe_code)]

mod parse;
mod counter;

use structopt::StructOpt;
use std::{error::Error, path::PathBuf};
use log;
use stderrlog;

use counter::Counter;

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

    #[structopt(short = "i", long = "input", default_value = ".")]
    paths: Vec<PathBuf>,
}

fn count_words(paths: &[PathBuf]) -> Result<Counter, Box<dyn Error>> {
    let mut counter = Counter::new();
    for path in paths {
        log::trace!("Going trough supplied path: {:?}", path);
        if path.is_dir() {
            counter.count_words_in_directory(path)?;
        } else if path.is_file() {
            counter.count_words_in_file(path)?;
        }
    }
    Ok(counter)
}

fn main() -> Result<(), Box<dyn Error>>{
    let opt = Opt::from_args();

    // Configure logger
    stderrlog::new().verbosity(opt.verbose as usize).init()?;
    log::debug!("Start {}", NAME);

    count_words(&opt.paths[..])?;

    Ok(())
}
