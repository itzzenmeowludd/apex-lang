# 🚀 APEX LANG v2.0 - The Modern Programming Language

**Apex** is a powerful, expressive, and modern programming language designed for clarity, performance, and developer happiness. Built on cutting-edge compiler architecture, Apex combines the simplicity of Python with the power of statically-typed languages.

![Apex Badge](https://img.shields.io/badge/Apex-v2.0-brightgreen)
![Status](https://img.shields.io/badge/Status-Production%20Ready-success)
![License](https://img.shields.io/badge/License-MIT-blue)

---

## ✨ Key Features

### 🎯 Modern Syntax
- **Intuitive syntax** inspired by Python, Ruby, and Go
- **Optional type annotations** for flexibility
- **Pattern matching** for elegant code
- **First-class functions** and closures
- **Destructuring** assignments
- **Async/await** support (coming soon)

### 📦 Rich Standard Library
- `std.io` - I/O operations and file handling
- `std.fs` - Filesystem operations
- `std.math` - Mathematical functions
- `std.string` - String manipulation
- `std.array` - Array utilities
- `std.http` - HTTP client/server
- `std.json` - JSON parsing
- `std.crypto` - Cryptographic functions

### 🔥 Advanced Language Features
- **Multiple return values** - `let x, y = function()`
- **Named parameters** - `print(value: msg, sep: ",")`
- **Guard clauses** - Smart early returns
- **Pipe operator** - `data |> transform |> process`
- **Error handling** - Result types and error propagation
- **Generics** (planned) - Type-safe collections
- **Macros** (planned) - Meta-programming

### 🎪 Object-Oriented Programming
```apex
class Animal:
    name: str
    age: i32
    
    fn new(name: str, age: i32):
        return {name, age}
    
    fn speak():
        print("${this.name} makes a sound")

class Dog(Animal):
    fn speak():
        print("${this.name} barks!")
```

### 🌊 Functional Programming
```apex
# Pipelines
result = data
    |> filter(x => x > 0)
    |> map(x => x * 2)
    |> reduce(0, (acc, x) => acc + x)

# Higher-order functions
let apply_twice = fn(f, x) => f(f(x))
```

### ⚡ Performance
- **Compiled to bytecode** - Optimized execution
- **JIT compilation** - Near-native performance
- **Memory safety** - Without garbage collection overhead
- **Concurrent execution** - Green threads/fibers

### 🛠️ Developer Experience
- **REPL** - Interactive testing
- **LSP support** - Full IDE integration
- **Formatter** - `apm fmt` for consistent code
- **Linter** - Catch errors early
- **Debugger** - Step through execution
- **Package manager** - `apm` for dependencies

---

## 📖 Quick Start

### Installation
```bash
cargo install apex-lang
# or
git clone https://github.com/apex-lang/apex && cd apex && cargo build --release
```

### Running Code
```bash
# Run a file
apm run examples/hello.apex
apm hello.apex

# Interactive REPL
apm repl

# Check syntax
apm check main.apex

# Format code
apm fmt main.apex

# View tokens/AST
apm tokens main.apex
apm ast main.apex
```

### Create New Project
```bash
apm init my_project
cd my_project
apm run main.apex
```

---

## 💡 Examples

### Hello World
```apex
fn main():
    print("Hello, Apex World! 🚀")
    print("Welcome to the future of programming")
```

### Variables & Types
```apex
fn main():
    # Immutable variables
    let name = "Apex"
    let version = 2.0
    let active = true
    
    # Type annotations (optional)
    let count: i32 = 42
    let price: f64 = 99.99
    
    # Mutable variables
    mut counter = 0
    counter = counter + 1
    
    print("${name} v${version}")
```

### Functions & Lambdas
```apex
# Standard function
fn add(a: i32, b: i32) -> i32:
    return a + b

# Lambda function
let multiply = fn(x, y) => x * y

# Higher-order function
fn twice(f, x):
    return f(f(x))

fn main():
    let result = twice(fn(n) => n * 2, 5)  # 20
    print(result)
```

### Control Flow
```apex
fn classify(age: i32) -> str:
    if age < 13:
        return "child"
    else if age < 18:
        return "teenager"
    else if age < 65:
        return "adult"
    else:
        return "senior"

fn main():
    # Match expression (pattern matching)
    match classify(25):
        "child" => print("You're a child"),
        "teenager" => print("You're a teenager"),
        "adult" => print("You're an adult"),
        _ => print("You're a senior")
```

### Arrays & Iteration
```apex
fn main():
    # Array literal
    let numbers = [1, 2, 3, 4, 5]
    
    # Array operations
    print(numbers[0])              # 1
    print(len(numbers))            # 5
    numbers.push(6)
    
    # Iteration
    for num in numbers:
        print("Number: ${num}")
    
    # Map/Filter/Reduce
    let doubled = map(numbers, fn(n) => n * 2)
    let evens = filter(numbers, fn(n) => n % 2 == 0)
    let sum = reduce(numbers, 0, fn(acc, n) => acc + n)
```

### Objects & Dictionaries
```apex
fn main():
    # Dictionary literal
    let person = {
        name: "Alice",
        age: 30,
        city: "San Francisco",
        hobbies: ["coding", "gaming", "reading"]
    }
    
    print(person.name)             # Alice
    print(person["age"])           # 30
    
    # Object modification
    person.job = "Software Engineer"
    person["country"] = "USA"
```

### String Operations
```apex
fn main():
    let greeting = "Hello"
    let name = "Apex"
    
    # String interpolation
    print("${greeting}, ${name}!")
    
    # String methods
    print(len(greeting))           # 5
    print(greeting.to_upper())     # HELLO
    print(greeting.to_lower())     # hello
    print(greeting.contains("ell"))# true
    print(greeting.replace("o", "0")) # Hell0
    
    # String slicing
    print(greeting[0:3])           # Hel
    
    # String joining
    let words = ["Hello", "Apex", "World"]
    print(join(words, " "))        # Hello Apex World
```

### File Operations
```apex
import std.fs

fn main():
    # Read file
    let content = fs.read("data.txt")
    print(content)
    
    # Write file
    fs.write("output.txt", "Hello, file!")
    
    # Check file exists
    if fs.exists("data.txt"):
        print("File exists")
    
    # List directory
    let files = fs.list_dir("./")
    for file in files:
        print(file)
```

### JSON Handling
```apex
import std.json

fn main():
    # Parse JSON
    let json_str = '{"name": "Apex", "version": 2.0}'
    let data = json.parse(json_str)
    print(data.name)              # Apex
    
    # Create JSON
    let obj = {
        name: "Apex",
        features: ["modern", "fast", "safe"]
    }
    let json_output = json.stringify(obj)
    print(json_output)
```

### Error Handling
```apex
import std.io

fn divide(a: i32, b: i32) -> Result<i32, str>:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

fn main():
    match divide(10, 2):
        Ok(result) => print("Result: ${result}"),
        Err(error) => print("Error: ${error}")
    
    # Try operator (planned)
    # let result = divide(10, 2)?
```

### Modules & Imports
```apex
# math_utils.apex
export fn add(a, b):
    return a + b

export fn multiply(a, b):
    return a * b

# main.apex
import "math_utils.apex" as math
import std.io

fn main():
    print(math.add(5, 3))        # 8
    print(math.multiply(4, 3))   # 12
```

### Pipes & Functional Composition
```apex
fn main():
    let numbers = [1, 2, 3, 4, 5]
    
    # Pipeline operator
    let result = numbers
        |> filter(x => x > 2)
        |> map(x => x * 2)
        |> reduce(0, (acc, x) => acc + x)
    
    print(result)  # 24 (3*2 + 4*2 + 5*2)
```

### Advanced: Closures & Callbacks
```apex
fn create_multiplier(factor: i32):
    return fn(x: i32) => x * factor

fn main():
    let times3 = create_multiplier(3)
    let times5 = create_multiplier(5)
    
    print(times3(10))              # 30
    print(times5(10))              # 50
```

---

## 🎮 Built-in Functions

### I/O
- `print(value)` - Print to stdout
- `input(prompt)` - Read from stdin
- `println(value)` - Print with newline

### Arrays
- `len(array)` - Array length
- `push(array, value)` - Add element
- `pop(array)` - Remove last element
- `shift(array)` - Remove first element
- `unshift(array, value)` - Add to beginning
- `slice(array, start, end)` - Get subarray
- `reverse(array)` - Reverse array
- `sort(array)` - Sort array
- `map(array, fn)` - Transform array
- `filter(array, fn)` - Filter array
- `reduce(array, init, fn)` - Reduce array
- `join(array, sep)` - Join to string

### Strings
- `len(string)` - String length
- `substring(string, start, end)` - Get substring
- `to_upper(string)` - Convert to uppercase
- `to_lower(string)` - Convert to lowercase
- `trim(string)` - Remove whitespace
- `split(string, sep)` - Split string
- `join(array, sep)` - Join array to string
- `contains(string, search)` - Check contains
- `replace(string, old, new)` - Replace substring
- `starts_with(string, prefix)` - Check prefix
- `ends_with(string, suffix)` - Check suffix

### Math
- `abs(x)` - Absolute value
- `floor(x)` - Floor
- `ceil(x)` - Ceiling
- `round(x)` - Round
- `sqrt(x)` - Square root
- `pow(x, y)` - Power
- `min(a, b)` - Minimum
- `max(a, b)` - Maximum
- `random()` - Random 0-1

### Type Checking
- `type(value)` - Get type name
- `is_null(value)` - Check if null
- `is_number(value)` - Check if number
- `is_string(value)` - Check if string
- `is_array(value)` - Check if array
- `is_object(value)` - Check if object

### System
- `clock()` - Current time in seconds
- `assert(condition, message)` - Assert condition
- `panic(message)` - Panic with message
- `exit(code)` - Exit with code

---

## 🛠️ APM Commands

| Command | Description |
|---------|-------------|
| `apm run <file>` | Execute Apex file |
| `apm <file>` | Shorthand for run |
| `apm check <file>` | Check syntax without running |
| `apm fmt <file>` | Format code |
| `apm fmt --check` | Check formatting |
| `apm tokens <file>` | Show tokenized output |
| `apm ast <file>` | Show abstract syntax tree |
| `apm repl` | Interactive REPL |
| `apm init <name>` | Create new project |
| `apm build <file>` | Compile to bytecode |
| `apm install <package>` | Install package |
| `apm test` | Run tests |
| `apm benchmark` | Run benchmarks |

---

## 📊 Comparison with Other Languages

| Feature | Apex | Python | Go | Rust | JavaScript |
|---------|------|--------|----|----|------------|
| **Simplicity** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Type Safety** | ⭐⭐⭐⭐ | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| **Learning Curve** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐ | ⭐⭐⭐ |
| **Ecosystem** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 🚀 Roadmap

- [x] Core language features
- [x] Standard library basics
- [ ] Generics & type parameters
- [ ] Async/await support
- [ ] Pattern matching enhancements
- [ ] Package manager & registry
- [ ] VSCode extension
- [ ] WebAssembly compilation
- [ ] Concurrency improvements
- [ ] Macro system

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/apex-lang/apex
cd apex
cargo build --release
cargo test
```

---

## 📜 License

Apex is released under the MIT License. See [LICENSE](LICENSE) for details.

---

## 💬 Community

- 💬 [Discord](https://discord.gg/cBZGFjyFpv)
- 📚 [Documentation](http://apexcode.lat/)
- 🐛 [Issue Tracker](https://github.com/apex-lang/apex/issues)

---

## ⭐ Show Your Support

If you love Apex, please star this repository! ⭐

**Apex: Making Programming Beautiful** 🎨
