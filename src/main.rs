extern crate leantex;
use leantex::{examples, lean, mathml, regex_expr};
use regex::Regex;

fn main() {
    let pattern = regex_expr::REGEX::default().pattern;
    let re = Regex::new(&pattern).unwrap();
    if let Some(caps) = re.captures(examples::LE_ANTISYMM) {
        let lean = lean::LeanParsed::new(caps);
        let html = mathml::Markdown::new(lean);
        match html.create_file("lean") {
            Ok(()) => println!("File created"),
            Err(e) => println!("Error: {e}"),
        }
    } else {
        println!("Lean statement not found!");
    }
}
