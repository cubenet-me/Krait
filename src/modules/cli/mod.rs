// CLI модуль для работы с командной строкой

use std::fs;
use std::path::Path;

/// Результат выполнения CLI команды
pub enum CliResult {
    Success(String),
    Error(String),
}

impl CliResult {
    pub fn print(self) {
        match self {
            CliResult::Success(msg) => println!("✓ {}", msg),
            CliResult::Error(err) => eprintln!("✗ {}", err),
        }
    }
}

/// Транслирует одиночный файл Krait в Rust
pub fn translate_file(input_path: &str, output_path: &str) -> CliResult {
    // Читаем исходный файл
    let source = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            return CliResult::Error(format!("Не могу прочитать файл: {}", e));
        }
    };
    
    // Показываем количество строк и файл
    println!("📄 Файл: {}", input_path);
    println!("📏 Строк: {}", source.lines().count());
    
    // Транслируем код
    let rust_code = match crate::translate(&source) {
        Ok(code) => code,
        Err(e) => {
            return CliResult::Error(format!("Ошибка синтаксического анализа: {}", e));
        }
    };
    
    // Пишем в выходной файл
    if let Err(e) = fs::write(output_path, &rust_code) {
        return CliResult::Error(format!("Не могу написать файл: {}", e));
    }
    
    CliResult::Success(format!("Транслирован в: {}", output_path))
}

/// Транслирует проект (все .kr файлы в директории)
pub fn translate_project(input_dir: &str, output_dir: &str) -> CliResult {
    // Проверяем что входная директория существует
    if !Path::new(input_dir).is_dir() {
        return CliResult::Error(format!("Директория '{}' не существует", input_dir));
    }
    
    // Создаем выходную директорию
    if let Err(e) = fs::create_dir_all(output_dir) {
        return CliResult::Error(format!("Не могу создать директорию: {}", e));
    }
    
    // Ищем все .kr файлы
    let mut translated_count = 0;
    let mut error_count = 0;
    
    match fs::read_dir(input_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("kr") {
                        let input_path = path.to_string_lossy().to_string();
                        let filename = path.file_stem().unwrap().to_string_lossy();
                        let output_path = format!("{}/{}.rs", output_dir, filename);
                        
                        match translate_file(&input_path, &output_path) {
                            CliResult::Success(msg) => {
                                println!("  ✓ {}", msg);
                                translated_count += 1;
                            }
                            CliResult::Error(err) => {
                                println!("  ✗ {}", err);
                                error_count += 1;
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            return CliResult::Error(format!("Ошибка при чтении директории: {}", e));
        }
    }
    
    if error_count > 0 {
        CliResult::Error(format!("Транслировано: {}, Ошибок: {}", translated_count, error_count))
    } else {
        CliResult::Success(format!("Проект транслирован: {} файлов", translated_count))
    }
}

/// Собирает проект: транслирует все .kr файлы и компилирует с cargo
pub fn build_project() -> CliResult {
    println!("🔨 Building Krait project...\n");
    
    // Проверяем наличие исходной директории
    if !std::path::Path::new("krait_src").is_dir() {
        return CliResult::Error("Директория 'krait_src' не найдена".to_string());
    }
    
    // Создаем выходную директорию rust_code
    if let Err(e) = fs::create_dir_all("rust_code") {
        return CliResult::Error(format!("Не могу создать директорию rust_code: {}", e));
    }
    
    // Ищем и транслируем все .kr файлы
    let mut translated_count = 0;
    let mut error_count = 0;
    
    println!("📦 Step 1: Translating Krait files...\n");
    
    match fs::read_dir("krait_src") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("kr") {
                        let input_path = path.to_string_lossy().to_string();
                        let filename = path.file_stem().unwrap().to_string_lossy();
                        let output_path = format!("rust_code/{}.rs", filename);
                        
                        // Читаем исходный файл
                        let source = match fs::read_to_string(&input_path) {
                            Ok(content) => content,
                            Err(e) => {
                                eprintln!("  ✗ Ошибка чтения {}: {}", input_path, e);
                                error_count += 1;
                                continue;
                            }
                        };
                        
                        // Транслируем код
                        let rust_code = match crate::translate(&source) {
                            Ok(code) => code,
                            Err(e) => {
                                eprintln!("  ✗ Ошибка трансляции {}: {}", input_path, e);
                                error_count += 1;
                                continue;
                            }
                        };
                        
                        // Пишем в выходной файл
                        if let Err(e) = fs::write(&output_path, &rust_code) {
                            eprintln!("  ✗ Ошибка записи {}: {}", output_path, e);
                            error_count += 1;
                            continue;
                        }
                        
                        println!("  ✓ {}.kr → {}.rs", filename, filename);
                        translated_count += 1;
                    }
                }
            }
        }
        Err(e) => {
            return CliResult::Error(format!("Ошибка при чтении директории krait_src: {}", e));
        }
    }
    
    println!("\n✅ Транслировано: {} файлов", translated_count);
    
    if error_count > 0 {
        println!("❌ Ошибок: {}\n", error_count);
    }
    
    // Проверяем наличие Cargo.toml в директории проекта
    if !std::path::Path::new("Cargo.toml").is_file() {
        println!("⚠️  Cargo.toml не найден. Пропускаем компиляцию.");
        return CliResult::Success("Трансляция завершена успешно".to_string());
    }
    
    // Компилируем проект с cargo
    println!("🦀 Step 2: Compiling with Cargo...\n");
    
    let output = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Compilation successful!\n");
                CliResult::Success("Проект успешно собран".to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                CliResult::Error(format!("Ошибка компиляции:\n{}", stderr))
            }
        }
        Err(e) => {
            CliResult::Error(format!("Не могу запустить cargo: {}", e))
        }
    }
}

