use crate::lean::LeanParsed;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
mod content_builder;

#[derive(Debug)]
pub struct Markdown {
    file_content: String,
}

impl Markdown {
    pub fn new(parsed_lean: LeanParsed) -> Self {
        let content = content_builder::make_content(parsed_lean);

        Self {
            file_content: content,
        }
    }

    pub fn create_file(&self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(format!("{}.html", file_name))?;
        file.write_all(self.file_content.as_bytes())?;
        Ok(())
    }
}
