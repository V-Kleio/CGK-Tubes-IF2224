use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct ParseNode {
    pub node_type: NodeType,
    pub children: Vec<ParseNode>,
}

#[derive(Debug)]
pub enum NodeType {
    // Non-Terminal Grammar Rules
    Program,
    ProgramHeader,
    DeclarationPart,
    ConstDeclaration,
    TypeDeclaration,
    VarDeclaration,
    IdentifierList,
    Type,
    ArrayType,
    Range,
    SubprogramDeclaration,
    ProcedureDeclaration,
    FunctionDeclaration,
    FormalParameterList,
    CompoundStatement,
    StatementList,
    AssignmentStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    ProcedureOrFunctionCall,
    ParameterList,
    Expression,
    SimpleExpression,
    Term,
    Factor,
    // Terminal
    Terminal(Token),
}

impl ParseNode {
    pub fn new(node_type: NodeType) -> Self {
        ParseNode {
            node_type,
            children: Vec::new(),
        }
    }

    pub fn new_terminal(token: Token) -> Self {
        ParseNode {
            node_type: NodeType::Terminal(token),
            children: Vec::new(),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Terminal(token) => write!(f, "{}", token),

            NodeType::Program => write!(f, "<program>"),
            NodeType::ProgramHeader => write!(f, "<program-header>"),
            NodeType::DeclarationPart => write!(f, "<declaration-part>"),
            NodeType::ConstDeclaration => write!(f, "<const-declaration>"),
            NodeType::TypeDeclaration => write!(f, "<type-declaration>"),
            NodeType::VarDeclaration => write!(f, "<var-declaration>"),
            NodeType::IdentifierList => write!(f, "<identifier-list>"),
            NodeType::Type => write!(f, "<type>"),
            NodeType::ArrayType => write!(f, "<array-type>"),
            NodeType::Range => write!(f, "<range>"),
            NodeType::SubprogramDeclaration => write!(f, "<subprogram-declaration>"),
            NodeType::ProcedureDeclaration => write!(f, "<procedure-declaration>"),
            NodeType::FunctionDeclaration => write!(f, "<function-declaration>"),
            NodeType::FormalParameterList => write!(f, "<formal-parameter-list>"),
            NodeType::CompoundStatement => write!(f, "<compound-statement>"),
            NodeType::StatementList => write!(f, "<statement-list>"),
            NodeType::AssignmentStatement => write!(f, "<assignment-statement>"),
            NodeType::IfStatement => write!(f, "<if-statement>"),
            NodeType::WhileStatement => write!(f, "<while-statement>"),
            NodeType::ForStatement => write!(f, "<for-statement>"),
            NodeType::ProcedureOrFunctionCall => write!(f, "<procedure/function-call>"),
            NodeType::ParameterList => write!(f, "<parameter-list>"),
            NodeType::Expression => write!(f, "<expression>"),
            NodeType::SimpleExpression => write!(f, "<simple-expression>"),
            NodeType::Term => write!(f, "<term>"),
            NodeType::Factor => write!(f, "<factor>"),
        }
    }
}

impl fmt::Display for ParseNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_recursive(f, 0)
    }
}

impl ParseNode {
    fn fmt_recursive(&self, f: &mut fmt::Formatter<'_>, indent_level: usize) -> fmt::Result {
        let indent = " ".repeat(indent_level * 2);

        writeln!(f, "{}{}", indent, self.node_type)?;

        for child in &self.children {
            child.fmt_recursive(f, indent_level + 1)?;
        }

        Ok(())
    }
}
