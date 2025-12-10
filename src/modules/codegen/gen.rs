// Генератор Rust кода из AST Krait
use super::parser::{Expr, FunctionDef, RouteDef, Statement, TopLevel, DataType};
use super::libs::LibraryRegistry;
use std::collections::HashSet;

pub struct CodeGenerator {
    output: String,
    indent_level: usize,
    required_libs: HashSet<String>,
    library_registry: LibraryRegistry,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
            required_libs: HashSet::new(),
            library_registry: LibraryRegistry::new(),
        }
    }
    
    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }
    
    fn write_line(&mut self, content: &str) {
        self.output.push_str(&self.indent());
        self.output.push_str(content);
        self.output.push('\n');
    }
    
    fn write(&mut self, content: &str) {
        self.output.push_str(content);
    }
    
    pub fn generate(&mut self, items: &[TopLevel]) -> String {
        let mut functions = Vec::new();
        let mut routes = Vec::new();
        let mut has_main = false;
        
        for item in items {
            match item {
                TopLevel::Import { module: _, from } => {
                    self.required_libs.insert(from.clone());
                }
                TopLevel::Function(func) => {
                    if func.name == "main" {
                        has_main = true;
                    }
                    functions.push(func);
                }
                TopLevel::Route(_route) => {
                    self.required_libs.insert("rest".to_string());
                    routes.push(_route);
                }
                TopLevel::Statement(_stmt) => {}
            }
        }
        
        self.generate_imports();
        self.output.push('\n');
        
        for func in &functions {
            if func.name != "main" {
                self.generate_function(func);
                self.output.push('\n');
            }
        }
        
        for route in &routes {
            self.generate_route_handler(route);
            self.output.push('\n');
        }
        
        if !routes.is_empty() {
            self.generate_main_with_server(&routes);
        } else if has_main {
            for func in &functions {
                if func.name == "main" {
                    self.generate_function(func);
                    break;
                }
            }
        } else {
            self.write_line("fn main() {");
            self.indent_level += 1;
            self.write_line("println!(\"Hello from Krait!\");");
            self.indent_level -= 1;
            self.write_line("}");
        }
        
        self.output.clone()
    }
    
    fn generate_imports(&mut self) {
        self.write_line("// Автоматически сгенерировано из Krait");
        self.output.push('\n');
        
        let libs: Vec<String> = self.required_libs.iter().cloned().collect();
        let imports = self.library_registry.get_imports(&libs);
        
        for import in imports {
            self.write_line(&import);
        }
    }
    
    fn generate_function(&mut self, func: &FunctionDef) {
        let visibility = if func.is_public { "pub " } else { "" };
        let return_type = func.return_type.to_rust();
        
        let params = func.params
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty.to_rust()))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_str = if matches!(func.return_type, DataType::Auto) {
            String::new()
        } else {
            format!(" -> {}", return_type)
        };
        
        self.write_line(&format!("{}fn {}({}){} {{", visibility, func.name, params, return_str));
        self.indent_level += 1;
        
        for stmt in &func.body {
            self.generate_statement(stmt);
        }
        
        self.indent_level -= 1;
        self.write_line("}");
    }
    
    fn generate_route_handler(&mut self, route: &RouteDef) {
        let method = route.method.to_lowercase();
        let path = &route.path;
        
        self.write_line(&format!("#[{}(\"{}\")]", method, path));
        self.write_line("async fn handler() -> HttpResponse {");
        self.indent_level += 1;
        
        for stmt in &route.body {
            self.generate_statement(stmt);
        }
        
        if !route.body.iter().any(|s| matches!(s, Statement::Return(_))) {
            self.write_line("HttpResponse::Ok().finish()");
        }
        
        self.indent_level -= 1;
        self.write_line("}");
    }
    
    fn generate_main_with_server(&mut self, _routes: &[&RouteDef]) {
        self.write_line("#[actix_web::main]");
        self.write_line("async fn main() -> std::io::Result<()> {");
        self.indent_level += 1;
        
        self.write_line("println!(\"Starting API server on http://127.0.0.1:8080\");");
        self.write_line("");
        self.write_line("HttpServer::new(|| {");
        self.indent_level += 1;
        
        self.write_line("App::new()");
        self.indent_level += 1;
        self.write_line(".service(handler)");
        self.indent_level -= 1;
        
        self.indent_level -= 1;
        self.write_line("})");
        self.write_line(".bind(\"127.0.0.1:8080\")?");
        self.write_line(".run()");
        self.write_line(".await");
        
        self.indent_level -= 1;
        self.write_line("}");
    }
    
    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl { name, var_type, value, is_public } => {
                let visibility = if *is_public { "pub " } else { "" };
                let ty = var_type.to_rust();
                
                if let Some(val) = value {
                    let expr_str = self.generate_expr(val);
                    self.write_line(&format!("{}let {} = {};", visibility, name, expr_str));
                } else {
                    self.write_line(&format!("{}let {}: {};", visibility, name, ty));
                }
            }
            Statement::Return(Some(expr)) => {
                let expr_str = self.generate_expr(expr);
                self.write_line(&format!("{}", expr_str));
            }
            Statement::Return(None) => {
                self.write_line("return;");
            }
            Statement::If { condition, body, else_body } => {
                let cond_str = self.generate_expr(condition);
                self.write_line(&format!("if {} {{", cond_str));
                self.indent_level += 1;
                
                for s in body {
                    self.generate_statement(s);
                }
                
                self.indent_level -= 1;
                
                if let Some(else_stmts) = else_body {
                    self.write_line("} else {");
                    self.indent_level += 1;
                    
                    for s in else_stmts {
                        self.generate_statement(s);
                    }
                    
                    self.indent_level -= 1;
                    self.write_line("}");
                } else {
                    self.write_line("}");
                }
            }
            Statement::While { condition, body } => {
                let cond_str = self.generate_expr(condition);
                self.write_line(&format!("while {} {{", cond_str));
                self.indent_level += 1;
                
                for s in body {
                    self.generate_statement(s);
                }
                
                self.indent_level -= 1;
                self.write_line("}");
            }
            Statement::For { var, start, end, body } => {
                let start_str = self.generate_expr(start);
                let end_str = self.generate_expr(end);
                self.write_line(&format!("for {} in {}..{} {{", var, start_str, end_str));
                self.indent_level += 1;
                
                for s in body {
                    self.generate_statement(s);
                }
                
                self.indent_level -= 1;
                self.write_line("}");
            }
            Statement::Try { body, catch_body } => {
                self.write_line("match (|| -> Result<(), Box<dyn std::error::Error>> {");
                self.indent_level += 1;
                
                for s in body {
                    self.generate_statement(s);
                }
                
                self.write_line("Ok(())");
                self.indent_level -= 1;
                self.write_line("})() {");
                
                self.indent_level += 1;
                self.write_line("Ok(_) => {},");
                self.write_line("Err(_) => {");
                self.indent_level += 1;
                
                for s in catch_body {
                    self.generate_statement(s);
                }
                
                self.indent_level -= 1;
                self.write_line("}");
                self.indent_level -= 1;
                self.write_line("}");
            }
            Statement::ExprStmt(expr) => {
                let expr_str = self.generate_expr(expr);
                self.write_line(&format!("{};", expr_str));
            }
        }
    }
    
    fn generate_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(s) => {
                if s.starts_with('"') {
                    format!("\"{}\".to_string()", s.trim_matches('"'))
                } else {
                    s.clone()
                }
            }
            Expr::Identifier(name) => name.clone(),
            Expr::BinaryOp { left, op, right } => {
                let left_str = self.generate_expr(left);
                let right_str = self.generate_expr(right);
                format!("{} {} {}", left_str, op, right_str)
            }
            Expr::FunctionCall { name, args } => {
                let args_str = args
                    .iter()
                    .map(|a| self.generate_expr(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", name, args_str)
            }
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
