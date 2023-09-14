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
    fn test_ast_printer_should_return_parenthesis_and_name() { let mut printer: AstPrinter = AstPrinter::new();
        let expressions = vec![];
        assert_eq!("(+)", printer.parenthesize("+", &expressions))
    }

    #[test]
    fn test_ast_printer_should_return_list_of_literals() {
        let mut printer: AstPrinter = AstPrinter::new();
        let expression1 = Expr::Literal(Object::Num(2.2));
        let expression2 = Expr::Literal(Object::Num(2.3));
        let expression3 = Expr::Literal(Object::Num(2.4));
        let expression4 = Expr::Literal(Object::Num(2.5));
        let expressions = vec![&expression1, &expression2, &expression3, &expression4];
        assert_eq!("(+ 2.2 2.3 2.4 2.5)", printer.parenthesize("+", &expressions))
    }

    #[test]
    fn test_ast_printer_with_binary_expression() {
        let mut printer: AstPrinter = AstPrinter::new();
        let expression1 = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-".to_string(), None,1),
                Box::new(Expr::Literal(Object::Nil))
            )),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Object::Num(45.67)))))
        );
        assert_eq!("(* (- nil) (group 45.67))", printer.print(expression1) )
    }
}
