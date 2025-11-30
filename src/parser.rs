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

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    // Grammar Rule Functions

    fn parse_program(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::Program);
        node.children.push(self.parse_program_header()?);
        node.children.push(self.parse_declaration_part()?);
        node.children.push(self.parse_compound_statement()?);
        node.children
            .push(self.consume(TokenType::Dot, "Expected '.' at the end of the program.")?);
        Ok(node)
    }

    fn parse_program_header(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ProgramHeader);
        node.children
            .push(self.consume_keyword("program", "Expected 'program' keyword.")?);
        node.children
            .push(self.consume(TokenType::Identifier, "Expected program name.")?);
        node.children
            .push(self.consume(TokenType::Semicolon, "Expected ';' after program name.")?);
        Ok(node)
    }

    fn parse_declaration_part(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::DeclarationPart);

        loop {
            if self.check_value(&TokenType::Keyword, "konstanta") {
                node.children.push(self.parse_const_declaration()?);
            } else if self.check_value(&TokenType::Keyword, "tipe") {
                node.children.push(self.parse_type_declaration()?);
            } else if self.check_value(&TokenType::Keyword, "variabel") {
                node.children.push(self.parse_var_declaration()?);
            } else if self.check_value(&TokenType::Keyword, "prosedur")
                || self.check_value(&TokenType::Keyword, "fungsi")
            {
                node.children.push(self.parse_subprogram_declaration()?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_const_declaration(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ConstDeclaration);

        node.children
            .push(self.consume_keyword("konstanta", "Expected 'konstanta' keyword.")?);

        loop {
            node.children
                .push(self.consume(TokenType::Identifier, "Expected constant identifier.")?);
            node.children.push(self.consume(
                TokenType::RelationalOperator,
                "Expected '=' in constant declaration.",
            )?);
            node.children.push(self.parse_expression()?);
            node.children.push(self.consume(
                TokenType::Semicolon,
                "Expected ';' after constant declaration.",
            )?);

            if !self.check(&TokenType::Identifier) {
                break;
            }
        }

        Ok(node)
    }

    fn parse_type_declaration(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::TypeDeclaration);

        node.children
            .push(self.consume_keyword("tipe", "Expected 'tipe' keyword.")?);

        loop {
            node.children
                .push(self.consume(TokenType::Identifier, "Expected type identifier.")?);
            node.children.push(self.consume(
                TokenType::RelationalOperator,
                "Expected '=' in type declaration.",
            )?);
            node.children.push(self.parse_type()?);
            node.children
                .push(self.consume(TokenType::Semicolon, "Expected ';' after type declaration.")?);

            if !self.check(&TokenType::Identifier) {
                break;
            }
        }

        Ok(node)
    }

    fn parse_var_declaration(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::VarDeclaration);

        node.children
            .push(self.consume_keyword("variabel", "Expected 'variabel' keyword.")?);

        loop {
            node.children.push(self.parse_identifier_list()?);
            node.children
                .push(self.consume(TokenType::Colon, "Expected ':' after identifier list.")?);
            node.children.push(self.parse_type()?);
            node.children.push(self.consume(
                TokenType::Semicolon,
                "Expected ';' after variable declaration.",
            )?);

            if !self.check(&TokenType::Identifier) {
                break;
            }
        }

        Ok(node)
    }

    fn parse_identifier_list(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::IdentifierList);

        node.children
            .push(self.consume(TokenType::Identifier, "Expected identifier.")?);

        while self.match_token(&TokenType::Comma) {
            node.children.push(ParseNode::new_terminal(self.previous()));
            node.children
                .push(self.consume(TokenType::Identifier, "Expected identifier after ','.")?);
        }

        Ok(node)
    }

    fn parse_type(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::Type);

        if self.check_value(&TokenType::Keyword, "larik") {
            node.children.push(self.parse_array_type()?);
        } else if self.check_value(&TokenType::Keyword, "rekaman") {
            node.children.push(self.parse_record_type()?);
        } else if self.check_value(&TokenType::Keyword, "integer")
            || self.check_value(&TokenType::Keyword, "real")
            || self.check_value(&TokenType::Keyword, "boolean")
            || self.check_value(&TokenType::Keyword, "char")
        {
            node.children.push(ParseNode::new_terminal(self.advance()));
        } else if self.check(&TokenType::Identifier) {
            node.children.push(ParseNode::new_terminal(self.advance()));
        } else {
            return Err(ParseError {
                message: "Expected type name.".to_string(),
                token: self.peek().clone(),
            });
        }

        Ok(node)
    }

    fn parse_array_type(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ArrayType);

        node.children
            .push(self.consume_keyword("larik", "Expected 'larik' keyword.")?);
        node.children
            .push(self.consume(TokenType::LBracket, "Expected '[' after 'larik'.")?);

        node.children.push(self.parse_range()?);

        while self.match_token(&TokenType::Comma) {
            node.children.push(ParseNode::new_terminal(self.previous()));
            node.children.push(self.parse_range()?);
        }

        node.children
            .push(self.consume(TokenType::RBracket, "Expected ']' after range.")?);
        node.children
            .push(self.consume_keyword("dari", "Expected 'dari' keyword.")?);
        node.children.push(self.parse_type()?);

        Ok(node)
    }

    fn parse_range(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::Range);

        node.children.push(self.parse_expression()?);
        node.children
            .push(self.consume(TokenType::RangeOperator, "Expected '..' in range.")?);
        node.children.push(self.parse_expression()?);

        Ok(node)
    }

    fn parse_record_type(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::RecordType);

        node.children
            .push(self.consume_keyword("rekaman", "Expected 'rekaman' keyword.")?);

        loop {
            if self.check_value(&TokenType::Keyword, "selesai") {
                break;
            }

            node.children.push(self.parse_identifier_list()?);
            node.children
                .push(self.consume(TokenType::Colon, "Expected ':' after field identifiers.")?);
            node.children.push(self.parse_type()?);

            if self.check_value(&TokenType::Keyword, "selesai") {
                break;
            }

            if self.match_token(&TokenType::Semicolon) {
                node.children.push(ParseNode::new_terminal(self.previous()));
            }
        }

        node.children
            .push(self.consume_keyword("selesai", "Expected 'selesai' keyword.")?);

        Ok(node)
    }

    fn parse_subprogram_declaration(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::SubprogramDeclaration);

        if self.check_value(&TokenType::Keyword, "prosedur") {
            node.children.push(self.parse_procedure_declaration()?);
        } else if self.check_value(&TokenType::Keyword, "fungsi") {
            node.children.push(self.parse_function_declaration()?);
        } else {
            return Err(ParseError {
                message: "Expected 'prosedur' or 'fungsi' keyword.".to_string(),
                token: self.peek().clone(),
            });
        }

        Ok(node)
    }

    fn parse_procedure_declaration(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ProcedureDeclaration);

        node.children
            .push(self.consume_keyword("prosedur", "Expected 'prosedur' keyword.")?);
        node.children
            .push(self.consume(TokenType::Identifier, "Expected procedure name.")?);

        if self.check(&TokenType::LParenthesis) {
            node.children.push(self.parse_formal_parameter_list()?);
        }

        node.children
            .push(self.consume(TokenType::Semicolon, "Expected ';' after procedure header.")?);
        node.children.push(self.parse_declaration_part()?);
        node.children.push(self.parse_compound_statement()?);
        node.children
            .push(self.consume(TokenType::Semicolon, "Expected ';' after procedure body.")?);

        Ok(node)
    }

    fn parse_function_declaration(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::FunctionDeclaration);

        node.children
            .push(self.consume_keyword("fungsi", "Expected 'fungsi' keyword.")?);
        node.children
            .push(self.consume(TokenType::Identifier, "Expected function name.")?);

        if self.check(&TokenType::LParenthesis) {
            node.children.push(self.parse_formal_parameter_list()?);
        }

        node.children
            .push(self.consume(TokenType::Colon, "Expected ':' after function parameters.")?);
        node.children.push(self.parse_type()?);
        node.children
            .push(self.consume(TokenType::Semicolon, "Expected ';' after function header.")?);
        node.children.push(self.parse_declaration_part()?);
        node.children.push(self.parse_compound_statement()?);
        node.children
            .push(self.consume(TokenType::Semicolon, "Expected ';' after function body.")?);

        Ok(node)
    }

    fn parse_formal_parameter_list(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::FormalParameterList);

        node.children.push(self.consume(
            TokenType::LParenthesis,
            "Expected '(' to start parameter list.",
        )?);

        if self.match_keyword("variabel") {
            node.children.push(ParseNode::new_terminal(self.previous()));
        }

        node.children.push(self.parse_identifier_list()?);
        node.children.push(self.consume(
            TokenType::Colon,
            "Expected ':' after parameter identifiers.",
        )?);
        node.children.push(self.parse_type()?);

        while self.match_token(&TokenType::Semicolon) {
            node.children.push(ParseNode::new_terminal(self.previous()));

            if self.match_keyword("variabel") {
                node.children.push(ParseNode::new_terminal(self.previous()));
            }

            node.children.push(self.parse_identifier_list()?);
            node.children.push(self.consume(
                TokenType::Colon,
                "Expected ':' after parameter identifiers.",
            )?);
            node.children.push(self.parse_type()?);
        }

        node.children.push(self.consume(
            TokenType::RParenthesis,
            "Expected ')' to end parameter list.",
        )?);

        Ok(node)
    }

    fn parse_compound_statement(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::CompoundStatement);

        node.children
            .push(self.consume_keyword("mulai", "Expected 'mulai' keyword.")?);

        node.children.push(self.parse_statement_list()?);

        node.children
            .push(self.consume_keyword("selesai", "Expected 'selesai' keyword.")?);

        Ok(node)
    }

    fn parse_statement_list(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::StatementList);

        if !self.check_value(&TokenType::Keyword, "selesai") {
            node.children.push(self.parse_statement()?);

            while self.match_token(&TokenType::Semicolon) {
                node.children.push(ParseNode::new_terminal(self.previous()));

                if self.check_value(&TokenType::Keyword, "selesai") {
                    break;
                }

                node.children.push(self.parse_statement()?);
            }
        }

        Ok(node)
    }

    fn parse_statement(&mut self) -> ParseResult {
        if self.check_value(&TokenType::Keyword, "jika") {
            self.parse_if_statement()
        } else if self.check_value(&TokenType::Keyword, "selama") {
            self.parse_while_statement()
        } else if self.check_value(&TokenType::Keyword, "untuk") {
            self.parse_for_statement()
        } else if self.check_value(&TokenType::Keyword, "ulangi") {
            self.parse_repeat_statement()
        } else if self.check_value(&TokenType::Keyword, "mulai") {
            self.parse_compound_statement()
        } else if self.check(&TokenType::Identifier) {
            let saved_pos = self.current;
            self.advance();

            if self.check(&TokenType::AssignOperator) {
                self.current = saved_pos;
                self.parse_assignment_statement()
            } else if self.check(&TokenType::LParenthesis) {
                self.current = saved_pos;
                self.parse_procedure_or_function_call()
            } else {
                self.current = saved_pos;
                self.parse_procedure_or_function_call()
            }
        } else {
            Ok(ParseNode::new(NodeType::StatementList))
        }
    }

    fn parse_assignment_statement(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::AssignmentStatement);

        node.children
            .push(self.consume(TokenType::Identifier, "Expected identifier.")?);
        node.children
            .push(self.consume(TokenType::AssignOperator, "Expected ':=' operator.")?);
        node.children.push(self.parse_expression()?);

        Ok(node)
    }

    fn parse_if_statement(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::IfStatement);

        node.children
            .push(self.consume_keyword("jika", "Expected 'jika' keyword.")?);
        node.children.push(self.parse_expression()?);
        node.children
            .push(self.consume_keyword("maka", "Expected 'maka' keyword.")?);
        node.children.push(self.parse_statement()?);

        if self.match_keyword("selain_itu") {
            node.children.push(ParseNode::new_terminal(self.previous()));
            node.children.push(self.parse_statement()?);
        }

        Ok(node)
    }

    fn parse_while_statement(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::WhileStatement);

        node.children
            .push(self.consume_keyword("selama", "Expected 'selama' keyword.")?);
        node.children.push(self.parse_expression()?);
        node.children
            .push(self.consume_keyword("lakukan", "Expected 'lakukan' keyword.")?);
        node.children.push(self.parse_statement()?);

        Ok(node)
    }

    fn parse_for_statement(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ForStatement);

        node.children
            .push(self.consume_keyword("untuk", "Expected 'untuk' keyword.")?);
        node.children
            .push(self.consume(TokenType::Identifier, "Expected loop variable.")?);
        node.children
            .push(self.consume(TokenType::AssignOperator, "Expected ':=' operator.")?);
        node.children.push(self.parse_expression()?);

        if self.match_keyword("ke") {
            node.children.push(ParseNode::new_terminal(self.previous()));
        } else if self.match_keyword("turun_ke") {
            node.children.push(ParseNode::new_terminal(self.previous()));
        } else {
            return Err(ParseError {
                message: "Expected 'ke' or 'turun_ke' keyword.".to_string(),
                token: self.peek().clone(),
            });
        }

        node.children.push(self.parse_expression()?);
        node.children
            .push(self.consume_keyword("lakukan", "Expected 'lakukan' keyword.")?);
        node.children.push(self.parse_statement()?);

        Ok(node)
    }

    fn parse_repeat_statement(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::RepeatStatement);

        node.children
            .push(self.consume_keyword("ulangi", "Expected 'ulangi' keyword.")?);

        let mut statements = Vec::new();
        loop {
            if self.check_value(&TokenType::Keyword, "sampai") {
                break;
            }

            statements.push(self.parse_statement()?);

            if self.check_value(&TokenType::Keyword, "sampai") {
                break;
            }

            if self.match_token(&TokenType::Semicolon) {
                statements.push(ParseNode::new_terminal(self.previous()));
            }
        }

        node.children.extend(statements);

        node.children
            .push(self.consume_keyword("sampai", "Expected 'sampai' keyword.")?);
        node.children.push(self.parse_expression()?);

        Ok(node)
    }

    fn parse_procedure_or_function_call(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ProcedureOrFunctionCall);

        node.children.push(self.consume(
            TokenType::Identifier,
            "Expected procedure or function name.",
        )?);

        if self.match_token(&TokenType::LParenthesis) {
            node.children.push(ParseNode::new_terminal(self.previous()));

            if !self.check(&TokenType::RParenthesis) {
                node.children.push(self.parse_parameter_list()?);
            }

            node.children.push(self.consume(
                TokenType::RParenthesis,
                "Expected ')' after parameter list.",
            )?);
        }

        Ok(node)
    }

    fn parse_parameter_list(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::ParameterList);

        node.children.push(self.parse_expression()?);

        while self.match_token(&TokenType::Comma) {
            node.children.push(ParseNode::new_terminal(self.previous()));
            node.children.push(self.parse_expression()?);
        }

        Ok(node)
    }

    fn parse_expression(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::Expression);

        let left_node = self.parse_simple_expression()?;

        if self.check(&TokenType::RelationalOperator) {
            node.children.push(left_node);
            node.children.push(self.parse_relational_operator()?);
            node.children.push(self.parse_simple_expression()?);
        } else {
            node.children.push(left_node);
        }
        Ok(node)
    }

    fn parse_simple_expression(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::SimpleExpression);

        if self.check_value(&TokenType::ArithmeticOperator, "+")
            || self.check_value(&TokenType::ArithmeticOperator, "-")
        {
            node.children.push(ParseNode::new_terminal(self.advance()));
        }

        node.children.push(self.parse_term()?);

        while let Some(operator_token) = self.match_additive_operator() {
            node.children.push(ParseNode::new_terminal(operator_token));
            node.children.push(self.parse_term()?);
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::Term);

        node.children.push(self.parse_factor()?);

        while let Some(operator_token) = self.match_multiplicative_operator() {
            node.children.push(ParseNode::new_terminal(operator_token));
            node.children.push(self.parse_factor()?);
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> ParseResult {
        let mut node = ParseNode::new(NodeType::Factor);

        if self.match_token(&TokenType::Number) {
            node.children.push(ParseNode::new_terminal(self.previous()));
        } else if self.match_token(&TokenType::CharLiteral) {
            node.children.push(ParseNode::new_terminal(self.previous()));
        } else if self.match_token(&TokenType::StringLiteral) {
            node.children.push(ParseNode::new_terminal(self.previous()));
        } else if self.match_token(&TokenType::LParenthesis) {
            node.children.push(ParseNode::new_terminal(self.previous()));
            node.children.push(self.parse_expression()?);
            node.children
                .push(self.consume(TokenType::RParenthesis, "Expected ')' after expression.")?);
        } else if self.check_value(&TokenType::Keyword, "true")
            || self.check_value(&TokenType::Keyword, "false")
        {
            node.children.push(ParseNode::new_terminal(self.advance()));
        } else if self.check_value(&TokenType::LogicalOperator, "tidak") {
            node.children.push(ParseNode::new_terminal(self.advance()));
            node.children.push(self.parse_factor()?);
        } else if self.check(&TokenType::Identifier) {
            let identifier_token = self.advance();

            if self.check(&TokenType::LParenthesis) {
                let mut func_call_node = ParseNode::new(NodeType::ProcedureOrFunctionCall);
                func_call_node
                    .children
                    .push(ParseNode::new_terminal(identifier_token));

                func_call_node
                    .children
                    .push(self.consume(TokenType::LParenthesis, "Expected '('.")?);

                if !self.check(&TokenType::RParenthesis) {
                    func_call_node.children.push(self.parse_parameter_list()?);
                }

                func_call_node
                    .children
                    .push(self.consume(TokenType::RParenthesis, "Expected ')' after parameters.")?);

                node.children.push(func_call_node);
            } else {
                node.children
                    .push(ParseNode::new_terminal(identifier_token));

                while self.match_token(&TokenType::Dot) {
                    node.children.push(ParseNode::new_terminal(self.previous()));
                    node.children.push(
                        self.consume(TokenType::Identifier, "Expected field name after '.'.")?,
                    );
                }
            }
        } else {
            return Err(ParseError {
                message: "Expected a factor (e.g., number, identifier, or '(expression)')."
                    .to_string(),
                token: self.peek().clone(),
            });
        }

        Ok(node)
    }

    fn parse_relational_operator(&mut self) -> ParseResult {
        if self.check(&TokenType::RelationalOperator) {
            Ok(ParseNode::new_terminal(self.advance()))
        } else {
            Err(ParseError {
                message: "Expected a relational operator (e.g., =, <, >).".to_string(),
                token: self.peek().clone(),
            })
        }
    }

    fn match_additive_operator(&mut self) -> Option<Token> {
        if self.check_value(&TokenType::ArithmeticOperator, "+")
            || self.check_value(&TokenType::ArithmeticOperator, "-")
        {
            Some(self.advance())
        } else if self.check_value(&TokenType::LogicalOperator, "atau") {
            Some(self.advance())
        } else {
            None
        }
    }

    fn match_multiplicative_operator(&mut self) -> Option<Token> {
        if self.check_value(&TokenType::ArithmeticOperator, "*")
            || self.check_value(&TokenType::ArithmeticOperator, "/")
        {
            Some(self.advance())
        } else if self.check_value(&TokenType::Keyword, "bagi")
            || self.check_value(&TokenType::Keyword, "mod")
        {
            Some(self.advance())
        } else if self.check_value(&TokenType::LogicalOperator, "dan") {
            Some(self.advance())
        } else {
            None
        }
    }
}