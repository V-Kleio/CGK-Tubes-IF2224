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

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
