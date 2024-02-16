use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::scanner::{Token, TokenType, TokenType::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_tokens {
    ($parser:ident, $($token:ident),+) => {
        {
            let mut result = false;
            {
            $( result |= $parser.match_token($token); )*
            }
            result
        }
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(self: &mut Self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(self: &mut Self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn comparison(self: &mut Self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let rhs = self.term()?;
            expr = Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            }
        }

        Ok(expr)
    }

    fn term(self: &mut Self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous();
            let rhs = self.factor()?;
            expr = Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn factor(self: &mut Self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous();
            let rhs = self.unary()?;
            expr = Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn unary(self: &mut Self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let operator = self.previous();
            let rhs = self.unary()?;

            Ok(Unary {
                operator: operator,
                right: Box::from(rhs),
            })
        } else {
            self.primary()
        }
    }

    fn primary(self: &mut Self) -> Result<Expr, String> {
        let token = self.peek();
        if self.match_token(LeftParen) {
            let expr = self.expression()?;
            let _ = self.consume(RightParen, "Expected ')'");
            Ok(Grouping {
                expression: Box::from(expr),
            })
        } else {
            self.advance();
            Ok(Literal {
                value: LiteralValue::from_token(token),
            })
        }
    }

    fn consume(self: &mut Self, token_type: TokenType, msg: &str) ->Result<(), String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            Ok(())
        } else {
            Err(msg.to_string())
        }
    }

    fn match_token(self: &mut Self, t_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == t_type {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(self: &mut Self, t_types: &[TokenType]) -> bool {
        for t_type in t_types {
            if self.match_token(t_type.clone()) {
                return true;
            }
        }
        false
    }

    fn advance(self: &mut Self) -> Token {
        let token = self.peek();
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(self: &Self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(self: &Self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(self: &Self) -> bool {
        self.peek().token_type == Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LiteralValue::IntValue, scanner::Scanner};

    #[test]
    fn handle_addition() {
        let one = Token {
            token_type: TokenType::Number,
            lexeme: "1".to_string(),
            literal: Some(IntValue(1)),
            line_number: 1,
        };
        let plus = Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line_number: 1,
        };
        let two = Token {
            token_type: TokenType::Number,
            lexeme: "2".to_string(),
            literal: Some(IntValue(2)),
            line_number: 1,
        };
        let semicolon = Token {
            token_type: TokenType::Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            line_number: 1,
        };

        let tokens = vec![one, plus, two, semicolon];
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.expression();
        let string_expr = parsed_expr.unwrap().to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn handle_comparison() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        let mut parser = Parser::new(tokens.unwrap());
        let parsed_expr = parser.expression();
        let string_expr = parsed_expr.unwrap().to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }
}
