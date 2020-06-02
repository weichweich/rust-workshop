use colored::*;
use log;
use regex::{Captures, Regex};

pub trait Manipulator {
    fn manipulate(self: &mut Self, line: &str) -> Option<String>;
}

pub struct Printer {
    highlight: Regex,
}

impl Printer {
    pub fn new(highlight: Regex) -> Self {
        Self {
            highlight
        }
    }
}

impl Manipulator for Printer {
    // highlight in a color
    fn manipulate(self: &mut Self, line: &str) -> Option<String> {
        Some(String::from(
            self.highlight
                .replace_all(line, |caps: &Captures| format!("{}", caps[0].green())),
        ))
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

#[cfg(test)]
mod test {
    use super::{Manipulator, Printer};
    // use colored::{Color, ColoredString};
    #[test]
    fn it_works() {
        let highlight = Printer::default()
            .manipulate("Hello World")
            .unwrap_or(String::from(""));
        println!("{}", highlight);
        assert_ne!(highlight, "");
        // FIXME: Why is colored_str.is_play() true?
        // let colored_str = ColoredString::from(highlight.as_str());
        // assert_eq!(colored_str.fgcolor(), Some(Color::Green))
    }
}
