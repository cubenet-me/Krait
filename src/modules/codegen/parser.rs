// Parser для Krait
use super::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum DataType {
    Int, Float, Double, Txt, Bool, Auto,
}

impl DataType {
    pub fn to_rust(&self) -> &str {
        match self {
            DataType::Int => "i32",
            DataType::Float => "f32",
            DataType::Double => "f64",
            DataType::Txt => "String",
            DataType::Bool => "bool",
            DataType::Auto => "i32",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Identifier(String),
    BinaryOp { left: Box<Expr>, op: String, right: Box<Expr> },
    FunctionCall { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl { name: String, var_type: DataType, value: Option<Expr>, is_public: bool },
    Return(Option<Expr>),
    If { condition: Expr, body: Vec<Statement>, else_body: Option<Vec<Statement>> },
    While { condition: Expr, body: Vec<Statement> },
    For { var: String, start: Expr, end: Expr, body: Vec<Statement> },
    ExprStmt(Expr),
    Try { body: Vec<Statement>, catch_body: Vec<Statement> },
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<(String, DataType)>,
    pub return_type: DataType,
    pub body: Vec<Statement>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct RouteDef {
    pub path: String,
    pub method: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function(FunctionDef),
    Route(RouteDef),
    Statement(Statement),
    Import { module: String, from: String },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    fn current_token(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }
    
    fn peek_token(&self, offset: usize) -> &Token {
        &self.tokens[(self.pos + offset).min(self.tokens.len() - 1)]
    }
    
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
    
    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.current_token().token_type)
            == std::mem::discriminant(&expected)
        {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current_token().token_type))
        }
    }
    
