use crate::token::Token;
use std::fmt;

/// Semantic error types
#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    UndeclaredIdentifier(String),
    RedeclaredIdentifier(String),
    TypeMismatch { expected: String, found: String },
    InvalidOperation { op: String, types: String },
    #[allow(dead_code)]
    WrongParameterCount { expected: usize, found: usize },
    #[allow(dead_code)]
    NotCallable(String),
    #[allow(dead_code)]
    NotAssignable(String),
    InvalidArrayBounds,
    InvalidLoopVariable,
    ConditionNotBoolean,
}

/// Semantic error with location information
#[derive(Debug, Clone)]
pub struct SemanticError {
    #[allow(dead_code)]
    pub kind: SemanticErrorKind,
    pub message: String,
    pub token: Option<Token>,
}

impl SemanticError {
    pub fn new(kind: SemanticErrorKind, token: Option<Token>) -> Self {
        let message = match &kind {
            SemanticErrorKind::UndeclaredIdentifier(name) => {
                format!("Undeclared identifier '{}'", name)
            }
            SemanticErrorKind::RedeclaredIdentifier(name) => {
                format!("Identifier '{}' is already declared in this scope", name)
            }
            SemanticErrorKind::TypeMismatch { expected, found } => {
                format!("Type mismatch: expected {}, found {}", expected, found)
            }
            SemanticErrorKind::InvalidOperation { op, types } => {
                format!("Invalid operation '{}' for types {}", op, types)
            }
            SemanticErrorKind::WrongParameterCount { expected, found } => {
                format!(
                    "Wrong number of parameters: expected {}, found {}",
                    expected, found
                )
            }
            SemanticErrorKind::NotCallable(name) => {
                format!("'{}' is not a procedure or function", name)
            }
            SemanticErrorKind::NotAssignable(name) => {
                format!("Cannot assign to '{}'", name)
            }
            SemanticErrorKind::InvalidArrayBounds => {
                "Invalid array bounds: lower bound must be less than or equal to upper bound"
                    .to_string()
            }
            SemanticErrorKind::InvalidLoopVariable => {
                "Loop variable must be of integer type".to_string()
            }
            SemanticErrorKind::ConditionNotBoolean => {
                "Condition must be of boolean type".to_string()
            }
        };

        SemanticError {
            kind,
            message,
            token,
        }
    }

    pub fn undeclared(name: String, token: Option<Token>) -> Self {
        Self::new(SemanticErrorKind::UndeclaredIdentifier(name), token)
    }

    pub fn redeclared(name: String, token: Option<Token>) -> Self {
        Self::new(SemanticErrorKind::RedeclaredIdentifier(name), token)
    }

    pub fn type_mismatch(expected: String, found: String, token: Option<Token>) -> Self {
        Self::new(SemanticErrorKind::TypeMismatch { expected, found }, token)
    }

    pub fn invalid_operation(op: String, types: String, token: Option<Token>) -> Self {
        Self::new(SemanticErrorKind::InvalidOperation { op, types }, token)
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(token) = &self.token {
            write!(f, "Semantic error at {}: {}", token, self.message)
        } else {
            write!(f, "Semantic error: {}", self.message)
        }
    }
}

impl std::error::Error for SemanticError {}
