use crate::node::{NodeType, ParseNode};
use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub token: Token,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {} (found {})", self.message, self.token)
    }
}

type ParseResult = Result<ParseNode, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult {
        let program_node = self.parse_program()?;

        if !self.is_at_end() {
            return Err(ParseError {
                message: "Unexpected token after end of program.".to_string(),
                token: self.peek().clone(),
            });
        }

        Ok(program_node)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn check_value(&self, token_type: &TokenType, value: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        token.token_type == *token_type && token.value == value
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_keyword(&mut self, value: &str) -> bool {
        if self.check_value(&TokenType::Keyword, value) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        error_message: &str,
    ) -> Result<ParseNode, ParseError> {
        if self.check(&token_type) {
            Ok(ParseNode::new_terminal(self.advance()))
        } else {
            Err(ParseError {
                message: error_message.to_string(),
                token: self.peek().clone(),
            })
        }
    }

    fn consume_keyword(
        &mut self,
        value: &str,
        error_message: &str,
    ) -> Result<ParseNode, ParseError> {
        if self.check_value(&TokenType::Keyword, value) {
            Ok(ParseNode::new_terminal(self.advance()))
        } else {
            Err(ParseError {
                message: error_message.to_string(),
                token: self.peek().clone(),
            })
        }
    }
}
