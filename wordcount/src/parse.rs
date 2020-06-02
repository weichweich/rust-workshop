use std::{error::Error, fs::File, io::{BufRead, BufReader}, path::Path};

///Iterates over all words in a given text file.
pub struct TextIter {
    reader: BufReader<File>,
    cur_line: Option<Vec<String>>,
    index: usize,
}

impl TextIter {
    pub fn new(path: &Path) -> Result<TextIter, Box<dyn Error>> {
        //TODO: read all content from a file.
        //FIXME: might not be smart in reality because files can be huge!
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(TextIter {
            reader,
            cur_line: None,
            index: 0,
        })
    }
}

impl Iterator for TextIter {
    // FIXME: we are copying a lot here. might be cool to return a &str instead!
    type Item = Result<String, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(words) = &self.cur_line {
            if let Some(word) = words.get(self.index) {
                self.index += 1;
                return Some(Ok(word.clone()));
            }
        }
        let mut content: String = String::new();
        match self.reader.read_line(&mut content) {
            Ok(0) => None,
            Ok(_) => {
                let mut word_iter = content.split(" ").map(|s|s.to_owned());
                let cur_words = word_iter.next();
                let words: Vec<String> = word_iter.collect();
                self.cur_line = Some(words);
                self.index = 0;
                cur_words.map(|w| Ok(w))
            }
             Err(e) => Some(Err(e.into())),
        }
    }
}
