const STATEMENT_TYPE: &str = r"Statement\s+";
const STATEMENT_NAME: &str = r"(\w*)\s*";
const PARAMS: &str = r"(\{(?:[^:]+\s*:\s*[^\}]+\}\s*)+)?";
const HYPOTHESIS: &str = r"(\((?:[^:]+\s*:\s*[^\)]+\)\s*)+)?";
const STATEMENT: &str = r":\s*([^:]+)\s*:=\s*by\s*";
const PROOF: &str = r"(?s:(.*))";

pub struct REGEX {
    pub pattern: String,
}

impl REGEX {
    pub fn new() -> Self {
        Self {
            pattern: format!(
                "{}{}{}{}{}{}",
                STATEMENT_TYPE, STATEMENT_NAME, PARAMS, HYPOTHESIS, STATEMENT, PROOF
            ),
        }
    }
}
