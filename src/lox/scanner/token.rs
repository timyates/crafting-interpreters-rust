#[derive(Debug)]
pub enum TokenType {
    // Single chars
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    // One or two char tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier(String), String(String), Number(f64),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}