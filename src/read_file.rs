use log;
use regex::Regex;
use std::path::Path;
use std::{fs, io};

use crate::scan::Scan;

/// returns the number of files scanned, calles scanner.scan(file)
pub fn find_files(dir: &Path, re: &Regex, scanner: &mut Box<dyn Scan>) -> io::Result<usize> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_files(&path, re, scanner);
            } else {
                scanner.scan(&entry);
            }
        }
    };
    Ok(0usize)
}
