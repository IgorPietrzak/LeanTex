use crate::lean::LeanParsed;
use regex::Regex;
use std::collections::HashMap;

pub struct SymbolMap {
    map: HashMap<String, String>,
}

impl SymbolMap {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("<".to_string(), "&lt;".to_string());
        map.insert(">".to_string(), "&gt;".to_string());
        map.insert("≤".to_string(), "&le;".to_string());
        map.insert("≥".to_string(), "&ge;".to_string());
        map.insert("∈".to_string(), "&isin;".to_string());
        map.insert("∀".to_string(), "&forall;".to_string());
        map.insert("∃".to_string(), "&exist;".to_string());
        map.insert("+".to_string(), "&plus;".to_string());
        map.insert("-".to_string(), "&minus;".to_string());
        map.insert("*".to_string(), "&ast;".to_string());
        map.insert("/".to_string(), "&divide;".to_string());
        map.insert("√".to_string(), "&radic;".to_string());
        SymbolMap { map }
    }

    pub fn replace_symbols(&self, lean_statement: String) -> String {
        let mut result = lean_statement.to_string();

        for (symbol, mathml) in &self.map {
            let re = Regex::new(&regex::escape(symbol)).unwrap();
            result = re.replace_all(&result, mathml.as_str()).to_string();
        }

        result
    }
}

pub fn make_content(parsed_lean: LeanParsed) -> String {
    let content = format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Lean</title>
    <style>
      body {{
        background-color: black;
        color: hotpink;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
      }}
    </style>
  </head>
  <body>
    <h1>{:?}</h1>
    <p>Theorem documentation maybe</p>
    <math xmlns="http://www.w3.org/1998/Math/MathML">

     <p>{:?}</p> 
     <p>{:?}</p> 
     <p>{:?}</p> 
     <p>{:?}</p> 
    
     </math>
     </body>
</html>"#,
        parsed_lean.name.unwrap(),
        parsed_lean.params.unwrap(),
        parsed_lean.hypothesis.unwrap(),
        parsed_lean.statement.unwrap(),
        parsed_lean.proof.unwrap()
    );
    content
}
