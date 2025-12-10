# myapp

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
