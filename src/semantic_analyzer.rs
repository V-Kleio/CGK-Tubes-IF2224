use crate::ast::{AstNode, LiteralValue};
use crate::node::{NodeType, ParseNode};
use crate::semantic_error::{SemanticError, SemanticErrorKind};
use crate::symbol_table::{ATabEntry, SymbolTable, TabEntry};
use crate::token::TokenType;
use crate::types::{DataType, ObjectKind};

/// Semantic analyzer that transforms parse tree to decorated AST
pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    pub errors: Vec<SemanticError>,
    current_proc: Option<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            current_proc: None,
        }
    }

    /// Main entry point for semantic analysis
    pub fn analyze(&mut self, parse_tree: &ParseNode) -> Result<AstNode, Vec<SemanticError>> {
        let ast = self.visit_program(parse_tree);

        if self.errors.is_empty() {
            Ok(ast)
        } else {
            Err(self.errors.clone())
        }
    }

    /// Visit program node
    fn visit_program(&mut self, node: &ParseNode) -> AstNode {
        // program -> program-header declaration-part compound-statement DOT
        if let NodeType::Program = node.node_type {
            let program_name = self.get_program_name(&node.children[0]);

            // Insert program into symbol table
            let tab_index = self.symbol_table.insert(TabEntry {
                name: program_name.clone(),
                link: None,
                obj: ObjectKind::Program,
                data_type: DataType::Void,
                ref_index: None,
                normal: true,
                level: 0,
                address: 0,
            });

            // Process declarations
            let declarations = self.visit_declaration_part(&node.children[1]);

            // Enter new block for main compound statement (btab[1])
            let main_block_index = self.symbol_table.enter_block();

            // Process main compound statement
            let body = self.visit_compound_statement(&node.children[2]);

            // Exit main block
            self.symbol_table.exit_block();

            return AstNode::Program {
                name: program_name,
                declarations,
                body: Box::new(body),
                tab_index: Some(tab_index),
            };
        }

        AstNode::Empty
    }

    /// Get program name from program header
    fn get_program_name(&self, node: &ParseNode) -> String {
        // program-header -> KEYWORD(program) IDENTIFIER SEMICOLON
        if let NodeType::Terminal(token) = &node.children[1].node_type {
            return token.value.clone();
        }
        "Unknown".to_string()
    }

    /// Visit declaration part
    fn visit_declaration_part(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut declarations = Vec::new();

        for child in &node.children {
            match &child.node_type {
                NodeType::ConstDeclaration => {
                    declarations.extend(self.visit_const_declaration(child));
                }
                NodeType::TypeDeclaration => {
                    declarations.extend(self.visit_type_declaration(child));
                }
                NodeType::VarDeclaration => {
                    declarations.extend(self.visit_var_declaration(child));
                }
                NodeType::SubprogramDeclaration => {
                    if let Some(decl) = self.visit_subprogram_declaration(child) {
                        declarations.push(decl);
                    }
                }
                _ => {}
            }
        }

        declarations
    }

    /// Visit variable declaration
    fn visit_var_declaration(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut declarations = Vec::new();
        let mut i = 1; // Skip "variabel" keyword

        while i < node.children.len() {
            // Get identifier list
            let id_list = self.get_identifier_list(&node.children[i]);
            i += 1; // Skip identifier list

            // Skip colon
            i += 1;

            // Get type
            let data_type = self.get_type(&node.children[i]);
            i += 1;

            // Skip semicolon
            i += 1;

            // Insert variables into symbol table and create separate VarDecl for each
            let level = self.symbol_table.current_level();

            for name in &id_list {
                // Check for redeclaration
                if let Some(_) = self.symbol_table.lookup_current_scope(name) {
                    self.errors.push(SemanticError::redeclared(
                        name.clone(),
                        None,
                    ));
                    continue;
                }

                let tab_index = self.symbol_table.insert(TabEntry {
                    name: name.clone(),
                    link: None,
                    obj: ObjectKind::Variable,
                    data_type: data_type.clone(),
                    ref_index: None,
                    normal: true,
                    level,
                    address: 0,  // TODO: change
                });

                // Create individual VarDecl for each variable
                declarations.push(AstNode::VarDecl {
                    names: vec![name.clone()],
                    data_type: data_type.clone(),
                    tab_indices: vec![tab_index],
                    level,
                });

                // Update variable size
                self.symbol_table.add_var_size(1);
            }
        }

        declarations
    }

    /// Visit constant declaration
    fn visit_const_declaration(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut declarations = Vec::new();
        let mut i = 1; // Skip "konstanta" keyword

        while i < node.children.len() {
            // Get identifier
            let name = if let NodeType::Terminal(token) = &node.children[i].node_type {
                token.value.clone()
            } else {
                i += 1;
                continue;
            };
            i += 1;

            // Skip '='
            i += 1;

            // Get value expression
            let value_expr = self.visit_expression(&node.children[i]);
            let data_type = self.get_expr_type(&value_expr);
            i += 1;

            // Skip semicolon
            i += 1;

            // Check for redeclaration
            if let Some(_) = self.symbol_table.lookup_current_scope(&name) {
                self.errors.push(SemanticError::redeclared(name.clone(), None));
                continue;
            }

            let tab_index = self.symbol_table.insert(TabEntry {
                name: name.clone(),
                link: None,
                obj: ObjectKind::Constant,
                data_type: data_type.clone(),
                ref_index: None,
                normal: true,
                level: self.symbol_table.current_level(),
                address: 0,
            });

            declarations.push(AstNode::ConstDecl {
                name,
                value: Box::new(value_expr),
                data_type,
                tab_index,
            });
        }

        declarations
    }

    /// Visit type declaration
    fn visit_type_declaration(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut declarations = Vec::new();
        let mut i = 1; // Skip "tipe" keyword

        while i < node.children.len() {
            // Get identifier
            let name = if let NodeType::Terminal(token) = &node.children[i].node_type {
                token.value.clone()
            } else {
                i += 1;
                continue;
            };
            i += 1;

            // Skip '='
            i += 1;

            // Get type definition
            let type_def = self.get_type(&node.children[i]);
            i += 1;

            // Skip semicolon
            i += 1;

            // Check for redeclaration
            if let Some(_) = self.symbol_table.lookup_current_scope(&name) {
                self.errors.push(SemanticError::redeclared(name.clone(), None));
                continue;
            }

            let tab_index = self.symbol_table.insert(TabEntry {
                name: name.clone(),
                link: None,
                obj: ObjectKind::Type,
                data_type: type_def.clone(),
                ref_index: None,
                normal: true,
                level: self.symbol_table.current_level(),
                address: 0,
            });

            declarations.push(AstNode::TypeDecl {
                name,
                type_def,
                tab_index,
            });
        }

        declarations
    }

    /// Visit subprogram declaration
    fn visit_subprogram_declaration(&mut self, node: &ParseNode) -> Option<AstNode> {
        if node.children.is_empty() {
            return None;
        }

        let child = &node.children[0];
        match &child.node_type {
            NodeType::ProcedureDeclaration => Some(self.visit_procedure_declaration(child)),
            NodeType::FunctionDeclaration => Some(self.visit_function_declaration(child)),
            _ => None,
        }
    }

    /// Visit procedure declaration
    fn visit_procedure_declaration(&mut self, node: &ParseNode) -> AstNode {
        // prosedur IDENTIFIER (params)? SEMICOLON declarations compound-statement SEMICOLON
        let mut idx = 1; // Skip "prosedur" keyword

        let name = if let NodeType::Terminal(token) = &node.children[idx].node_type {
            token.value.clone()
        } else {
            "unknown".to_string()
        };
        idx += 1;

        // Check for redeclaration
        if let Some(_) = self.symbol_table.lookup_current_scope(&name) {
            self.errors.push(SemanticError::redeclared(name.clone(), None));
        }

        // Enter new block
        let block_index = self.symbol_table.enter_block();

        // Check if parameters exist and save the node index
        let param_node_idx = if idx < node.children.len() && matches!(node.children[idx].node_type, NodeType::FormalParameterList) {
            let temp_idx = idx;
            idx += 1;
            Some(temp_idx)
        } else {
            None
        };

        // Skip semicolon
        idx += 1;

        // Insert procedure into symbol table (at parent level)
        self.symbol_table.exit_block();
        let tab_index = self.symbol_table.insert(TabEntry {
            name: name.clone(),
            link: None,
            obj: ObjectKind::Procedure,
            data_type: DataType::Void,
            ref_index: Some(block_index),
            normal: true,
            level: self.symbol_table.current_level(),
            address: 0,
        });
        
        // Re-enter block for procedure body
        self.symbol_table.enter_block();
        
        // process parameters
        let params = if let Some(param_idx) = param_node_idx {
            self.visit_formal_parameter_list(&node.children[param_idx])
        } else {
            Vec::new()
        };

        // Process declarations
        let declarations = self.visit_declaration_part(&node.children[idx]);
        idx += 1;

        // Process body
        let body = self.visit_compound_statement(&node.children[idx]);

        // Exit block
        self.symbol_table.exit_block();

        AstNode::ProcDecl {
            name,
            params,
            declarations,
            body: Box::new(body),
            tab_index,
            block_index,
        }
    }

    /// Visit function declaration
    fn visit_function_declaration(&mut self, node: &ParseNode) -> AstNode {
        // fungsi IDENTIFIER (params)? COLON type SEMICOLON declarations compound-statement SEMICOLON
        let mut idx = 1; // Skip "fungsi" keyword

        let name = if let NodeType::Terminal(token) = &node.children[idx].node_type {
            token.value.clone()
        } else {
            "unknown".to_string()
        };
        idx += 1;

        // Check for redeclaration
        if let Some(_) = self.symbol_table.lookup_current_scope(&name) {
            self.errors.push(SemanticError::redeclared(name.clone(), None));
        }

        // Enter new block for function
        let block_index = self.symbol_table.enter_block();

        // Check if parameters exist and save the node index
        let param_node_idx = if idx < node.children.len() && matches!(node.children[idx].node_type, NodeType::FormalParameterList) {
            let temp_idx = idx;
            idx += 1;
            Some(temp_idx)
        } else {
            None
        };

        // Skip colon
        idx += 1;

        // Get return type
        let return_type = self.get_type(&node.children[idx]);
        idx += 1;

        // Skip semicolon
        idx += 1;

        // Insert function into symbol table (at parent level)
        self.symbol_table.exit_block();
        let tab_index = self.symbol_table.insert(TabEntry {
            name: name.clone(),
            link: None,
            obj: ObjectKind::Function,
            data_type: return_type.clone(),
            ref_index: Some(block_index),
            normal: true,
            level: self.symbol_table.current_level(),
            address: 0,
        });
        self.symbol_table.enter_block();

        // process parameters
        let params = if let Some(param_idx) = param_node_idx {
            self.visit_formal_parameter_list(&node.children[param_idx])
        } else {
            Vec::new()
        };

        self.current_proc = Some(name.clone());

        // Process declarations
        let declarations = self.visit_declaration_part(&node.children[idx]);
        idx += 1;

        // Process body
        let body = self.visit_compound_statement(&node.children[idx]);

        self.current_proc = None;

        // Exit block
        self.symbol_table.exit_block();

        AstNode::FuncDecl {
            name,
            params,
            return_type,
            declarations,
            body: Box::new(body),
            tab_index,
            block_index,
        }
    }

    /// Visit formal parameter list
    fn visit_formal_parameter_list(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut params = Vec::new();
        let mut i = 1; // Skip '('

        while i < node.children.len() - 1 { // Skip ')'
            // Skip optional 'variabel' keyword
            if let NodeType::Terminal(token) = &node.children[i].node_type {
                if token.value == "variabel" {
                    i += 1; // Skip the 'variabel' keyword
                }
            }
            
            // Get identifier list
            let id_list = self.get_identifier_list(&node.children[i]);
            i += 1;

            // Skip colon
            i += 1;

            // Get type
            let data_type = self.get_type(&node.children[i]);
            i += 1;

            // Insert parameters into symbol table
            let mut tab_indices = Vec::new();
            for name in &id_list {
                let tab_index = self.symbol_table.insert(TabEntry {
                    name: name.clone(),
                    link: None,
                    obj: ObjectKind::Parameter,
                    data_type: data_type.clone(),
                    ref_index: None,
                    normal: true,
                    level: self.symbol_table.current_level(),
                    address: 0,
                });
                tab_indices.push(tab_index);
            }

            params.push(AstNode::ParamDecl {
                names: id_list,
                data_type,
                is_var: false,
                tab_indices,
            });

            // Skip semicolon if present
            if i < node.children.len() - 1
                && matches!(node.children[i].node_type, NodeType::Terminal(_))
            {
                i += 1;
            }
        }

        params
    }

    /// Visit compound statement
    fn visit_compound_statement(&mut self, node: &ParseNode) -> AstNode {
        // mulai statement-list selesai
        if node.children.len() >= 2 {
            let statements = self.visit_statement_list(&node.children[1]);
            let block_index = self.symbol_table.current_block();
            let level = self.symbol_table.current_level();
            return AstNode::Block { 
                statements, 
                block_index,
                level,
            };
        }
        AstNode::Empty
    }

    /// Visit statement list
    fn visit_statement_list(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut statements = Vec::new();

        for child in &node.children {
            if let NodeType::Terminal(_) = child.node_type {
                continue; // Skip semicolons
            }

            let stmt = self.visit_statement(child);
            if !matches!(stmt, AstNode::Empty) {
                statements.push(stmt);
            }
        }

        statements
    }

    /// Visit statement
    fn visit_statement(&mut self, node: &ParseNode) -> AstNode {
        match &node.node_type {
            NodeType::AssignmentStatement => self.visit_assignment_statement(node),
            NodeType::IfStatement => self.visit_if_statement(node),
            NodeType::WhileStatement => self.visit_while_statement(node),
            NodeType::ForStatement => self.visit_for_statement(node),
            NodeType::ProcedureOrFunctionCall => self.visit_procedure_call(node),
            NodeType::CompoundStatement => self.visit_compound_statement(node),
            _ => AstNode::Empty,
        }
    }

    /// Visit assignment statement
    fn visit_assignment_statement(&mut self, node: &ParseNode) -> AstNode {
        // IDENTIFIER := expression
        let var_name = if let NodeType::Terminal(token) = &node.children[0].node_type {
            token.value.clone()
        } else {
            return AstNode::Empty;
        };

        // Lookup variable
        let tab_index = match self.symbol_table.lookup(&var_name) {
            Some(idx) => idx,
            None => {
                self.errors
                    .push(SemanticError::undeclared(var_name.clone(), None));
                return AstNode::Empty;
            }
        };

        let var_type = self.symbol_table.tab[tab_index].data_type.clone();
        let var_level = self.symbol_table.tab[tab_index].level;

        let target = AstNode::Var {
            name: var_name.clone(),
            data_type: var_type.clone(),
            tab_index,
            level: var_level,
        };

        // Visit value expression
        let value = self.visit_expression(&node.children[2]);
        let value_type = self.get_expr_type(&value);

        // Type check
        if !DataType::can_assign(&var_type, &value_type) {
            self.errors.push(SemanticError::type_mismatch(
                format!("{}", var_type),
                format!("{}", value_type),
                None,
            ));
        }

        AstNode::Assign {
            target: Box::new(target),
            value: Box::new(value),
            data_type: var_type,
        }
    }

    /// Visit if statement
    fn visit_if_statement(&mut self, node: &ParseNode) -> AstNode {
        // jika expression maka statement (selain_itu statement)?
        let condition = self.visit_expression(&node.children[1]);
        let cond_type = self.get_expr_type(&condition);

        // Check condition is boolean
        if cond_type != DataType::Boolean {
            self.errors.push(SemanticError::new(
                SemanticErrorKind::ConditionNotBoolean,
                None,
            ));
        }

        let then_stmt = self.visit_statement(&node.children[3]);

        let else_stmt = if node.children.len() > 5 {
            Some(Box::new(self.visit_statement(&node.children[5])))
        } else {
            None
        };

        AstNode::If {
            condition: Box::new(condition),
            then_stmt: Box::new(then_stmt),
            else_stmt,
        }
    }

    /// Visit while statement
    fn visit_while_statement(&mut self, node: &ParseNode) -> AstNode {
        // selama expression lakukan statement
        let condition = self.visit_expression(&node.children[1]);
        let cond_type = self.get_expr_type(&condition);

        // Check condition is boolean
        if cond_type != DataType::Boolean {
            self.errors.push(SemanticError::new(
                SemanticErrorKind::ConditionNotBoolean,
                None,
            ));
        }

        let body = self.visit_statement(&node.children[3]);

        AstNode::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    /// Visit for statement
    fn visit_for_statement(&mut self, node: &ParseNode) -> AstNode {
        // untuk IDENTIFIER := expression (ke|turun_ke) expression lakukan statement
        let var_name = if let NodeType::Terminal(token) = &node.children[1].node_type {
            token.value.clone()
        } else {
            return AstNode::Empty;
        };

        // Lookup variable
        let tab_index = match self.symbol_table.lookup(&var_name) {
            Some(idx) => idx,
            None => {
                self.errors
                    .push(SemanticError::undeclared(var_name.clone(), None));
                return AstNode::Empty;
            }
        };

        let var_type = self.symbol_table.tab[tab_index].data_type.clone();

        // Check variable is integer
        if var_type != DataType::Integer {
            self.errors.push(SemanticError::new(
                SemanticErrorKind::InvalidLoopVariable,
                None,
            ));
        }

        let start = self.visit_expression(&node.children[3]);
        
        let is_downto = if let NodeType::Terminal(token) = &node.children[4].node_type {
            token.value == "turun_ke"
        } else {
            false
        };

        let end = self.visit_expression(&node.children[5]);
        let body = self.visit_statement(&node.children[7]);

        AstNode::For {
            var_name,
            start: Box::new(start),
            end: Box::new(end),
            is_downto,
            body: Box::new(body),
            tab_index,
        }
    }

    /// Visit procedure call
    fn visit_procedure_call(&mut self, node: &ParseNode) -> AstNode {
        // IDENTIFIER (parameter-list)?
        let name = if let NodeType::Terminal(token) = &node.children[0].node_type {
            token.value.clone()
        } else {
            return AstNode::Empty;
        };

        // Lookup procedure/function (predefined procedures in reserved words)
        let tab_index = match self.symbol_table.lookup(&name) {
            Some(idx) => idx,
            None => {
                self.errors.push(SemanticError::undeclared(name.clone(), None));
                return AstNode::Empty;
            }
        };

        // Get arguments if present
        let args = if node.children.len() > 2 {
            // Has parameters
            let param_list_idx = node
                .children
                .iter()
                .position(|c| matches!(c.node_type, NodeType::ParameterList));
            if let Some(idx) = param_list_idx {
                self.visit_parameter_list(&node.children[idx])
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        AstNode::ProcCall {
            name,
            args,
            tab_index,
        }
    }

    /// Visit parameter list
    fn visit_parameter_list(&mut self, node: &ParseNode) -> Vec<AstNode> {
        let mut args = Vec::new();

        for child in &node.children {
            if let NodeType::Terminal(_) = child.node_type {
                continue; // Skip commas
            }

            if let NodeType::Expression = child.node_type {
                args.push(self.visit_expression(child));
            }
        }

        args
    }

    /// Visit expression
    fn visit_expression(&mut self, node: &ParseNode) -> AstNode {
        // expression -> simple-expression (relational-op simple-expression)?
        if node.children.len() == 1 {
            return self.visit_simple_expression(&node.children[0]);
        } else if node.children.len() == 3 {
            let left = self.visit_simple_expression(&node.children[0]);
            let op = if let NodeType::Terminal(token) = &node.children[1].node_type {
                token.value.clone()
            } else {
                "=".to_string()
            };
            let right = self.visit_simple_expression(&node.children[2]);

            let left_type = self.get_expr_type(&left);
            let right_type = self.get_expr_type(&right);

            let result_type = match DataType::get_relational_result_type(&left_type, &right_type) {
                Ok(t) => t,
                Err(_) => {
                    self.errors.push(SemanticError::invalid_operation(
                        op.clone(),
                        format!("{} and {}", left_type, right_type),
                        None,
                    ));
                    DataType::Unknown
                }
            };

            return AstNode::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                data_type: result_type,
            };
        }

        AstNode::Empty
    }

    /// Visit simple expression
    fn visit_simple_expression(&mut self, node: &ParseNode) -> AstNode {
        // simple-expression -> (sign)? term (additive-op term)*
        let mut i = 0;
        let mut result: Option<AstNode> = None;

        // Check for unary sign
        if let NodeType::Terminal(token) = &node.children[i].node_type {
            if token.value == "+" || token.value == "-" {
                i += 1;
                let operand = self.visit_term(&node.children[i]);
                i += 1;

                if token.value == "-" {
                    let op_type = self.get_expr_type(&operand);
                    result = Some(AstNode::UnaryOp {
                        op: "-".to_string(),
                        operand: Box::new(operand),
                        data_type: op_type,
                    });
                } else {
                    result = Some(operand);
                }
            }
        }

        // If no unary sign, start with first term
        if result.is_none() {
            result = Some(self.visit_term(&node.children[i]));
            i += 1;
        }

        // Process remaining terms with operators
        while i < node.children.len() {
            if let NodeType::Terminal(token) = &node.children[i].node_type {
                let op = token.value.clone();
                i += 1;

                if i < node.children.len() {
                    let right = self.visit_term(&node.children[i]);
                    i += 1;

                    let left = result.unwrap();
                    let left_type = self.get_expr_type(&left);
                    let right_type = self.get_expr_type(&right);

                    let result_type = if op == "atau" {
                        match DataType::get_logical_result_type(&left_type, &right_type) {
                            Ok(t) => t,
                            Err(_) => {
                                self.errors.push(SemanticError::invalid_operation(
                                    op.clone(),
                                    format!("{} and {}", left_type, right_type),
                                    None,
                                ));
                                DataType::Unknown
                            }
                        }
                    } else {
                        match DataType::get_arithmetic_result_type(&left_type, &right_type) {
                            Ok(t) => t,
                            Err(_) => {
                                self.errors.push(SemanticError::invalid_operation(
                                    op.clone(),
                                    format!("{} and {}", left_type, right_type),
                                    None,
                                ));
                                DataType::Unknown
                            }
                        }
                    };

                    result = Some(AstNode::BinOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                        data_type: result_type,
                    });
                }
            } else {
                i += 1;
            }
        }

        result.unwrap_or(AstNode::Empty)
    }

    /// Visit term
    fn visit_term(&mut self, node: &ParseNode) -> AstNode {
        // term -> factor (multiplicative-op factor)*
        let mut result = self.visit_factor(&node.children[0]);
        let mut i = 1;

        while i < node.children.len() {
            if let NodeType::Terminal(token) = &node.children[i].node_type {
                let op = token.value.clone();
                i += 1;

                if i < node.children.len() {
                    let right = self.visit_factor(&node.children[i]);
                    i += 1;

                    let left = result;
                    let left_type = self.get_expr_type(&left);
                    let right_type = self.get_expr_type(&right);

                    let result_type = if op == "dan" {
                        match DataType::get_logical_result_type(&left_type, &right_type) {
                            Ok(t) => t,
                            Err(_) => {
                                self.errors.push(SemanticError::invalid_operation(
                                    op.clone(),
                                    format!("{} and {}", left_type, right_type),
                                    None,
                                ));
                                DataType::Unknown
                            }
                        }
                    } else {
                        match DataType::get_arithmetic_result_type(&left_type, &right_type) {
                            Ok(t) => t,
                            Err(_) => {
                                self.errors.push(SemanticError::invalid_operation(
                                    op.clone(),
                                    format!("{} and {}", left_type, right_type),
                                    None,
                                ));
                                DataType::Unknown
                            }
                        }
                    };

                    result = AstNode::BinOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                        data_type: result_type,
                    };
                }
            } else {
                i += 1;
            }
        }

        result
    }

    /// Visit factor
    fn visit_factor(&mut self, node: &ParseNode) -> AstNode {
        if node.children.is_empty() {
            return AstNode::Empty;
        }

        let child = &node.children[0];

        match &child.node_type {
            NodeType::Terminal(token) => match token.token_type {
                TokenType::Number => {
                    // Determine if integer or real
                    if token.value.contains('.') {
                        if let Ok(val) = token.value.parse::<f64>() {
                            return AstNode::Literal {
                                value: LiteralValue::Real(val),
                                data_type: DataType::Real,
                            };
                        }
                    } else {
                        if let Ok(val) = token.value.parse::<i64>() {
                            return AstNode::Literal {
                                value: LiteralValue::Integer(val),
                                data_type: DataType::Integer,
                            };
                        }
                    }
                    AstNode::Empty
                }
                TokenType::CharLiteral => AstNode::Literal {
                    value: LiteralValue::Char(token.value.chars().nth(0).unwrap_or(' ')),
                    data_type: DataType::Char,
                },
                TokenType::StringLiteral => AstNode::Literal {
                    value: LiteralValue::String(token.value.clone()),
                    data_type: DataType::String,
                },
                TokenType::Identifier => {
                    let name = token.value.clone();

                    // Lookup identifier
                    match self.symbol_table.lookup(&name) {
                        Some(idx) => {
                            let entry = &self.symbol_table.tab[idx];
                            AstNode::Var {
                                name: name.clone(),
                                data_type: entry.data_type.clone(),
                                tab_index: idx,
                                level: entry.level,
                            }
                        }
                        None => {
                            self.errors.push(SemanticError::undeclared(name.clone(), None));
                            AstNode::Literal {
                                value: LiteralValue::Integer(0),
                                data_type: DataType::Unknown,
                            }
                        }
                    }
                }
                TokenType::Keyword => {
                    // Handle true/false
                    if token.value == "true" {
                        return AstNode::Literal {
                            value: LiteralValue::Boolean(true),
                            data_type: DataType::Boolean,
                        };
                    } else if token.value == "false" {
                        return AstNode::Literal {
                            value: LiteralValue::Boolean(false),
                            data_type: DataType::Boolean,
                        };
                    }
                    AstNode::Empty
                }
                TokenType::LogicalOperator if token.value == "tidak" => {
                    // Unary not
                    let operand = self.visit_factor(&node.children[1]);
                    let op_type = self.get_expr_type(&operand);

                    if op_type != DataType::Boolean {
                        self.errors.push(SemanticError::invalid_operation(
                            "tidak".to_string(),
                            format!("{}", op_type),
                            None,
                        ));
                    }

                    AstNode::UnaryOp {
                        op: "tidak".to_string(),
                        operand: Box::new(operand),
                        data_type: DataType::Boolean,
                    }
                }
                TokenType::LParenthesis => {
                    // Parenthesized expression
                    self.visit_expression(&node.children[1])
                }
                _ => AstNode::Empty,
            },
            NodeType::ProcedureOrFunctionCall => {
                // Function call
                self.visit_procedure_call(child)
            }
            _ => AstNode::Empty,
        }
    }

    /// Get type from parse tree type node
    fn get_type(&mut self, node: &ParseNode) -> DataType {
        if node.children.is_empty() {
            return DataType::Unknown;
        }

        let child = &node.children[0];

        match &child.node_type {
            NodeType::Terminal(token) => match token.value.as_str() {
                "integer" => DataType::Integer,
                "real" => DataType::Real,
                "boolean" => DataType::Boolean,
                "char" => DataType::Char,
                _ => {
                    // User-defined type or identifier
                    if let Some(idx) = self.symbol_table.lookup(&token.value) {
                        self.symbol_table.tab[idx].data_type.clone()
                    } else {
                        DataType::UserDefined(token.value.clone())
                    }
                }
            },
            NodeType::ArrayType => {
                // larik[range] dari type
                let range_node = &child.children[2];
                let (low, high) = self.get_range(range_node);

                let elem_type = self.get_type(&child.children[5]);

                let elem_size = 1; // Simplified
                let total_size = ((high - low + 1) as usize) * elem_size;

                let atab_index = self.symbol_table.insert_array(ATabEntry {
                    index_type: DataType::Integer,
                    element_type: elem_type.clone(),
                    element_ref: None,
                    low_bound: low,
                    high_bound: high,
                    element_size: elem_size,
                    total_size,
                });

                DataType::Array(atab_index)
            }
            _ => DataType::Unknown,
        }
    }

    /// Get range bounds
    fn get_range(&mut self, node: &ParseNode) -> (i32, i32) {
        // expression .. expression
        let low_expr = self.visit_expression(&node.children[0]);
        let high_expr = self.visit_expression(&node.children[2]);

        let low = self.get_literal_int(&low_expr).unwrap_or(0);
        let high = self.get_literal_int(&high_expr).unwrap_or(0);

        if low > high {
            self.errors.push(SemanticError::new(
                SemanticErrorKind::InvalidArrayBounds,
                None,
            ));
        }

        (low, high)
    }

    /// Get integer value from literal node or unary expression
    fn get_literal_int(&self, node: &AstNode) -> Option<i32> {
        match node {
            AstNode::Literal { value, .. } => {
                if let LiteralValue::Integer(v) = value {
                    return Some(*v as i32);
                }
                None
            }
            AstNode::UnaryOp { op, operand, .. } => {
                // Handle unary + and -
                if let Some(val) = self.get_literal_int(operand) {
                    match op.as_str() {
                        "-" => Some(-val),
                        "+" => Some(val),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            AstNode::Var { tab_index, .. } => {
                // Handle constant references
                let entry = &self.symbol_table.tab[*tab_index];
                if entry.obj == ObjectKind::Constant {
                    // TODO: Store constant values in symbol table
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get identifier list from parse tree
    fn get_identifier_list(&self, node: &ParseNode) -> Vec<String> {
        let mut ids = Vec::new();

        for child in &node.children {
            if let NodeType::Terminal(token) = &child.node_type {
                if token.token_type == TokenType::Identifier {
                    ids.push(token.value.clone());
                }
            }
        }

        ids
    }

    /// Get type of an expression AST node
    fn get_expr_type(&self, node: &AstNode) -> DataType {
        match node {
            AstNode::Literal { data_type, .. } => data_type.clone(),
            AstNode::Var { data_type, .. } => data_type.clone(),
            AstNode::BinOp { data_type, .. } => data_type.clone(),
            AstNode::UnaryOp { data_type, .. } => data_type.clone(),
            AstNode::ProcCall { tab_index, .. } => {
                self.symbol_table.tab[*tab_index].data_type.clone()
            }
            _ => DataType::Unknown,
        }
    }
}
