use crate::manipulator::Manipulator;
use regex::Regex;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};

pub trait Scan {
    fn scan(self: &mut Self, file: &DirEntry);
}

pub struct RegexFilter {
    filter: Regex,
    manipulator: Box<dyn Manipulator>,
}

impl Scan for RegexFilter {
    fn scan(self: &mut Self, file: &DirEntry) {
        let f = File::open(file.path())
            .unwrap_or_else(|_| panic!("Failed to read file {:?}", file.path()));
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        // read a line into buffer
        while reader.read_line(&mut buffer).unwrap_or(0) != 0 {
            self.match_line(&buffer[..]);
            buffer.clear();
        }
    }
}

impl RegexFilter {
    pub fn new(filter: Regex, manipulator: Box<dyn Manipulator>) -> Self {
        Self {
            filter,
            manipulator,
        }
    }

    fn match_line(self: &mut Self, line: &str) -> Option<String> {
        if self.filter.is_match(line) {
            log::trace!("Matched line '{}'", line);
            if let Some(manipulated_str) = self.manipulator.manipulate(line) {
                log::trace!("Manipulation yields '{}'", manipulated_str);
                return Some(manipulated_str);
            } else {
                log::trace!("Manipulation failed!");
            };
        }
        None
    }
}

#[allow(unsafe_code)]
mod tests {
    use super::RegexFilter;
    use crate::manipulator::Manipulator;

    struct MockManipulator {
        called_with: &'static mut Vec<String>,
    }

    impl MockManipulator {
        fn new(calls_vector: &'static mut Vec<String>) -> MockManipulator {
            MockManipulator {
                called_with: calls_vector,
            }
        }
    }

    impl Manipulator for MockManipulator {
        fn manipulate(self: &mut Self, line: &str) -> Option<String> {
            let line = String::from(line);
            self.called_with.push(String::from(&line));
            Some(line)
        }
    }

    #[test]
    fn test_line_matching() {
        let regexp = regex::Regex::new(r"\s*(?i)i love you gabi\s*").unwrap();
        let to_match = "    I love you Gabi     ";
        static mut MATCHES: Vec<String> = Vec::new();
        unsafe {
            let manip = MockManipulator::new(&mut MATCHES);
            let mut scanner = RegexFilter::new(regexp, Box::new(manip));
            assert_eq!(Some(String::from(to_match)), scanner.match_line(&to_match));
            assert_eq!(MATCHES.len(), 1);
            assert_eq!(MATCHES.first(), Some(&String::from(to_match)))
        }
    }
    #[test]
    fn test_line_matching_negative() {
        let regexp = regex::Regex::new(r"\s*(?i)i love you gabi\s*").unwrap();
        let to_match = "    i love my gabi     ";
        static mut MATCHES: Vec<String> = Vec::new();
        unsafe {
            let manip = MockManipulator::new(&mut MATCHES);
            let mut scanner = RegexFilter::new(regexp, Box::new(manip));
            assert_eq!(None, scanner.match_line(&to_match));
            assert_eq!(MATCHES.len(), 0);
            assert_eq!(MATCHES.first(), None)
        }
    }
}