    pub fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut items = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::Eof) {
            items.push(self.parse_top_level()?);
        }
        Ok(items)
    }
    
    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        match &self.current_token().token_type {
            TokenType::Import => self.parse_import(),
            TokenType::Route => self.parse_route(),
            TokenType::Public => {
                self.advance();
                self.parse_function_or_var(true)
            }
            TokenType::Private => {
                self.advance();
                self.parse_function_or_var(false)
            }
            TokenType::Func => self.parse_function_or_var(false),
            _ => self.parse_statement().map(TopLevel::Statement),
        }
    }
    
    fn parse_import(&mut self) -> Result<TopLevel, String> {
        self.expect(TokenType::Import)?;
        let module = match &self.current_token().token_type {
            TokenType::Identifier(name) => {
                let m = name.clone();
                self.advance();
                m
            }
            _ => return Err("Expected module name".to_string()),
        };
        self.expect(TokenType::From)?;
        let from = match &self.current_token().token_type {
            TokenType::Identifier(name) => {
                let f = name.clone();
                self.advance();
                f
            }
            _ => return Err("Expected from module".to_string()),
        };
        Ok(TopLevel::Import { module, from })
    }
    
    fn parse_function_or_var(&mut self, is_public: bool) -> Result<TopLevel, String> {
        if matches!(self.current_token().token_type, TokenType::Func) {
            self.parse_function(is_public)
        } else {
            self.parse_statement().map(TopLevel::Statement)
        }
    }
    
    fn parse_function(&mut self, is_public: bool) -> Result<TopLevel, String> {
        self.expect(TokenType::Func)?;
        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err("Expected function name".to_string()),
        };
        self.expect(TokenType::LeftParen)?;
        let mut params = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::RightParen) {
            let param_name = match &self.current_token().token_type {
                TokenType::Identifier(n) => {
                    let pn = n.clone();
                    self.advance();
                    pn
                }
                _ => return Err("Expected parameter name".to_string()),
            };
            self.expect(TokenType::Colon)?;
            let param_type = self.parse_type()?;
            params.push((param_name, param_type));
            if matches!(self.current_token().token_type, TokenType::Comma) {
                self.advance();
            }
        }
        self.expect(TokenType::RightParen)?;
        let return_type = if matches!(self.current_token().token_type, TokenType::Arrow) {
            self.advance();
            self.parse_type()?
        } else {
            DataType::Auto
        };
        let body = self.parse_block()?;
        Ok(TopLevel::Function(FunctionDef {
            name,
            params,
            return_type,
            body,
            is_public,
        }))
    }
    
    fn parse_route(&mut self) -> Result<TopLevel, String> {
        self.expect(TokenType::Route)?;
        let path = match &self.current_token().token_type {
            TokenType::String(p) => {
                let path = p.clone();
                self.advance();
                path
            }
            _ => return Err("Expected route path".to_string()),
        };
        let method = match &self.current_token().token_type {
            TokenType::Get => { self.advance(); "GET".to_string() }
            TokenType::Post => { self.advance(); "POST".to_string() }
            TokenType::Put => { self.advance(); "PUT".to_string() }
            TokenType::Delete => { self.advance(); "DELETE".to_string() }
            _ => return Err("Expected HTTP method".to_string()),
        };
        let body = self.parse_block()?;
        Ok(TopLevel::Route(RouteDef { path, method, body }))
    }
    
    fn parse_type(&mut self) -> Result<DataType, String> {
        match &self.current_token().token_type {
            TokenType::Int => { self.advance(); Ok(DataType::Int) }
            TokenType::Float => { self.advance(); Ok(DataType::Float) }
            TokenType::Double => { self.advance(); Ok(DataType::Double) }
            TokenType::Txt => { self.advance(); Ok(DataType::Txt) }
            TokenType::Bool => { self.advance(); Ok(DataType::Bool) }
            TokenType::Auto => { self.advance(); Ok(DataType::Auto) }
            _ => Err("Expected data type".to_string()),
        }
    }
    
    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while !matches!(self.current_token().token_type, TokenType::End | TokenType::Eof) {
            statements.push(self.parse_statement()?);
        }
        if matches!(self.current_token().token_type, TokenType::End) {
            self.advance();
        }
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.current_token().token_type {
            TokenType::Return => self.parse_return(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Try => self.parse_try(),
            _ => {
                if self.is_var_decl() {
                    self.parse_var_decl()
                } else {
                    let expr = self.parse_expr()?;
                    Ok(Statement::ExprStmt(expr))
                }
            }
        }
    }
    
    fn is_var_decl(&self) -> bool {
        matches!(self.current_token().token_type, TokenType::Auto)
            || (matches!(self.current_token().token_type, TokenType::Identifier(_))
                && matches!(self.peek_token(1).token_type, TokenType::Identifier(_)))
    }
    
    fn parse_var_decl(&mut self) -> Result<Statement, String> {
        let var_type = self.parse_type()?;
        let name = match &self.current_token().token_type {
            TokenType::Identifier(n) => {
                let vn = n.clone();
                self.advance();
                vn
            }
            _ => return Err("Expected variable name".to_string()),
        };
        let value = if matches!(self.current_token().token_type, TokenType::Equal) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Statement::VarDecl {
            name,
            var_type,
            value,
            is_public: false,
        })
    }
    
    fn parse_return(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::Return)?;
        let expr = if matches!(
            self.current_token().token_type,
            TokenType::End | TokenType::Eof
        ) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        Ok(Statement::Return(expr))
    }
    
    fn parse_if(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::If)?;
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        let else_body = if matches!(self.current_token().token_type, TokenType::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Statement::If { condition, body, else_body })
    }
    
    fn parse_while(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::While)?;
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Statement::While { condition, body })
    }
    
    fn parse_for(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::For)?;
        let var = match &self.current_token().token_type {
            TokenType::Identifier(v) => {
                let vn = v.clone();
                self.advance();
                vn
            }
            _ => return Err("Expected variable name".to_string()),
        };
        self.expect(TokenType::Equal)?;
        let start = self.parse_expr()?;
        self.expect(TokenType::Comma)?;
        let end = self.parse_expr()?;
        let body = self.parse_block()?;
        Ok(Statement::For { var, start, end, body })
    }
    
    fn parse_try(&mut self) -> Result<Statement, String> {
        self.expect(TokenType::Try)?;
        let body = self.parse_block()?;
        self.expect(TokenType::Catch)?;
        let catch_body = self.parse_block()?;
        Ok(Statement::Try { body, catch_body })
    }
    
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while matches!(self.current_token().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: "||".to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while matches!(self.current_token().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: "&&".to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match &self.current_token().token_type {
                TokenType::EqualEqual => "==",
                TokenType::NotEqual => "!=",
                TokenType::Less => "<",
                TokenType::Greater => ">",
                TokenType::LessEqual => "<=",
                TokenType::GreaterEqual => ">=",
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match &self.current_token().token_type {
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;
        loop {
            let op = match &self.current_token().token_type {
                TokenType::Star => "*",
                TokenType::Slash => "/",
                TokenType::Percent => "%",
                _ => break,
            };
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current_token().token_type {
            TokenType::Number(n) => {
                let num = n.clone();
                self.advance();
                Ok(Expr::Literal(num))
            }
            TokenType::String(s) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::Literal(format!("\"{}\"", string)))
            }
            TokenType::Identifier(name) => {
                let id = name.clone();
                self.advance();
                if matches!(self.current_token().token_type, TokenType::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.current_token().token_type, TokenType::RightParen) {
                        args.push(self.parse_expr()?);
                        if matches!(self.current_token().token_type, TokenType::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(TokenType::RightParen)?;
                    Ok(Expr::FunctionCall { name: id, args })
                } else {
                    Ok(Expr::Identifier(id))
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token().token_type)),
        }
    }
}
