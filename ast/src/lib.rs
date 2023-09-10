pub mod ast; 
pub mod ast_printer; 

#[cfg(test)]
mod tests{
    use lox_syntax::token::{self, TokenType, Token, Object};
    use super::{
        ast_printer::AstPrinter, 
        ast::*,
    };

    #[test]
    fn test_ast_printer_should_return_parenthesis<'a>() {
        let mut printer = AstPrinter::new(); 
        let expressions = vec![
            &Expr::Literal(Token::new(TokenType::Number, "2.2".to_string(), Some(Object::Num(2.2)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "3.3".to_string(), Some(Object::Num(3.3)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "2.3".to_string(), Some(Object::Num(2.3)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "3.2".to_string(), Some(Object::Num(3.2)), 2))
        ];
        assert_eq!("(", printer.parenthesize("+",&expressions))

    }

    #[test]
    fn test_ast_printer_should_return_parenthesis_and_name() { 
        let mut printer: AstPrinter = AstPrinter::new(); 
        let expressions = vec![
            &Expr::Literal(Token::new(TokenType::Number, "2.2".to_string(), Some(Object::Num(2.2)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "3.3".to_string(), Some(Object::Num(3.3)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "2.3".to_string(), Some(Object::Num(2.3)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "3.2".to_string(), Some(Object::Num(3.2)), 2))
        ];
        assert_eq!("(+", printer.parenthesize("+", &expressions))
    }

    #[test]
    fn test_ast_printer_should_return_list_of_subexpr() { 
        let mut printer: AstPrinter = AstPrinter::new(); 
        let expressions = vec![
            &Expr::Literal(Token::new(TokenType::Number, "2.2".to_string(), Some(Object::Num(2.2)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "3.3".to_string(), Some(Object::Num(3.3)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "2.3".to_string(), Some(Object::Num(2.3)), 2)),
            &Expr::Literal(Token::new(TokenType::Number, "3.2".to_string(), Some(Object::Num(3.2)), 2))
        ];
        assert_eq!("+", printer.parenthesize("+", &expressions))
    }
} 
