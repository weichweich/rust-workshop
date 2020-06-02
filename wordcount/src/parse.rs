use std::path::Path;

///Iterates over all words in a given text file.
pub struct TextIter {
    content: String,
}

impl TextIter {
    pub fn new(file: &Path) -> TextIter {
        //TODO: read all content from a file. 
        //FIXME: might not be smart in reality because files can be huge!
        unimplemented!()
    }
}

impl Iterator for TextIter {
    // FIXME: we are copying a lot here. might be cool to return a &str instead!
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: return next word in self.content.
        None
    }
}