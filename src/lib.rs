use std::fs::File;
use std::io::prelude::*;
use std::path::{Path};
use regex::Regex;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct RubyProcessor;

impl Preprocessor for RubyProcessor {
    fn name(&self) -> &str {
        "ruby"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content =
                    self.process_chapter(&chapter.content)
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

impl RubyProcessor {
    fn process_chapter(
        &self,
        raw_content: &str,
    ) -> String {
        let mut rendered_content = String::from("");
        let re = Regex::new(r"\[(?P<main>.+)\]<<(?P<ruby>.+)>>").unwrap();
        let content = re.replace_all(raw_content, "<ruby>$main<rt>$ruby</rt></ruby>");
        rendered_content.push_str(&content);
        rendered_content
    }
}


pub fn load_as_string(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    };
    string
}

#[cfg(test)]
mod tests;