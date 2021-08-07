mod token;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<token::Token>,
    line: u32,
    start: usize,
    current: usize,
}

#[derive(Debug, Clone)]
pub struct LoxError;

pub fn init(source: &str) -> Scanner {
    Scanner {
        source,
        tokens: vec![],
        line: 1,
        start: 0,
        current: 0,
    }
}

fn digit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}

impl Scanner<'_> {
    pub fn scan_tokens(&mut self) -> Result<Vec<token::Token>, LoxError> {
        let mut had_error = false;
        while !self.at_end() {
            self.start = self.current;
            had_error |= self.scan_token()
        }
        self.add_token_type(token::TokenType::Eof);
        if had_error {
            Err(LoxError)
        } else {
            Ok(self.tokens.to_vec())
        }
    }

    fn scan_token(&mut self) -> bool {
        let c = self.advance();
        let mut had_error = false;
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

            '!' => {
                if self.lookahead('=') {
                    self.add_token_type(token::TokenType::BangEqual)
                } else {
                    self.add_token_type(token::TokenType::Bang)
                }
            }
            '=' => {
                if self.lookahead('=') {
                    self.add_token_type(token::TokenType::EqualEqual)
                } else {
                    self.add_token_type(token::TokenType::Equal)
                }
            }
            '<' => {
                if self.lookahead('=') {
                    self.add_token_type(token::TokenType::LessEqual)
                } else {
                    self.add_token_type(token::TokenType::Less)
                }
            }
            '>' => {
                if self.lookahead('=') {
                    self.add_token_type(token::TokenType::GreaterEqual)
                } else {
                    self.add_token_type(token::TokenType::Greater)
                }
            }

            '/' => {
                if self.lookahead('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_type(token::TokenType::Slash)
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => {
                if !self.string() {
                    eprintln!("[line {}] Error: Unterminated string", self.line);
                    had_error = true
                }
            }

            '0'..='9' => self.number(),

            _ => {
                eprintln!("[line {}] Error: Unexpected character", self.line);
                had_error = true
            }
        }
        had_error
    }

    fn number(&mut self) {
        while digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && digit(self.peek_next()) {
            self.advance();
        }

        while digit(self.peek()) {
            self.advance();
        }

        let slice = self.source.get(self.start..self.current);
        println!("Slicing {}", slice.unwrap());
        self.add_token_type(token::TokenType::Number(
            slice.unwrap().parse::<f64>().unwrap(),
        ))
    }

    fn string(&mut self) -> bool {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.at_end() {
            return false;
        }

        self.advance();
        let slice = self.source.get((self.start + 1)..(self.current - 1));
        self.add_token_type(token::TokenType::String(slice.unwrap().to_string()));
        true
    }

    fn peek(&mut self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn lookahead(&mut self, target: char) -> bool {
        if self.at_end() || self.source.chars().nth(self.current).unwrap() != target {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token_type(&mut self, token_type: token::TokenType) {
        self.tokens.push(token::Token {
            token_type,
            lexeme: "".to_string(),
            line: self.line,
        });
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
