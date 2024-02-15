use crate::scanner::Token;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub fn to_string(&self) ->String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::StringValue(x) => x.clone(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Nil => "nil".to_string(),
        }
    }
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {

    pub fn print(self: &Self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(self: &Self) -> String {
        match self {
            Expr::Binary { left, operator, right } => {
                format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string())
            },
            Expr::Grouping { expression } => {
                format!("(group {})", expression.to_string())
            },
            Expr::Literal { value } => {
                format!("{}", value.to_string())
            },
            Expr::Unary { operator, right } => {
                let operator_str = &operator.lexeme;
                let right_str = right.to_string();
                format!("({} {})", operator_str, right_str)

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Expr::*;
    use super::LiteralValue::*;
    use crate::TokenType;

    #[test]
    fn handle_pretty_print_ast() {
        let minus_token = Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: None, line_number: 1 };
        let onetwothree = Literal { value : Number(123.0) };
        let grouping = Grouping { expression: Box::from(Literal { value : Number(45.67) } )};
        let multi_operator = Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: None, line_number: 1 };
        let ast = Binary {left: Box::from(Unary {operator: minus_token, right: Box::from(onetwothree)}), operator: multi_operator, right: Box::from(grouping) };

        let result = ast.to_string();

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
