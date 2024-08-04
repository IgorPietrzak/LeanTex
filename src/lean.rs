use regex::Captures;

#[derive(Debug)]
pub struct LeanParsed {
    pub full: Option<String>,
    pub name: Option<String>,
    pub params: Option<String>,
    pub hypothesis: Option<String>,
    pub statement: Option<String>,
    pub proof: Option<String>,
}

impl LeanParsed {
    pub fn new(regex_cap_groups: Captures) -> Self {
        Self {
            full: regex_cap_groups.get(0).map(|m| m.as_str().to_string()),
            name: regex_cap_groups.get(1).map(|m| m.as_str().to_string()),
            params: regex_cap_groups.get(2).map(|m| m.as_str().to_string()),
            hypothesis: regex_cap_groups.get(3).map(|m| m.as_str().to_string()),
            statement: regex_cap_groups.get(4).map(|m| m.as_str().to_string()),
            proof: regex_cap_groups.get(5).map(|m| m.as_str().to_string()),
        }
    }
}
