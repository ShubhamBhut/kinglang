use crate::scanner::{self, Token, TokenType};

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}
use LiteralValue::*;

fn unwrap_as_f32(literal: Option<scanner::LiteralValue>) -> f32 {
    match literal {
        Some(scanner::LiteralValue::IntValue(x)) => x as f32,
        Some(scanner::LiteralValue::FValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        Some(scanner::LiteralValue::IdentifierValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as String"),
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::StringKing => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token),
        }
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            True
        } else {
            False
        }
    }

    pub fn is_false(self: &Self) -> LiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0 as f32 {
                    True
                } else {
                    False
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    True
                } else {
                    False
                }
            }
            True => False,
            False => True,
            Nil => True,
        }
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn print(self: &Self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(self: &Self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expr::Grouping { expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expr::Literal { value } => {
                format!("{}", value.to_string())
            }
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn evaluate(self: &Self) -> Result<LiteralValue, String> {
        match self {
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Unary { operator, right } => {
                let right = right.evaluate()?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => {
                        return Err(format!("Minus not implemented for  {}", right.to_string()))
                    }
                    (any, TokenType::Bang) => Ok(any.is_false()),
                    (_, ttype) => Err(format!("{} is not a valid unary operator", ttype)),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&left, operator.token_type, &right) {
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Number(x), TokenType::Greater, Number(y)) => {
                        Ok(LiteralValue::from_bool(x > y))
                    }
                    (Number(x), TokenType::GreaterEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x >= y))
                    }
                    (Number(x), TokenType::Less, Number(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (Number(x), TokenType::LessEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x <= y))
                    }
                    (Number(x), TokenType::BangEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x != y))
                    }
                    (Number(x), TokenType::EqualEqual, Number(y)) => {
                        Ok(LiteralValue::from_bool(x == y))
                    }
                    (StringValue(_), operator, Number(_)) => Err(format!(
                        "{} cannot operate between String and Number",
                        operator
                    )),
                    (Number(_), operator, StringValue(_)) => Err(format!(
                        "{} cannot operate between Number and String",
                        operator
                    )),
                    (StringValue(x), TokenType::Plus, StringValue(y)) => {
                        Ok(StringValue(format!("{}{}", x, y)))
                    }
                    (StringValue(x), TokenType::BangEqual, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x != y))
                    }
                    (StringValue(x), TokenType::EqualEqual, StringValue(y)) => {
                        Ok(LiteralValue::from_bool(x == y))
                    }
                    _ => todo!(),
                }
            }
            _ => Err("Can't perform binary operation on given values".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::*;
    use super::*;
    use crate::TokenType;

    #[test]
    fn handle_pretty_print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 1,
        };
        let onetwothree = Literal {
            value: LiteralValue::Number(123.0),
        };
        let grouping = Grouping {
            expression: Box::from(Literal {
                value: LiteralValue::Number(45.67),
            }),
        };
        let multi_operator = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 1,
        };
        let ast = Binary {
            left: Box::from(Unary {
                operator: minus_token,
                right: Box::from(onetwothree),
            }),
            operator: multi_operator,
            right: Box::from(grouping),
        };

        let result = ast.to_string();

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