/// Инициализирует новый проект Krait
pub fn init_project(app_name: &str) -> CliResult {
    println!("🚀 Создание нового проекта Krait: {}\n", app_name);
    
    // Создаем основные директории
    let dirs = vec![
        format!("{}/krait_src", app_name),
        format!("{}/rust_code", app_name),
        format!("{}/target", app_name),
    ];
    
    for dir in &dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            return CliResult::Error(format!("Не могу создать директорию {}: {}", dir, e));
        }
        println!("  ✓ Создана папка: {}", dir);
    }
    
    // Создаем Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-rt = "2"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"

[[bin]]
name = "{}"
path = "rust_code/main.rs"
"#,
        app_name, app_name
    );
    
    let cargo_path = format!("{}/Cargo.toml", app_name);
    if let Err(e) = fs::write(&cargo_path, cargo_toml) {
        return CliResult::Error(format!("Не могу создать Cargo.toml: {}", e));
    }
    println!("  ✓ Создан файл: {}/Cargo.toml", app_name);
    
    // Создаем пример main.kr
    let example_kr = r#"// Пример программы на Krait
public func main()
    print("Hello from Krait!")
end
"#;
    
    let main_kr_path = format!("{}/krait_src/main.kr", app_name);
    if let Err(e) = fs::write(&main_kr_path, example_kr) {
        return CliResult::Error(format!("Не могу создать main.kr: {}", e));
    }
    println!("  ✓ Создан файл: {}/krait_src/main.kr", app_name);
    
    // Создаем .gitignore
    let gitignore = r#"target/
rust_code/
*.rs
Cargo.lock
.DS_Store
"#;
    
    let gitignore_path = format!("{}/.gitignore", app_name);
    if let Err(e) = fs::write(&gitignore_path, gitignore) {
        return CliResult::Error(format!("Не могу создать .gitignore: {}", e));
    }
    println!("  ✓ Создан файл: {}/.gitignore", app_name);
    
    // Создаем README.md
    let readme = format!(
        r#"# {}

Проект на языке Krait.

## Структура проекта

- `krait_src/` - исходные файлы на языке Krait (.kr)
- `rust_code/` - сгенерированный Rust код
- `Cargo.toml` - конфигурация Cargo

## Команды

```bash
# Перевести и скомпилировать проект
krait build

# Перевести один файл
krait krait_src/main.kr rust_code/main.rs

# Показать справку
krait --help
```

## Разработка

Редактируй файлы в `krait_src/` и запускай `krait build` для компиляции.
"#,
        app_name
    );
    
    let readme_path = format!("{}/README.md", app_name);
    if let Err(e) = fs::write(&readme_path, readme) {
        return CliResult::Error(format!("Не могу создать README.md: {}", e));
    }
    println!("  ✓ Создан файл: {}/README.md", app_name);
    
    println!("\n✅ Проект успешно создан!");
    println!("\n📚 Следующие шаги:");
    println!("  1. Перейди в директорию: cd {}", app_name);
    println!("  2. Отредактируй файлы в krait_src/");
    println!("  3. Скомпилируй проект: krait build");
    
    CliResult::Success(format!("Проект '{}' создан успешно", app_name))
}

/// Показывает справку
pub fn show_help() {
    println!(
        "Krait to Rust Translator v{}\n\
         \n\
         Использование:\n\
           krait init <app_name>              - Создать новый проект\n\
           krait <input.kr> <output.rs>       - Транслировать один файл\n\
           krait project <src_dir> <out_dir>  - Транслировать директорию\n\
           krait build                        - Собрать проект\n\
           krait --help | -h                  - Показать эту справку\n\
           krait --version | -v               - Показать версию\n\
         \n\
         Примеры:\n\
           krait init myapp\n\
           krait example.kr generated.rs\n\
           krait project ./src ./output\n\
           krait build",
        crate::VERSION
    );
}

/// Показывает версию
pub fn show_version() {
    println!("Krait Translator v{}", crate::VERSION);
}
