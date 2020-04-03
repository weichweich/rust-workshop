use regex::Regex;
use log;

pub trait Manipulator {
    fn manipulate(self: &mut Self, line: &str) -> Option<String>;
}

pub struct Printer {
    highlight: Regex,
}

impl Manipulator for Printer {
    fn manipulate(self: &mut Self, line: &str) -> Option<String> {
        log::trace!("Doing nothing...");
        None
    }
}

impl Default for Printer {
    fn default() -> Self { 
        log::trace!("New default Printer");
        Printer {
            highlight: Regex::new(".*").unwrap(),
        }
     }
}
