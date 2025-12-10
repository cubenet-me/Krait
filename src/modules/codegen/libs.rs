// Система управления библиотеками для кодгенерации

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Library {
    pub name: String,
    pub crate_name: String,
    pub imports: Vec<String>,
    pub features: Vec<String>,
}

pub struct LibraryRegistry {
    libraries: HashMap<String, Library>,
}

impl LibraryRegistry {
    pub fn new() -> Self {
        let mut libs = HashMap::new();
        
        // REST API библиотеки
        libs.insert(
            "rest".to_string(),
            Library {
                name: "REST API (Actix)".to_string(),
                crate_name: "actix-web".to_string(),
                imports: vec![
                    "use actix_web::{get, post, put, delete, App, HttpServer, HttpResponse, web};".to_string(),
                ],
                features: vec!["routing".to_string(), "json".to_string()],
            },
        );
        
        // JSON библиотека
        libs.insert(
            "json".to_string(),
            Library {
                name: "JSON".to_string(),
                crate_name: "serde_json".to_string(),
                imports: vec![
                    "use serde_json::{json, Value};".to_string(),
                ],
                features: vec!["serialization".to_string()],
            },
        );
        
        // Database библиотеки
        libs.insert(
            "sqlx".to_string(),
            Library {
                name: "SQLx (SQL Database)".to_string(),
                crate_name: "sqlx".to_string(),
                imports: vec![
                    "use sqlx::{PgPool, Row};".to_string(),
                ],
                features: vec!["async-runtime".to_string()],
            },
        );
        
        libs.insert(
            "mongodb".to_string(),
            Library {
                name: "MongoDB".to_string(),
                crate_name: "mongodb".to_string(),
                imports: vec![
                    "use mongodb::Client;".to_string(),
                ],
                features: vec!["async-runtime".to_string()],
            },
        );
        
        // Async runtime
        libs.insert(
            "tokio".to_string(),
            Library {
                name: "Tokio (Async Runtime)".to_string(),
                crate_name: "tokio".to_string(),
                imports: vec![
                    "use tokio::task;".to_string(),
                ],
                features: vec!["async-runtime".to_string()],
            },
        );
        
        // Logging
        libs.insert(
            "log".to_string(),
            Library {
                name: "Log".to_string(),
                crate_name: "log".to_string(),
                imports: vec![
                    "use log::{info, warn, error};".to_string(),
                ],
                features: vec!["logging".to_string()],
            },
        );
        
        // Serialization
        libs.insert(
            "serde".to_string(),
            Library {
                name: "Serde".to_string(),
                crate_name: "serde".to_string(),
                imports: vec![
                    "use serde::{Deserialize, Serialize};".to_string(),
                ],
                features: vec!["serialization".to_string()],
            },
        );
        
        LibraryRegistry { libraries: libs }
    }
    
    pub fn get(&self, name: &str) -> Option<&Library> {
        self.libraries.get(name)
    }
    
    pub fn add_library(&mut self, name: String, library: Library) {
        self.libraries.insert(name, library);
    }
    
    pub fn list(&self) -> Vec<&String> {
        self.libraries.keys().collect()
    }
    
    pub fn get_imports(&self, names: &[String]) -> Vec<String> {
        let mut imports = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for name in names {
            if let Some(lib) = self.get(name) {
                for imp in &lib.imports {
                    if seen.insert(imp.clone()) {
                        imports.push(imp.clone());
                    }
                }
            }
        }
        
        imports
    }
}

impl Default for LibraryRegistry {
    fn default() -> Self {
        Self::new()
    }
}
