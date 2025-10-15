use crate::{dfa::Dfa, token::{Token, TokenType}};

pub struct Lexer {
    source: String,
    dfa: Dfa,
    position: usize,
}

impl Lexer {
    pub fn new(source: String, dfa: Dfa) -> Self {
        Lexer { source, dfa, position: 0 }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        if self.position >= self.source.len() {
            return None;
        }

        self.position = self.source.len();
        Some(Token { token_type: TokenType::Identifier, value: "todo".to_string() })
    }
}
