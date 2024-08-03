// NOTE: outdated, needs a refactor once regex is generalised properly.


#[derive(Debug)]
pub struct LeanParsed {
    full: Option<String>,
    params: Option<String>,
    params_type: Option<String>,
    statement: Option<String>,
    proof: Option<String>,
}

impl LeanParsed {
    pub fn new() -> Self {
        LeanParsed {
            full: None,
            params: None,
            params_type: None,
            statement: None,
            proof: None,
        }
    }

    pub fn full(mut self, full: &str) -> Self {
        self.full = Some(String::from(full));
        self
    }
    pub fn params(mut self, params: &str) -> Self {
        self.params = Some(String::from(params));
        self
    }
    pub fn params_type(mut self, params_type: &str) -> Self {
        self.params_type = Some(String::from(params_type));
        self
    }
    pub fn statement(mut self, statement: &str) -> Self {
        self.statement = Some(String::from(statement));
        self
    }
    pub fn proof(mut self, proof: &str) -> Self {
        self.proof = Some(String::from(proof));
        self
    }
    pub fn build(self) -> Self {
        LeanParsed {
            full: self.full,
            params: self.params,
            params_type: self.params_type,
            statement: self.statement,
            proof: self.proof,
        }
    }
}
