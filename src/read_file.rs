use regex::Regex;
use std::io;
use std::path::Path;
use log;

use crate::scan::Scan;

/// returns the number of files scanned, calles scanner.scan(file)
pub fn find_files(dir: &Path, re: Regex, scanner: Box<dyn Scan>) -> io::Result<usize> {
    log::trace!("finding nothing...");
    Ok(0usize)
}
