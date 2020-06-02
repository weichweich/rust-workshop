use std::{collections::HashMap, error::Error, path::Path, io::{BufRead, BufReader}, fs::File};

pub struct Counter {
    //FIXME: Maybe there is a better data structure for the task?
    // count: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        // TODO: return a new instance of self!
        Counter {}
    }

    pub fn count(&mut self, word: &str) {
        log::trace!("Count word: {:?}", word);
        // TODO: count the given word!
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
}
