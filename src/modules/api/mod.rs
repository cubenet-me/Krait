// API модуль для Web приложений

use super::codegen::parser::RouteDef;

/// Информация о HTTP API
pub struct ApiInfo {
    pub has_routes: bool,
    pub routes_count: usize,
    pub http_methods: Vec<String>,
}

impl Default for ApiInfo {
    fn default() -> Self {
        ApiInfo {
            has_routes: false,
            routes_count: 0,
            http_methods: Vec::new(),
        }
    }
}

impl ApiInfo {
    /// Анализирует роуты и собирает информацию
    pub fn from_routes(routes: &[&RouteDef]) -> Self {
        let mut methods = Vec::new();
        
        for route in routes {
            if !methods.contains(&route.method) {
                methods.push(route.method.clone());
            }
        }
        
        ApiInfo {
            has_routes: !routes.is_empty(),
            routes_count: routes.len(),
            http_methods: methods,
        }
    }
    
    /// Нужны ли импорты actix_web
    pub fn needs_actix(&self) -> bool {
        self.has_routes
    }
    
    /// Описание API
    pub fn describe(&self) -> String {
        if !self.has_routes {
            "Консольное приложение (CLI)".to_string()
        } else {
            format!(
                "REST API ({} маршрутов, методы: {})",
                self.routes_count,
                self.http_methods.join(", ")
            )
        }
    }
}

/// Конфигурация для Web приложения
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub log_middleware: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            log_middleware: true,
        }
    }
}

impl ApiConfig {
    /// Адрес сервера
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    
    /// URL для браузера
    pub fn url(&self) -> String {
        format!("http://{}", self.address())
    }
    
    /// Генерирует код для Rust
    pub fn to_rust_code(&self) -> String {
        format!(
            r#"    .bind(("{}".to_string(), {}))?
    .run()
    .await"#,
            self.host, self.port
        )
    }
}

/// Проверяет и валидирует маршруты
pub fn validate_routes(routes: &[&RouteDef]) -> Result<(), String> {
    let mut paths = Vec::new();
    
    for route in routes {
        // Проверяем уникальность пути для каждого метода
        let key = format!("{} {}", route.method, route.path);
        if paths.contains(&key) {
            return Err(format!("Дублированный маршрут: {}", key));
        }
        paths.push(key);
        
        // Проверяем валидность пути
        if !route.path.starts_with('/') {
            return Err(format!("Путь должен начинаться с '/': {}", route.path));
        }
    }
    
    Ok(())
}

/// Статистика API
pub fn print_api_stats(routes: &[&RouteDef]) {
    if routes.is_empty() {
        println!("📊 API статистика: нет маршрутов");
        return;
    }
    
    let mut method_counts = std::collections::HashMap::new();
    for route in routes {
        *method_counts.entry(&route.method).or_insert(0) += 1;
    }
    
    println!("📊 API статистика:");
    println!("   ├─ Всего маршрутов: {}", routes.len());
    for (method, count) in method_counts {
        println!("   ├─ {}: {}", method, count);
    }
    println!("   └─ Хост: http://127.0.0.1:8080");
}
