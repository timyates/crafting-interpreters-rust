use crate::lox::scanner::token::TokenType;

mod token;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<token::Token>,
    line: u32,
    start: usize,
    current: usize,
}

pub fn init(source: &str) -> Scanner {
    Scanner { source, tokens: vec![], line: 1, start: 0, current: 0 }
}

impl Scanner<'_> {
    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.add_token_type(token::TokenType::Eof);
        self.tokens.to_vec()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_type(token::TokenType::LeftParen),
            ')' => self.add_token_type(token::TokenType::RightParen),
            '{' => self.add_token_type(token::TokenType::LeftBrace),
            '}' => self.add_token_type(token::TokenType::RightBrace),
            ',' => self.add_token_type(token::TokenType::Comma),
            '.' => self.add_token_type(token::TokenType::Dot),
            '-' => self.add_token_type(token::TokenType::Minus),
            '+' => self.add_token_type(token::TokenType::Plus),
            ';' => self.add_token_type(token::TokenType::SemiColon),
            '*' => self.add_token_type(token::TokenType::Star),
            _ => ()
        }
    }

    fn add_token_type(&mut self, token_type: TokenType) {
        self.tokens.push(token::Token { token_type, lexeme: "".to_string(), line: self.line });
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }
}