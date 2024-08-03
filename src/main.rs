use regex::Regex;
mod examples;
mod lean;
fn main() {
    // NOTE: params and hypothesis patterns are both optional
    let statement_type = r"Statement\s+";
    let statement_name = r"(\w*)\s*";
    let params = r"(\{(?:[^:]+\s*:\s*[^\}]+\}\s*)+)";
    let hypothesis = r"(\((?:[^:]+\s*:\s*[^\)]+\)\s*)+)";
    let statement = r":\s*([^:]+)\s*:=\s*by\s*";
    let proof = r"(?s:(.*))";

    // Combine sub-patterns into a single regex pattern
    let combined_pattern = format!(
        "{}{}{}{}{}{}",
        statement_type, statement_name, params, hypothesis, statement, proof
    );

    let re = Regex::new(&combined_pattern).unwrap();

    if let Some(caps) = re.captures(examples::LE_ANTISYMM) {
        println!("0 :{}", caps.get(0).unwrap().as_str());
        println!("1: {}", caps.get(1).unwrap().as_str());
        println!("2: {}", caps.get(2).unwrap().as_str());
        println!("3: {}", caps.get(3).unwrap().as_str());
        println!("4: {}", caps.get(4).unwrap().as_str());
        println!("5: {}", caps.get(5).unwrap().as_str());
    } else {
        println!("No match found.");
    }
}

// TODO:
// Generalise the regex more and more.
// split up params and (maybe) hypothesis patterns with further regex into
// params names and their types.
