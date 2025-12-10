// Основной файл Krait Translator
use krait_translator::modules::cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            // Нет аргументов - показываем справку
            cli::show_help();
        }
        2 => {
            // Один аргумент - может быть --help, --version, build
            match args[1].as_str() {
                "--help" | "-h" => cli::show_help(),
                "--version" | "-v" => cli::show_version(),
                "build" => cli::build_project().print(),
                _ => {
                    eprintln!("✗ Ошибка: укажите выходной файл");
                    eprintln!("Использование: {} <input.kr> <output.rs>", args[0]);
                }
            }
        }
        3 => {
            // Два аргумента - может быть init или файл -> файл
            match args[1].as_str() {
                "init" => {
                    let app_name = &args[2];
                    cli::init_project(app_name).print();
                }
                _ => {
                    // Два аргумента - файл -> файл
                    let input = &args[1];
                    let output = &args[2];
                    
                    // Транслируем и показываем результат
                    cli::translate_file(input, output).print();
                }
            }
        }
        4.. => {
            // Три и более аргументов - может быть project команда
            if args[1] == "project" {
                let input_dir = &args[2];
                let output_dir = &args[3];
                
                cli::translate_project(input_dir, output_dir).print();
            } else {
                eprintln!("✗ Неизвестная команда");
                cli::show_help();
            }
        }
        _ => {
            eprintln!("✗ Неверные аргументы");
            cli::show_help();
        }
    }
}
