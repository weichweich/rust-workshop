use colored::*;
use log;
use regex::Regex;

pub trait Manipulator {
    fn manipulate(self: &mut Self, line: &str) -> Option<String>;
}

pub struct Printer {
    highlight: Regex,
}

impl Manipulator for Printer {
    // highlight in a color
    fn manipulate(self: &mut Self, line: &str) -> Option<String> {
        let mut my_vec: Vec<ColoredString> = vec![];
        for word in line.split_whitespace() {
            if self.highlight.is_match(word) {
                my_vec.push(word.green());
            } else {
                my_vec.push(ColoredString::from(word));
            }
        }
        Some(
            my_vec
                .iter()
                .fold(String::from(""), |mut acc, word| {
                    acc.push_str(&format!("{}", word));
                    acc.push_str(" ");
                    acc
                })
                .trim()
                .to_string(),
        )
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
    #[test]
    fn it_works() {
        println!("{}", Printer::default().manipulate("Hello World").unwrap());
    }
}
