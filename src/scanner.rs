use crate::LiteralValue::{FValue, StringValue};
use std::{collections::HashMap, string::String};

fn is_digit(ch: char) -> bool {
    (ch as u8) >= '0' as u8 && (ch as u8) <= '9' as u8
}

fn is_alpha(ch: char) -> bool {
    let uch = ch as u8;
    (uch >= 'a' as u8 && uch <= 'z' as u8) || (uch >= 'A' as u8 && uch <= 'Z' as u8) || (ch == '_')
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn get_keywords_hashmap() ->HashMap<&'static str, TokenType> {
    HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fun", Fun),
        ("if", If),
        ("nil", Nil),
        ("or", Or),
        ("print", Print),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("var", Var),
        ("while", While),
    ])
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,

    keywords: HashMap<&'static str, TokenType>
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords_hashmap(),
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut joined_errors = "".to_string();
            for error in errors {
                joined_errors.push_str(&error);
                joined_errors.push_str("\n");
            }
            return Err(joined_errors);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.char_match('=') {
                    BangEqual
                } else {
                    Equal
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            c => {
                if is_digit(c) {
                    let _ = self.number()?;
                } else if is_alpha(c) {
                    self.identifier()
                } else {
                    return Err(format!(
                        "Unrecognised character at line {}: {}",
                        self.line, c
                    ));
                }
            }
        }

        Ok(())
    }

    fn identifier(self: &mut Self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start .. self.current];
        if let Some(token_type) = self.keywords.get(substring) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(Identifier);
        }

    }

    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();
        match value {
            Ok(value) => self.add_token_lit(Number, Some(LiteralValue::FValue(value))),
            Err(_) => return Err(format!("Couldn't parse the number: {}", substring)),
        }

        Ok(())
    }

    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated String".to_string());
        }

        self.advance();

        let value = self.source.as_bytes()[self.start + 1..self.current - 1]
            .iter()
            .map(|byt| *byt as char)
            .collect::<String>();

        self.add_token_lit(StringKing, Some(StringValue(value)));

        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != ch {
             false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let mut text = "".to_string();
        let _lit = self.source[self.start..self.current]
            .chars()
            .map(|ch| text.push(ch));

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //single char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Not,

    //one or two chars
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //literals
    Identifier,
    StringKing,
    Number,

    //keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

use TokenType::*;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: u64,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: u64,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(}{)";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, RightBrace);
        assert_eq!(scanner.tokens[2].token_type, LeftBrace);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "= != == >=";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Equal);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_string_literal() {
        let source = r#""ABC""#;
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_token();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, StringKing);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_string_literal_unterminated() {
        let source = r#""ABC"#;
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_token();

        match result {
            Err(_) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn handle_string_literal_multiline() {
        let source = "\"ABC\ndef\"";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_token().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, StringKing);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC\ndef"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_number_literals() {
        let source = "123.123\n321.0\n5";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 4);
        for _ in 0..3 {
            assert_eq!(scanner.tokens[0].token_type, Number);
        }
        match scanner.tokens[0].literal {
            Some(FValue(val)) => assert_eq!(val, 123.123),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[1].literal {
            Some(FValue(val)) => assert_eq!(val, 321.0),
            _ => panic!("Incorrect literal type"),
        }
        match scanner.tokens[2].literal {
            Some(FValue(val)) => assert_eq!(val, 5.0),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_identifier() {
        let source = "this_is_a_var = 12;";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Identifier);
        assert_eq!(scanner.tokens[1].token_type, Equal);
        assert_eq!(scanner.tokens[2].token_type, Number);
        assert_eq!(scanner.tokens[3].token_type, Semicolon);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_reserved_keywords() {
        let source = "var this_is_var = 12;\n while true {print 3}";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 12);
        assert_eq!(scanner.tokens[0].token_type, Var);
        assert_eq!(scanner.tokens[1].token_type, Identifier);
        assert_eq!(scanner.tokens[2].token_type, Equal);
        assert_eq!(scanner.tokens[3].token_type, Number);
        assert_eq!(scanner.tokens[4].token_type, Semicolon);
        assert_eq!(scanner.tokens[5].token_type, While);
        assert_eq!(scanner.tokens[6].token_type, True);
        assert_eq!(scanner.tokens[7].token_type, LeftBrace);
        assert_eq!(scanner.tokens[8].token_type, Print);
        assert_eq!(scanner.tokens[9].token_type, Number);
        assert_eq!(scanner.tokens[10].token_type, RightBrace);
        assert_eq!(scanner.tokens[11].token_type, Eof);
    }
}
