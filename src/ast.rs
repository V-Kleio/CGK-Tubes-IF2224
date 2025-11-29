use crate::types::DataType;
use std::fmt;

/// AST Node - decorated abstract syntax tree
#[derive(Debug, Clone)]
pub enum AstNode {
    // Program
    Program {
        name: String,
        declarations: Vec<AstNode>,
        body: Box<AstNode>,
        tab_index: Option<usize>,
    },
    
    // Declarations
    VarDecl {
        names: Vec<String>,
        data_type: DataType,
        tab_indices: Vec<usize>,
        level: usize,
    },
    
    ConstDecl {
        name: String,
        value: Box<AstNode>,
        data_type: DataType,
        tab_index: usize,
    },
    
    TypeDecl {
        name: String,
        type_def: DataType,
        tab_index: usize,
    },
    
    ProcDecl {
        name: String,
        params: Vec<AstNode>,
        declarations: Vec<AstNode>,
        body: Box<AstNode>,
        tab_index: usize,
        block_index: usize,
    },
    
    FuncDecl {
        name: String,
        params: Vec<AstNode>,
        return_type: DataType,
        declarations: Vec<AstNode>,
        body: Box<AstNode>,
        tab_index: usize,
        block_index: usize,
    },
    
    ParamDecl {
        names: Vec<String>,
        data_type: DataType,
        is_var: bool,
        tab_indices: Vec<usize>,
    },
    
    // Statements
    Block {
        statements: Vec<AstNode>,
    },
    
    Assign {
        target: Box<AstNode>,
        value: Box<AstNode>,
        data_type: DataType,
    },
    
    If {
        condition: Box<AstNode>,
        then_stmt: Box<AstNode>,
        else_stmt: Option<Box<AstNode>>,
    },
    
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    
    For {
        var_name: String,
        start: Box<AstNode>,
        end: Box<AstNode>,
        is_downto: bool,
        body: Box<AstNode>,
        tab_index: usize,
    },
    
    ProcCall {
        name: String,
        args: Vec<AstNode>,
        tab_index: usize,
    },
    
    // Expressions
    BinOp {
        op: String,
        left: Box<AstNode>,
        right: Box<AstNode>,
        data_type: DataType,
    },
    
    UnaryOp {
        op: String,
        operand: Box<AstNode>,
        data_type: DataType,
    },
    
    Var {
        name: String,
        data_type: DataType,
        tab_index: usize,
        level: usize,
    },
    
    Literal {
        value: LiteralValue,
        data_type: DataType,
    },
    
    // Empty statement
    Empty,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Char(char),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Integer(v) => write!(f, "{}", v),
            LiteralValue::Real(v) => write!(f, "{}", v),
            LiteralValue::Boolean(v) => write!(f, "{}", v),
            LiteralValue::Char(v) => write!(f, "'{}'", v),
            LiteralValue::String(v) => write!(f, "\"{}\"", v),
        }
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_recursive(f, 0)
    }
}

