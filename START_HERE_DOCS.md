# 🐍 Krait Language - Start Here

Welcome to **Krait** - a programming language combining Python's simplicity with Rust's performance and safety!

---

## 📖 Which Document Should I Read?

### ⏱️ Have 15 minutes?
→ Read **QUICKSTART.md**
- Get started immediately
- First working example
- Common tasks

### ⏱️ Have 1 hour?
→ Read **KRAIT_COMPLETE_DOCS.md**
- Complete language reference
- All features explained
- Code examples

### ⏱️ Want to understand how it works?
→ Read **ARCHITECTURE.md**
- Compiler internals
- Module system
- Code generation pipeline

### ⏱️ Want to extend with new libraries?
→ Read **LIBRARY_GUIDE.md**
- Add new Cargo crates
- Step-by-step guide
- Examples

### ⏱️ Lost and need navigation?
→ Read **DOCUMENTATION_INDEX_FINAL.md**
- Find what you need
- Quick reference
- Troubleshooting

---

## 🚀 Fastest Path to Your First App

```bash
# 1. Create project
krait init my_app
cd my_app

# 2. Write code
cat > main.kr << 'EOF'
public func main() -> num
    return 0
end
EOF

# 3. Build and run
krait build
krait run
```

---

## 📚 All Documentation Files

| File | Length | Best For |
|------|--------|----------|
| **KRAIT_COMPLETE_DOCS.md** | 949 lines | Complete reference |
| **QUICKSTART.md** | 200 lines | Getting started |
| **ARCHITECTURE.md** | 280 lines | Understanding internals |
| **LIBRARY_GUIDE.md** | 180 lines | Adding libraries |
| **DOCUMENTATION_INDEX_FINAL.md** | 400 lines | Navigation |
| **FINAL_SUMMARY.md** | 200 lines | Project overview |
| **START_HERE_DOCS.md** | This file | What to read |

---

## 🎯 One-Minute Overview

**Krait** is a language that:
- Looks like Python (easy syntax)
- Compiles to Rust (fast + safe)
- Built for web APIs (Actix-web)
- Easy to extend (library system)

```krait
# Example: REST API
import rest from actix

@route "/api/hello" GET
    return "Hello, World!"
end
```

Generates:
```rust
#[get("/api/hello")]
async fn get_api_hello() -> impl Responder {
    HttpResponse::Ok().json("Hello, World!")
}
```

---

## 🎓 Recommended Reading Order

### Step 1: Quick Start (15 min)
Read: **QUICKSTART.md**
- What is Krait?
- Installation
- First project
- Basic syntax

### Step 2: Language Features (30 min)
Read: **KRAIT_COMPLETE_DOCS.md**
- Types: txt, num, bool, json
- Functions (public/private)
- Variables
- Control flow
- REST API routes

### Step 3: Architecture (if developing) (30 min)
Read: **ARCHITECTURE.md**
- How compiler works
- Lexer → Parser → CodeGen
- Adding new features

### Step 4: Extend with Libraries (as needed)
Read: **LIBRARY_GUIDE.md**
- Add new Cargo crates
- Register in system
- Use in code

---

## 💡 Quick Syntax Reference

```krait
# Comments start with #

# Functions
public func hello(name txt) -> txt
    return "Hello, " + name
end

# Variables
public txt greeting = "Hi"       # Public
private num count = 0           # Private

# Control flow
if x > 10
    return "big"
else
    return "small"
end

# Loops
num i = 0
while i < 10
    i = i + 1
end

# REST API
import rest from actix

@route "/api/users" GET
    return "Users"
end

@route "/api/users/:id" GET
    txt id = path_param("id")
    return "User " + id
end

# Libraries
import json from serde
import db from sqlx
```

---

## 🔧 CLI Commands

```bash
krait init <name>       # Create new project
krait build             # Compile .kr → Rust → Binary
krait run               # Build and run
krait list              # Show commands
```

---

## ❓ FAQ

**Q: Is it ready for production?**
A: Not yet. Currently v0.3.0 (alpha). Wait for v1.0.0.

**Q: Can I use Rust libraries?**
A: Yes! Through the library system in libs.rs.

**Q: What's the performance?**
A: Same as hand-written Rust (native compiled).

**Q: How do I debug?**
A: Check generated Rust in rust_code/ directory.

---

## 🗺️ Document Map

```
START_HERE_DOCS.md (you are here)
    ↓
QUICKSTART.md (first steps)
    ↓
KRAIT_COMPLETE_DOCS.md (full reference)
    ↓
ARCHITECTURE.md (how it works)
    ├→ LIBRARY_GUIDE.md (extend it)
    └→ DOCUMENTATION_INDEX_FINAL.md (navigate)
```

---

## 🎯 Next Steps

1. **Right now**: Open QUICKSTART.md
2. **In 15 minutes**: Have your first app running
3. **Later**: Read KRAIT_COMPLETE_DOCS.md
4. **When needed**: Check specific docs

---

## 📌 Key Points

- 🐍 **Python-like syntax** - Easy to learn
- ⚡ **Rust performance** - Fast execution
- 🛡️ **Memory safe** - No crashes
- 🌐 **REST ready** - Built-in API support
- 📦 **Extensible** - Add libraries easily

---

## 🚨 Common Issues

**Build fails?**
→ Check rust_code/main.rs for errors

**Route not working?**
→ Make sure you did: import rest from actix

**Type error?**
→ Use .to_string() or .to_num() to convert

**Can't find function?**
→ Ensure it's marked as public

---

## 📞 Where to Get Help

1. **Questions about syntax** → KRAIT_COMPLETE_DOCS.md
2. **How to build projects** → QUICKSTART.md
3. **How compiler works** → ARCHITECTURE.md
4. **How to add libraries** → LIBRARY_GUIDE.md
5. **Can't find something** → DOCUMENTATION_INDEX_FINAL.md

---

## ✅ You're Ready!

You now know:
- What Krait is
- Where to find documentation
- What files to read
- How to get started

**Go to QUICKSTART.md and build your first app! 🚀**

---

**Krait v0.3.0** | GPL 3.0 | Python syntax × Rust power 🐍⚡
