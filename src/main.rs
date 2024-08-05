use regex::Regex;
mod examples;
mod lean;
mod mathml;
mod regex_expr;

fn main() {
    let pattern = regex_expr::REGEX::new().pattern;
    let re = Regex::new(&pattern).unwrap();
    if let Some(caps) = re.captures(examples::LE_ANTISYMM) {
      let lean = lean::LeanParsed::new(caps);
      let html = mathml::Markdown::new(lean);
      match html.create_file("bug") {
        Ok(()) => println!("File created"),
        Err(e) => println!("Error: {e}")
      }
    }

    
}
