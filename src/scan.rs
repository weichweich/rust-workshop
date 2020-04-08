use crate::manipulator::Manipulator;
use regex::Regex;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};

pub trait Scan {
    fn scan(self: &mut Self, file: &DirEntry);
}

pub struct RegexFilter<T: Manipulator> {
    filter: Regex,
    manipulator: T,
}

impl<T: Manipulator> Scan for RegexFilter<T> {
    fn scan(self: &mut Self, file: &DirEntry) {
        let f = File::open(file.path())
            .unwrap_or_else(|_| panic!("Failed to read file {:?}", file.path()));
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        // read a line into buffer
        while reader.read_line(&mut buffer).unwrap_or(0) != 0 {
            self.match_line(buffer.as_str());
            buffer.clear();
        }
    }
}

impl<T: Manipulator> RegexFilter<T> {
    pub fn new(filter: Regex, manipulator: T) -> Self {
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

mod tests {
    use super::RegexFilter;
    use crate::manipulator::Manipulator;

    struct MockManipulator {
        called_with: Vec<String>,
    }

    impl MockManipulator {
        fn new() -> MockManipulator {
            MockManipulator {
                called_with: Vec::new(),
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
        let mut scanner = RegexFilter::new(regexp, MockManipulator::new());
        assert_eq!(Some(String::from(to_match)), scanner.match_line(&to_match));
        assert_eq!(scanner.manipulator.called_with.len(), 1);
        assert_eq!(
            scanner.manipulator.called_with.first(),
            Some(&String::from(to_match))
        );
    }
    #[test]
    fn test_line_matching_negative() {
        let regexp = regex::Regex::new(r"\s*(?i)i love you gabi\s*").unwrap();
        let to_match = "    i love my gabi     ";
        let mut scanner = RegexFilter::new(regexp, MockManipulator::new());
        assert_eq!(None, scanner.match_line(&to_match));
        assert_eq!(scanner.manipulator.called_with.len(), 0);
        assert_eq!(scanner.manipulator.called_with.first(), None)
    }
}
