mod token;

pub struct Scanner {
    pub source: String,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        vec![]
    }
}