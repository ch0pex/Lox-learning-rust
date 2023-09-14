pub mod ast;
pub mod ast_printer;

#[cfg(test)]
mod tests{
    use lox_syntax::token::{TokenType, Token, Object};
    use super::{
        ast_printer::AstPrinter,
        ast::*,
    };

    #[test]
    fn test_ast_printer_should_return_parenthesis() {
        let mut printer = AstPrinter::new();
        let expressions = vec![];
            assert_eq!("()", printer.parenthesize("",&expressions))

    }

    #[test]
    fn test_ast_printer_should_return_parenthesis_and_name() {
        let mut printer: AstPrinter = AstPrinter::new();
        let expressions = vec![];
        assert_eq!("(+)", printer.parenthesize("+", &expressions))
    }

    #[test]
    fn test_ast_printer_should_return_list_of_subexpr() {
        let mut printer: AstPrinter = AstPrinter::new();
        let expression1 = Expr::Literal(Token::new(TokenType::Number, "2.2".to_string(), Some(Object::Num(2.2)), 2));
        let expression2 = Expr::Literal(Token::new(TokenType::Number, "2.3".to_string(), Some(Object::Num(2.3)), 3));
        let expression3 = Expr::Literal(Token::new(TokenType::Number, "2.4".to_string(), Some(Object::Num(2.4)), 4));
        let expression4 = Expr::Literal(Token::new(TokenType::Number, "2.5".to_string(), Some(Object::Num(2.5)), 5));
        let expressions = vec![&expression1, &expression2, &expression3, &expression4];
        assert_eq!("(+ 2.2 2.3 2.4 2.5)", printer.parenthesize("+", &expressions))
    }
}
