#![warn(clippy::all)]
#![deny(unsafe_code)]

use structopt::StructOpt;
use std::{error::Error, path::{Path, PathBuf}, fs::{ReadDir, DirEntry, FileType}};
use log;
use stderrlog;

mod parse;
mod counter;

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

fn count_words_in_file(path: &Path) -> Result<(), Box<dyn Error>> {
    log::trace!("Scan file: {:?}", path);

    Ok(())
}

fn count_words_in_directory(path: &Path) -> Result<(), Box<dyn Error>> {
    log::trace!("Walk directory: {:?}", path);
    for maybe_entry in path.read_dir()? {
        let path = maybe_entry?.path();
        if path.is_dir() {
            count_words_in_directory(&path)?;
        } else if path.is_file() {
            count_words_in_file(&path)?;
        }
    }
    Ok(())
}

fn count_words(paths: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    for path in paths {
        log::trace!("Going trough supplied path: {:?}", path);
        if path.is_dir() {
            count_words_in_directory(path)?;
        } else if path.is_file() {
            count_words_in_file(path)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let opt = Opt::from_args();

    // Configure logger
    stderrlog::new().verbosity(opt.verbose as usize).init()?;
    log::debug!("Start {}", NAME);

    count_words(&opt.paths[..])?;

    Ok(())
}
