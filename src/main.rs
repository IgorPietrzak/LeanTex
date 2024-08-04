use regex::Regex;
mod examples;
mod lean;
mod regex_expr;
fn main() {
    let pattern = regex_expr::REGEX::new().pattern;
    let re = Regex::new(&pattern).unwrap();
   }

// TODO:
// Generalise the regex more and more.
// split up params and (maybe) hypothesis patterns with further regex into
// params names and their types.
