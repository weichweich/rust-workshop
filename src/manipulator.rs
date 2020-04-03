use regex::Regex;

pub trait Manipulator {
    fn manipulate(self: &mut Self, line: &str) -> Option<String>;
}

pub struct Printer {
    highlight: Regex,
}

impl Manipulator for Printer {
    fn manipulate(self: &mut Self, line: &str) -> Option<String> {
        None
    }
}

impl Default for Printer {
    fn default() -> Self { 
        Printer {
            highlight: Regex::new(".*").unwrap(),
        }
     }
}
