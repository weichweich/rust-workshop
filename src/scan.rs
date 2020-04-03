use regex::Regex;
use std::fs::DirEntry;
use crate::manipulator::Manipulator;

pub trait Scan {
    fn scan(self: &mut Self, file: &DirEntry);
}

pub struct RegexFilter {
    filter: Regex,
    manipulator: Box<dyn Manipulator>,
}

impl Scan for RegexFilter {
    fn scan(self: &mut Self, file: &DirEntry) {
        log::trace!("Scanning nothing... {:?}", file);
        // go over lines of file?
    }
}

impl RegexFilter {
    pub fn new(filter: Regex, manipulator: Box<dyn Manipulator>) -> Self {
        Self {
            filter,
            manipulator,
        }
    }
}