impl AstNode {
    fn fmt_recursive(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let ind = "  ".repeat(indent);
        
        match self {
            AstNode::Program { name, declarations, body, tab_index } => {
                writeln!(f, "{}Program(name: '{}', tab_index: {:?})", ind, name, tab_index)?;
                if !declarations.is_empty() {
                    writeln!(f, "{}  Declarations:", ind)?;
                    for decl in declarations {
                        decl.fmt_recursive(f, indent + 2)?;
                    }
                }
                writeln!(f, "{}  Body:", ind)?;
                body.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::VarDecl { names, data_type, tab_indices, level } => {
                writeln!(f, "{}VarDecl(names: {:?}, type: {}, indices: {:?}, level: {})", 
                         ind, names, data_type, tab_indices, level)?;
            }
            
            AstNode::ConstDecl { name, value, data_type, tab_index } => {
                writeln!(f, "{}ConstDecl(name: '{}', type: {}, tab_index: {})", 
                         ind, name, data_type, tab_index)?;
                writeln!(f, "{}  Value:", ind)?;
                value.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::TypeDecl { name, type_def, tab_index } => {
                writeln!(f, "{}TypeDecl(name: '{}', type: {}, tab_index: {})", 
                         ind, name, type_def, tab_index)?;
            }
            
            AstNode::ProcDecl { name, params, declarations, body, tab_index, block_index } => {
                writeln!(f, "{}ProcDecl(name: '{}', tab_index: {}, block_index: {})", 
                         ind, name, tab_index, block_index)?;
                if !params.is_empty() {
                    writeln!(f, "{}  Parameters:", ind)?;
                    for param in params {
                        param.fmt_recursive(f, indent + 2)?;
                    }
                }
                if !declarations.is_empty() {
                    writeln!(f, "{}  Declarations:", ind)?;
                    for decl in declarations {
                        decl.fmt_recursive(f, indent + 2)?;
                    }
                }
                writeln!(f, "{}  Body:", ind)?;
                body.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::FuncDecl { name, params, return_type, declarations, body, tab_index, block_index } => {
                writeln!(f, "{}FuncDecl(name: '{}', return_type: {}, tab_index: {}, block_index: {})", 
                         ind, name, return_type, tab_index, block_index)?;
                if !params.is_empty() {
                    writeln!(f, "{}  Parameters:", ind)?;
                    for param in params {
                        param.fmt_recursive(f, indent + 2)?;
                    }
                }
                if !declarations.is_empty() {
                    writeln!(f, "{}  Declarations:", ind)?;
                    for decl in declarations {
                        decl.fmt_recursive(f, indent + 2)?;
                    }
                }
                writeln!(f, "{}  Body:", ind)?;
                body.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::ParamDecl { names, data_type, is_var, tab_indices } => {
                writeln!(f, "{}ParamDecl(names: {:?}, type: {}, var: {}, indices: {:?})", 
                         ind, names, data_type, is_var, tab_indices)?;
            }
            
            AstNode::Block { statements } => {
                writeln!(f, "{}Block", ind)?;
                for stmt in statements {
                    stmt.fmt_recursive(f, indent + 1)?;
                }
            }
            
            AstNode::Assign { target, value, data_type } => {
                writeln!(f, "{}Assign(type: {})", ind, data_type)?;
                writeln!(f, "{}  Target:", ind)?;
                target.fmt_recursive(f, indent + 2)?;
                writeln!(f, "{}  Value:", ind)?;
                value.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::If { condition, then_stmt, else_stmt } => {
                writeln!(f, "{}If", ind)?;
                writeln!(f, "{}  Condition:", ind)?;
                condition.fmt_recursive(f, indent + 2)?;
                writeln!(f, "{}  Then:", ind)?;
                then_stmt.fmt_recursive(f, indent + 2)?;
                if let Some(else_part) = else_stmt {
                    writeln!(f, "{}  Else:", ind)?;
                    else_part.fmt_recursive(f, indent + 2)?;
                }
            }
            
            AstNode::While { condition, body } => {
                writeln!(f, "{}While", ind)?;
                writeln!(f, "{}  Condition:", ind)?;
                condition.fmt_recursive(f, indent + 2)?;
                writeln!(f, "{}  Body:", ind)?;
                body.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::For { var_name, start, end, is_downto, body, tab_index } => {
                writeln!(f, "{}For(var: '{}', downto: {}, tab_index: {})", 
                         ind, var_name, is_downto, tab_index)?;
                writeln!(f, "{}  Start:", ind)?;
                start.fmt_recursive(f, indent + 2)?;
                writeln!(f, "{}  End:", ind)?;
                end.fmt_recursive(f, indent + 2)?;
                writeln!(f, "{}  Body:", ind)?;
                body.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::ProcCall { name, args, tab_index } => {
                writeln!(f, "{}ProcCall(name: '{}', tab_index: {})", ind, name, tab_index)?;
                if !args.is_empty() {
                    writeln!(f, "{}  Arguments:", ind)?;
                    for arg in args {
                        arg.fmt_recursive(f, indent + 2)?;
                    }
                }
            }
            
            AstNode::BinOp { op, left, right, data_type } => {
                writeln!(f, "{}BinOp(op: '{}', type: {})", ind, op, data_type)?;
                writeln!(f, "{}  Left:", ind)?;
                left.fmt_recursive(f, indent + 2)?;
                writeln!(f, "{}  Right:", ind)?;
                right.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::UnaryOp { op, operand, data_type } => {
                writeln!(f, "{}UnaryOp(op: '{}', type: {})", ind, op, data_type)?;
                writeln!(f, "{}  Operand:", ind)?;
                operand.fmt_recursive(f, indent + 2)?;
            }
            
            AstNode::Var { name, data_type, tab_index, level } => {
                writeln!(f, "{}Var(name: '{}', type: {}, tab_index: {}, level: {})", 
                         ind, name, data_type, tab_index, level)?;
            }
            
            AstNode::Literal { value, data_type } => {
                writeln!(f, "{}Literal(value: {}, type: {})", ind, value, data_type)?;
            }
            
            AstNode::Empty => {
                writeln!(f, "{}Empty", ind)?;
            }
        }
        
        Ok(())
    }
}
