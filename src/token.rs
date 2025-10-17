use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Identifier,
    ArithmeticOperator,
    RelationalOperator,
    LogicalOperator,
    AssignOperator,
    Number,
    CharLiteral,
    StringLiteral,
    Semicolon,
    Comma,
    Colon,
    Dot,
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    RangeOperator,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We convert the enum variant to a string for the output
        let type_str = match self.token_type {
            TokenType::Keyword => "KEYWORD",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::ArithmeticOperator => "ARITHMETIC_OPERATOR",
            TokenType::RelationalOperator => "RELATIONAL_OPERATOR",
            TokenType::LogicalOperator => "LOGICAL_OPERATOR",
            TokenType::AssignOperator => "ASSIGN_OPERATOR",
            TokenType::Number => "NUMBER",
            TokenType::CharLiteral => "CHAR_LITERAL",
            TokenType::StringLiteral => "STRING_LITERAL",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Comma => "COMMA",
            TokenType::Colon => "COLON",
            TokenType::Dot => "DOT",
            TokenType::LParenthesis => "LPARENTHESIS",
            TokenType::RParenthesis => "RPARENTHESIS",
            TokenType::LBracket => "LBRACKET",
            TokenType::RBracket => "RBRACKET",
            TokenType::RangeOperator => "RANGE_OPERATOR",
        };
        write!(f, "{}({})", type_str, self.value)
    }
}
