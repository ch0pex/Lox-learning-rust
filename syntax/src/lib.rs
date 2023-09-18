pub mod scanner;
pub mod token;

#[cfg(test)]
mod test {

    use super::{
        scanner::Scanner,
        token::{TokenType, Token, Object}
    };

  #[test]
    fn test_scan_tokens_single_token() {
        let mut scanner = Scanner::new("(");
        let scanned_tokens = scanner.scan_tokens().unwrap();
        assert_eq!(scanned_tokens[0].lexeme, "(");

    }
    #[test]
    fn test_scan_tokens_multiple_tokens() {
        let mut scanner = Scanner::new("(2.2)");
        let scanned_tokens = scanner.scan_tokens().unwrap();
        let vec_tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 2),
            Token::new(TokenType::Number, "2.2".to_string(), Some(Object::Num(2.2)), 2),
            Token::new(TokenType::RightParen, ")".to_string(), None, 2),
        ];
        assert_eq!(scanned_tokens[0].lexeme, vec_tokens[0].lexeme);
        assert_eq!(scanned_tokens[1].lexeme, vec_tokens[1].lexeme);
        assert_eq!(scanned_tokens[2].lexeme, vec_tokens[2].lexeme);
    }

    #[test]
    fn test_scan_tokens_multiple_tokens_with_reserved_words() {
        let mut scanner = Scanner::new("if (2.2) \n { while true \n { } }");
        let scanned_tokens = scanner.scan_tokens().unwrap();
        let vec_tokens = vec![
            Token::new(TokenType::If, "if".to_string(), None, 2),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 2),
            Token::new(TokenType::Number, "2.2".to_string(), Some(Object::Num(2.2)), 2),
            Token::new(TokenType::RightParen, ")".to_string(), None, 2),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 2),
            Token::new(TokenType::While, "while".to_string(), None, 2),
            Token::new(TokenType::True, "true".to_string(), None, 2),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 2),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 2),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 2),
            Token::new(TokenType::Eof, "".to_string(), None, 2),

        ];
        assert_eq!(scanned_tokens.len(), vec_tokens.len());
        for (index, token) in scanned_tokens.iter().enumerate(){
            assert_eq!(token.lexeme, vec_tokens[index].lexeme);
            assert_eq!(token.ttype, vec_tokens[index].ttype);
        }
    }
}
