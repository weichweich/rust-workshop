use crate::parse::TextIter;
use std::{collections::HashMap, error::Error, path::Path};

pub struct Counter {
    //FIXME: Maybe there is a better data structure for the task?
    // count: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        // TODO: return a new instance of self!
        unimplemented!()
    }

    pub fn count(&mut self, word: &str) {
        // TODO: count the given word!
        unimplemented!()
    }

    pub fn count_words_in_file(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        log::trace!("Scan file: {:?}", path);
        let words = TextIter::new(path);
        words.for_each(|w|self.count(&w));
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
