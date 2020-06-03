use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct Counter {
    //FIXME: Maybe there is a better data structure for the task?
    pub count: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        // TODO: return a new instance of self!
        Counter {
            count: HashMap::new(),
        }
    }

    pub fn count(&mut self, word: &str) {
        log::trace!("Count word: {:?}", word);
        if word.chars().any(|a| a.is_alphabetic()){
            *self.count.entry(word.to_ascii_lowercase()).or_insert(0) += 1
        }
    }

    pub fn count_words_in_file(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        log::trace!("Scan file: {:?}", path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        while reader.read_line(&mut content)? != 0 {
            for word in content.split(" ") {
                self.count(word);
            }
            content.clear();
        }
        Ok(())
    }

    pub fn count_words_in_directory(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        log::trace!("Walk directory: {:?}", path);
        for maybe_entry in path.read_dir()? {
            let path = maybe_entry?.path();
            if path.is_dir() {
                self.count_words_in_directory(&path)?;
            } else if path.is_file() {
                self.count_words_in_file(&path)?;
            }
        }
        Ok(())
    }

    pub fn sort_by_number(&self) -> Vec<(&String, &usize)> {
        let mut count_vec: Vec<_> = self.count.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        count_vec
    }

    pub fn print_most_common_words(&self, n: usize) {
        let slice = self.sort_by_number();
        for (word, n) in &slice[..n] {
            println!("'{}': {} occurences", word, n);
        }
    }
}
