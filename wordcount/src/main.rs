#![warn(clippy::all)]
#![deny(unsafe_code)]

use structopt::StructOpt;
use std::{error::Error, path::PathBuf};
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

    #[structopt(short = "d", long = "dirs", default_value = ".")]
    dir: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>>{
    let opt = Opt::from_args();

    // Configure logger
    stderrlog::new().verbosity(opt.verbose as usize).init()?;
    log::debug!("Start {}", NAME);

    Ok(())
}
