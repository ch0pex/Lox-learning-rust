
pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::Parser;
    use lox_syntax::token::{Token, TokenType};
    use lox_syntax::token::Object;

    // Helper function to create tokens for testing
    fn make_token(ttype: TokenType, lexeme: &str, literal: Object) -> Token {
        Token {
            ttype,
            lexeme: lexeme.to_string(),
            literal: Some(literal),
            line: 1, // You may adjust the line number as needed
        }
    }

    #[test]
    fn test_parse_literal() {
        let tokens = vec![make_token(TokenType::Number, "123", Object::Num(123.0))];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        // Add more assertions as needed to validate the parsed expression
    }

    #[test]
    fn test_parse_binary_expression() {
        let tokens = vec![
            make_token(TokenType::Number, "5", Object::Num(5.0)),
            make_token(TokenType::Plus, "+", Object::Nil),
            make_token(TokenType::Number, "7", Object::Num(7.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        // Add more assertions as needed to validate the parsed expression
    }

    // Add more test cases to cover other parsing scenarios
}