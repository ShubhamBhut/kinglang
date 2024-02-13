pub struct Scanner {}

impl Scanner {
    pub fn new(_source: &str) -> Self {
        Self {}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String> {
        todo!()
    }
}

#[derive(Debug)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: u64,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: u64) ->Self {
        Self {
            token_type,
            lexeme, 
            literal,
            line_number
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    //single char
    LeftParen,
    Rightaren,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Start,

    //one or two chars
    Bang,
    BangEqual,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    //literals
    IDENTIFIER,
    STRING,
    NUMBER,

    //keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
