#!/bin/bash

# Демонстрационный скрипт использования Backenium транслятора

echo "🚀 Демонстрация Backenium to Rust Translator"
echo "=============================================="
echo ""

# Путь к транслятору
TRANSLATOR="./target/release/bkm_translator"

# Проверка существования транслятора
if [ ! -f "$TRANSLATOR" ]; then
    echo "❌ Ошибка: транслятор не найден!"
    echo "Пожалуйста, выполните: cargo build --release"
    exit 1
fi

echo "✓ Транслятор найден"
echo ""

# Пример 1: Простой файл
echo "📝 Пример 1: Трансляция простого файла (test_simple.bkc)"
echo "---------------------------------------------------------"
$TRANSLATOR test_simple.bkc /tmp/test_simple_output.rs
echo ""
echo "Результат:"
echo "———————————"
head -20 /tmp/test_simple_output.rs
echo "..."
echo ""

# Пример 2: Сложный файл с несколькими функциями
echo "📝 Пример 2: Трансляция сложного файла (example.bkc)"
echo "————————————————————————————————————————————————————————"
$TRANSLATOR example.bkc /tmp/example_output.rs
echo ""
echo "Результат:"
echo "——————————"
head -40 /tmp/example_output.rs
echo "..."
echo ""

echo "✅ Демонстрация завершена!"
echo ""
echo "Дополнительные команды:"
echo "- Трансляция проекта: $TRANSLATOR project <source_dir> <output_dir>"
echo "- Проверка помощи: $TRANSLATOR"
