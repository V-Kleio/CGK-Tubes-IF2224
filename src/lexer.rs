use crate::{dfa::Dfa, token::{Token, TokenType}};

pub struct Lexer {
    source: Vec<char>,
    dfa: Dfa,
    position: usize,
}

impl Lexer {
    pub fn new(source: String, dfa: Dfa) -> Self {
        Lexer { source: source.chars().collect(), dfa, position: 0 }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.source.len() {
            return None;
        }

        let mut current_state = self.dfa.start_state.clone();
        let start_pos = self.position;
        let mut last_final_state: Option<(String, usize)> = None;

        while self.position < self.source.len() {
            let current_char = self.source[self.position];

            if let Some(next_state) = self.get_next_state(&current_state, current_char) {
                current_state = next_state;
                self.position += 1;

                if self.dfa.final_states.contains_key(&current_state) {
                    last_final_state = Some((current_state.clone(), self.position));
                }

                if current_state == "S_Start" {
                    let token_value: String = self.source[start_pos..self.position].iter().collect();
                    if token_value.starts_with('{') || token_value.starts_with("(*") {
                        return self.get_next_token();
                    }
                }
            } else {
                break;
            }
        }

        if let Some((final_state, end_pos)) = last_final_state {
            let value: String = self.source[start_pos..end_pos].iter().collect();
            self.position = end_pos;

            if let Some(token_type_str) = self.dfa.final_states.get(&final_state) {
                let mut token = self.create_token(token_type_str, value);

                if token.token_type == TokenType::Identifier {
                    self.check_identifier(&mut token);
                }

                if token.token_type == TokenType::StringLiteral {
                    let content = &token.value[1..&token.value.len() - 1];
                    if content.chars().count() == 1 {
                        token.token_type = TokenType::CharLiteral;
                    }
                }

                return Some(token);
            }
        }

        if self.position < self.source.len() {
            eprintln!("Error: Invalid token starting with '{}' at position {}", self.source[start_pos], start_pos);
            self.position = self.source.len();
        }

        None
    }

    fn get_next_state(&self, current_state: &str, ch: char) -> Option<String> {
        if let Some(transitions) = self.dfa.transitions.get(current_state) {
            if let Some(next_state) = transitions.get(&ch.to_string()) {
                return Some(next_state.clone());
            }

            for (key, next_state) in transitions {
                if key.contains('-') && key.len() == 3 {
                    let mut parts = key.chars();
                    let start = parts.next()?;
                    parts.next();
                    let end = parts.next()?;
                    if ch >= start && ch <= end {
                        return Some(next_state.clone());
                    }
                } else if key.contains(ch) && !key.contains('-') {
                    return Some(next_state.clone());
                }
            }

            if let Some(next_state) = transitions.get("any") {
                return Some(next_state.clone());
            }
        }

        None
    }

    fn create_token(&self, token_type_str: &str, value: String) -> Token {
        let token_type = match token_type_str {
            "IDENTIFIER" => TokenType::Identifier,
            "NUMBER" => TokenType::Number,
            "STRING_LITERAL" => TokenType::StringLiteral,
            "ASSIGN_OPERATOR" => TokenType::AssignOperator,
            "RELATIONAL_OPERATOR" => TokenType::RelationalOperator,
            "ARITHMETIC_OPERATOR" => TokenType::ArithmeticOperator,
            "COLON" => TokenType::Colon,
            "DOT" => TokenType::Dot,
            "RANGE_OPERATOR" => TokenType::RangeOperator,
            "SEMICOLON" => TokenType::Semicolon,
            "COMMA" => TokenType::Comma,
            "LPARENTHESIS" => TokenType::LParenthesis,
            "RPARENTHESIS" => TokenType::RParenthesis,
            "LBRACKET" => TokenType::LBracket,
            "RBRACKET" => TokenType::RBracket,
            _ => panic!("Unknown token type: {}", token_type_str),
        };
        Token { token_type, value }
    }

    fn check_identifier(&self, token: &mut Token) {
        if self.dfa.keywords.contains(&token.value) {
            token.token_type = TokenType::Keyword;
        } else if self.dfa.word_logical_operators.contains(&token.value) {
            token.token_type = TokenType::LogicalOperator;
        } else if self.dfa.word_arithmetic_operators.contains(&token.value) {
            token.token_type = TokenType::ArithmeticOperator;
        }
    }
}
