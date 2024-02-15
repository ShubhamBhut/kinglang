use crate::scanner::{Token, TokenType::*, TokenType};
use crate::expr::{Expr, Expr::*, LiteralValue};

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
    fn new(tokens: Vec<Token>) ->Self {
        Self { tokens, current: 0 }
    }

    fn expression(self: &mut Self) ->Expr {
        self.equality()
    }

    fn equality(self: &mut Self) ->Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = Binary {
                left: Box::from(expr),
                operator: operator.clone(),
                right: Box::from(rhs),
            };
        }

        expr
    }

    fn comparison(self: &mut Self) ->Expr {
        let mut expr = self.term();

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let rhs = self.term();
            expr = Binary { left: Box::from(expr), operator:operator, right: Box::from(rhs) }
        }

        expr
    }

    fn term(self: &mut Self) ->Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[Minus, Plus]) {
            let operator = self.previous();
            let rhs = self.factor();
            expr = Binary { left: Box::from(expr), operator: operator , right: Box::from(rhs) };
        }

        expr
    }

    fn factor(self: &mut Self) ->Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[Slash, Star]) {
            let operator = self.previous();
            let rhs = self.unary();
            expr = Binary { left: Box::from(expr), operator: operator , right: Box::from(rhs) };
        }

        expr
    }

    fn unary(self: &mut Self) ->Expr {
        if self.match_tokens(&[Bang, Minus]) {
            let operator = self.previous();
            let rhs = self.unary();

            Unary { operator: operator, right: Box::from(rhs) }
        } else {
            self.primary()
        }
    }

    fn primary(self: &mut Self) ->Expr {
        let token = self.peek();
        if self.match_token(LeftParen) {
            let expr = self.expression();
            self.consume(RightParen, "Expected ')'");
            Grouping { expression: Box::from(expr) }
        } else {
            self.advance();
            Literal { value: LiteralValue::from_token(token) }
        }
    }

    fn consume(self: &mut Self, token_type: TokenType, msg: &str) {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}",msg);
        }
    }

    fn match_token(self: &mut Self, t_type: TokenType) ->bool {
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

    fn match_tokens(self: &mut Self, t_types: &[TokenType]) ->bool {
        for t_type in t_types {
            if self.match_token(t_type.clone()) {
                return true;
            }
        }
        false
    }

    fn advance(self: &mut Self) ->Token {
        let token = self.peek();
        if !self.is_at_end() {
            self.current +=1;
        }

        self.previous()
    }

    fn peek(self: &Self) ->Token {
        self.tokens[self.current].clone()
    }

    fn previous(self: &Self) ->Token {
        self.tokens[self.current -1].clone()
    }

    fn is_at_end(self: &Self) ->bool {
        self.peek().token_type == Eof
    }
}
