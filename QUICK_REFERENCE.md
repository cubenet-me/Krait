# 🚀 Быстрый справочник Backenium

Краткий справочник для быстрого поиска синтаксиса и примеров.

## 📌 Типы данных

```backenium
int x = 42              // целые числа
float y = 3.14          // числа с плавающей запятой
double z = 2.71        // числа двойной точности
txt s = "hello"         // строки
bool b = true           // булевы значения
auto a = 10             // автоматический тип (int)
```

## 🔧 Функции

```backenium
// Простая функция
func hello()
    print("Hi")
end

// С параметрами
func add(int a, int b) -> int
    return a + b
end

// Публичная и приватная
public func public_func()
    print("public")
end

private func private_func()
    print("private")
end
```

## 📝 Переменные

```backenium
// С инициализацией
int count = 5
float price = 9.99
txt name = "John"

// Без инициализации
int x
float y
```

## 🔢 Операции

```backenium
// Арифметика
int sum = 10 + 5       // 15
int diff = 10 - 3      // 7
int prod = 4 * 3       // 12
int div = 20 / 4       // 5
int mod = 20 % 3       // 2

// Сравнение
bool eq = 5 == 5       // true
bool ne = 5 != 3       // true
bool lt = 3 < 5        // true
bool gt = 5 > 3        // true
bool le = 5 <= 5       // true
bool ge = 5 >= 3       // true

// Логика
bool and = true && false    // false
bool or = true || false     // true
```

## 🔀 Управление потоком

```backenium
if x > 10
    print("large")
else
    print("small")
end

// С скобками
if (x > 10) {
    print("large")
} else {
    print("small")
}
```

## 🌐 Web роуты

```backenium
@route "/api/users" GET
    return "users list"
end

@route "/api/users" POST
    int id = 1
    return id
end

@route "/api/users" PUT
    return "updated"
end

@route "/api/users" DELETE
    return "deleted"
end
```

## 🔨 Встроенные функции

```backenium
print("Hello")          // вывод
log("Debug message")    // логирование
txt json = json_encode()   // JSON кодирование
```

## 📚 Полные примеры

### Факториал
```backenium
func factorial(int n) -> int
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)
    end
end

func main()
    int result = factorial(5)
    print(result)  // 120
end
```

### Максимум
```backenium
func max(int a, int b) -> int
    if a > b
        return a
    else
        return b
    end
end

func main()
    int m = max(10, 20)
    print(m)  // 20
end
```

### Классификация
```backenium
func classify(int age) -> txt
    if age < 13
        return "child"
    else
        if age < 18
            return "teen"
        else
            return "adult"
        end
    end
end
```

### Web API
```backenium
public func get_status() -> txt
    return "ok"
end

@route "/api/status" GET
    txt status = get_status()
    return status
end

@route "/api/calculate" POST
    int x = 10
    int y = 20
    int sum = x + y
    return sum
end
```

## 🎯 Таблица операторов

| Оператор | Описание | Пример |
|----------|---------|--------|
| + | Сложение | a + b |
| - | Вычитание | a - b |
| * | Умножение | a * b |
| / | Деление | a / b |
| % | Остаток | a % b |
| == | Равно | a == b |
| != | Не равно | a != b |
| < | Меньше | a < b |
| > | Больше | a > b |
| <= | ≤ | a <= b |
| >= | ≥ | a >= b |
| && | И | a && b |
| &#124;&#124; | ИЛИ | a &#124;&#124; b |

## 📋 Таблица типов

| Backenium | Rust | Диапазон |
|-----------|------|----------|
| int | i32 | -2,147,483,648 to 2,147,483,647 |
| float | f32 | ±3.4e±38 |
| double | f64 | ±1.8e±308 |
| txt | String | любая длина |
| bool | bool | true/false |
| auto | i32 | как int |

## 🔗 Команды CLI

```bash
# Трансляция одного файла
./bkm_translator input.bkc output.rs

# Трансляция проекта
./bkm_translator project /input/dir /output/dir

# Справка
./bkm_translator --help
```

## ✅ Основные правила

✓ Функции и переменные заканчиваются `end` или `}`
✓ Все переменные должны быть типизированы
✓ Функции возвращают значения через `return`
✓ Условия в скобках: `if (condition)` или без `if condition`
✓ Комментарии: `// это комментарий`
✓ Public функции генерируются с `pub fn`
✓ Private функции генерируются как `fn`

## 🐛 Частые ошибки

```backenium
// ❌ Забыли type
x = 10
// ✅ Правильно
int x = 10

// ❌ Не закрыли блок
if x > 5 {
    print("hi")
// ✅ Правильно
if x > 5 {
    print("hi")
}

// ❌ Неправильный return
func add(int a, int b) -> int
    a + b  // забыли return
// ✅ Правильно
func add(int a, int b) -> int
    return a + b
end
```

## 📖 Где больше информации?

- **Полная документация:** `COMPLETE_DOCUMENTATION.md`
- **Навигация:** `DOCUMENTATION_INDEX.md`
- **Примеры:** `example.bkc`, `final_example.bkc`
- **Использование:** `USAGE.md`

---

**Backenium v0.1.1** - Быстрый справочник
