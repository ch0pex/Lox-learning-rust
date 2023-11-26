use std::fmt;
use result::result::LoxResult;
use result::result::LoxResult::{Error, RunTimeError};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Assign, Equals,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token { ttype, lexeme, literal, line }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lit;
        match &self.literal {
            Some(literal) => {lit = literal.to_string()}
            None => { lit = "None".to_string()}
        }
        write!(f, "{:?} {} {}", self.ttype, self.lexeme, lit)
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Nil => write!(f, "Nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

impl Object {
    pub fn to_f64(&self) -> f64 {
        match self {
            Object::Num(value) => *value,
            Object::Str(string) => string.parse().unwrap(),
            Object::Nil => 0.0,
            Object::True => 1.0,
            Object::False => 0.0
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            Object::Num(value) => {
                let string = value.to_string();
                if string.ends_with(".0") {
                    string.strip_suffix(".0").unwrap().to_string()
                } else {
                    string
                }
            },
            Object::Str(string) => string.clone(),
            Object::Nil => "nil".to_string(),
            Object::True => "true".to_string(),
            Object::False => "false".to_string()
        }
    }
    pub fn negate(&self, line: usize) -> Result<Self, LoxResult> {
       match self {
           Object::True => Ok(Object::Num(-1.0)),
           Object::False => Ok(Object::Num(0.0)),
           Object::Num(val) => Ok(Object::Num(-val)),
           Object::Str(_) => Err(RunTimeError {line, message: "TypeError: bad operand type for unary -: 'str'".to_string()}),
           Object::Nil => Err(RunTimeError {line, message: "TypeError: bad operand type for unary -: 'Nil'".to_string()})
       }
    }

    pub fn is_truthy(&self) -> Object{
       match self  {
           Object::Num(num) => if *num == 1.0 {Object::True} else {Object::False},
           Object::Str(string) => if string.len() > 0 {Object::True} else {Object::False},
           Object::Nil => Object::False,
           Object::True => Object::True,
           Object::False => Object::False,
       }
    }

    pub fn add(left: Self, right: Self,  line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
               return Ok(Object::Num(left_value + right_value))
            }
        }
        if let Object::Str(left_value) = left {
            if let Object::Str(right_value) = right {
                let left_string = left_value.to_owned();
                let right_string = right_value.to_owned();
                return Ok(Object::Str(format!("{left_string}{right_string}")));
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary +"))
    }

    pub fn subtract(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return Ok(Object::Num(left_value - right_value))
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary -"))
    }

    pub fn multiply(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return Ok(Object::Num(left_value * right_value))
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary *"))
    }

    pub fn divide(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                if right_value == 0.0 {
                    return Err(LoxResult::run_time_error(line, "ZerDivisionError: division by zero"))
                }
                return Ok(Object::Num(left_value / right_value))
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary /"))
    }

    pub fn greater(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return match left_value > right_value {
                    true => Ok(Object::True),
                    false => Ok(Object::False)
                }
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary >"))
    }

    pub fn greater_equal(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return match left_value >= right_value {
                    true => Ok(Object::True),
                    false => Ok(Object::False)
                }
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary >="))
    }

    pub fn less(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return match left_value < right_value {
                    true => Ok(Object::True),
                    false => Ok(Object::False)
                }
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary <"))
    }

    pub fn less_equal(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return match left_value <= right_value {
                    true => Ok(Object::True),
                    false => Ok(Object::False)
                }
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary <="))
    }

    pub fn bang_equal(left: Self, right: Self, line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return match left_value != right_value {
                    true => Ok(Object::True),
                    false => Ok(Object::False)
                }
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary !="))
    }

    pub fn equals(left: Self, right: Self,line: usize) -> Result<Self, LoxResult> {
        if let Object::Num(left_value) = left {
            if let Object::Num(right_value) = right {
                return match left_value == right_value {
                    true => Ok(Object::True),
                    false => Ok(Object::False)
                }
            }
        }
        Err(LoxResult::run_time_error(line, "TypeError: bad operands type for binary =="))
    }
}

