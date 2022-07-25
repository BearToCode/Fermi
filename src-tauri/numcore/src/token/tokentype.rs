use self::TokenType::*;
use super::super::out::{ErrorType, EvalResult};

/// Contains all the possible input tokens type.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // OPERATORS
    /// A plus '+' character.
    Plus,
    /// A minus '-' character.
    Minus,
    /// A star '*' character.
    Star,
    /// A slash '/' character.
    Slash,
    /// A comma ',' character.
    Comma,

    /// An opening bracket '(' character.
    OpeningBracket,
    /// A closing bracket ')' character.
    ClosingBracket,

    /// A dot '.' character.
    Dot,
    /// A string representing a value.
    Literal,

    /// An unknown identifier waiting to be processed.
    UnknownIdentifier,
    /// A string representing a function call.
    FunctionIdentifier,
    /// A string representing a variable.
    VariableIdentifier,
}

impl TokenType {
    pub fn is_expression(&self) -> bool {
        *self == TokenType::VariableIdentifier || // A variable or a constant
        *self == TokenType::FunctionIdentifier || // A function
        *self == TokenType::Literal ||  // A number
        self.is_binary_operator() // An operator
    }

    pub fn is_binary_operator(&self) -> bool {
        match self {
            Plus | Minus | Star | Slash | Comma => true,
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match self {
            Minus => true,
            _ => false,
        }
    }

    pub fn precedence(&self) -> EvalResult<u16> {
        Ok(match self {
            Comma => 10,
            Plus | Minus => 30,
            Star | Slash => 40,

            FunctionIdentifier | VariableIdentifier => 200,

            Literal => 300,
            _ => return Err(ErrorType::NotAnOperator { token: *self }),
        })
    }
}
