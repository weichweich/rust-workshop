use crate::manipulator::Manipulator;
use regex::Regex;
use std::fs::DirEntry;

pub trait Scan {
    fn scan(self: &Self, file: &DirEntry);
}

pub struct RegexFilter {
    filter: Regex,
    manipulator: Box<dyn Manipulator>,
}

impl Scan for RegexFilter {
    fn scan(self: &Self, file: &DirEntry) {
        log::trace!("Scanning nothing...");
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
