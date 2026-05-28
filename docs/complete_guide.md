# 📚 Apex Language Guide - Complete Reference

## Table of Contents
1. [Getting Started](#getting-started)
2. [Basic Syntax](#basic-syntax)
3. [Variables & Types](#variables--types)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Data Structures](#data-structures)
7. [Object-Oriented Programming](#object-oriented-programming)
8. [Functional Programming](#functional-programming)
9. [Error Handling](#error-handling)
10. [Modules & Imports](#modules--imports)
11. [Standard Library](#standard-library)
12. [Best Practices](#best-practices)

---

## Getting Started

### Installation
```bash
curl -fsSL https://apex-lang.org/install.sh | bash
# or
cargo install apex-lang
```

### Your First Program
```apex
fn main():
    print("Hello, Apex!")
```

Run with:
```bash
apm hello.apex
```

---

## Basic Syntax

### Comments
```apex
# Single line comment

#* 
   Multi-line comment
   Can span multiple lines
*#
```

### Statements & Expressions
```apex
# Statements (no return value)
print("Hello")

# Expressions (have return value)
let x = if condition then 1 else 2

# Expression with block
let result = {
    let a = 5
    let b = 10
    a + b  # implicit return
}
```

---

## Variables & Types

### Variable Declaration
```apex
# Immutable (preferred)
let x = 5
let name = "Alice"

# Mutable
mut counter = 0
counter = counter + 1

# Type annotations (optional)
let count: i32 = 42
let price: f64 = 19.99
let active: bool = true
let message: str = "Hello"
```

### Type System
```
Primitive Types:
  - i8, i16, i32, i64 (integers)
  - u8, u16, u32, u64 (unsigned)
  - f32, f64 (floats)
  - bool (true/false)
  - str (strings)
  - null (null value)

Compound Types:
  - [] (arrays)
  - {} (objects/dicts)
  - fn (functions)
```

### Type Conversion
```apex
let num = 42
let str_num = num.to_string()

let price = "19.99"
let float_price = price.parse::<f64>()

let text = "true"
let bool_val = text.parse::<bool>()
```

---

## Functions

### Function Definition
```apex
# Basic function
fn add(a: i32, b: i32) -> i32:
    return a + b

# Function with no return
fn greet(name: str):
    print("Hello, ${name}!")

# Function with default parameters
fn greet(name: str, greeting: str = "Hello") -> str:
    return "${greeting}, ${name}!"

# Function with variable arguments
fn sum(numbers: ...i32) -> i32:
    let total = 0
    for num in numbers:
        total = total + num
    return total
```

### Lambda Functions
```apex
# Basic lambda
let square = fn(x) => x * x

# Multiline lambda
let complex = fn(x, y) =>
    let sum = x + y
    sum * 2

# In array operations
let doubled = map([1, 2, 3], fn(x) => x * 2)
```

### Higher-Order Functions
```apex
# Function that takes a function
fn apply(f, x, y):
    return f(x, y)

let result = apply(fn(a, b) => a + b, 5, 3)

# Function that returns a function
fn create_multiplier(n):
    return fn(x) => x * n

let times5 = create_multiplier(5)
let result = times5(10)  # 50
```

---

## Control Flow

### If/Else
```apex
if age < 18:
    print("Minor")
else if age < 65:
    print("Adult")
else:
    print("Senior")

# If as expression
let status = if age >= 18 then "adult" else "minor"
```

### Match Expression
```apex
match value:
    1 => print("One"),
    2 => print("Two"),
    3...10 => print("Three to Ten"),
    _ => print("Other")
```

### Loops
```apex
# While loop
mut i = 0
while i < 10:
    print(i)
    i = i + 1

# For loop
for i in range(0, 10):
    print(i)

# For each
for item in array:
    print(item)

# Enumerate
for i, item in enumerate(array):
    print("${i}: ${item}")

# Break & Continue
for i in range(0, 10):
    if i == 3:
        continue
    if i == 8:
        break
    print(i)
```

---

## Data Structures

### Arrays
```apex
let arr = [1, 2, 3, 4, 5]
print(arr[0])           # 1
print(len(arr))         # 5
print(arr[-1])          # 5 (last element)

# Array operations
arr.push(6)
arr.pop()
arr.unshift(0)
arr.shift()

let slice = arr[1:4]    # [2, 3, 4]
```

### Objects/Dictionaries
```apex
let person = {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}

print(person.name)      # Alice
print(person["age"])    # 30

person.job = "Engineer"
person["city"] = "NYC"
```

### Destructuring
```apex
# Array destructuring
let [first, second, ...rest] = [1, 2, 3, 4, 5]

# Object destructuring
let {name, age, ...other} = {name: "Bob", age: 25, city: "LA"}

# In function parameters
fn process([a, b, c]):
    return a + b + c
```

---

## Object-Oriented Programming

### Class Definition
```apex
class Animal:
    name: str
    age: i32
    
    fn new(name: str, age: i32):
        return {name: name, age: age}
    
    fn speak():
        print("${this.name} makes a sound")
    
    fn have_birthday():
        this.age = this.age + 1
```

### Inheritance
```apex
class Dog(Animal):
    breed: str
    
    fn new(name: str, age: i32, breed: str):
        return {
            name: name,
            age: age,
            breed: breed
        }
    
    fn speak():
        print("${this.name} barks!")
```

### Properties & Methods
```apex
# Accessing properties
print(dog.name)
dog.age = 5

# Calling methods
dog.speak()
dog.have_birthday()
```

---

## Functional Programming

### Map, Filter, Reduce
```apex
let numbers = [1, 2, 3, 4, 5]

# Map - transform each element
let doubled = map(numbers, fn(x) => x * 2)

# Filter - keep matching elements
let evens = filter(numbers, fn(x) => x % 2 == 0)

# Reduce - combine elements
let sum = reduce(numbers, 0, fn(acc, x) => acc + x)
```

### Pipe Operator
```apex
let result = data
    |> filter(x => x > 0)
    |> map(x => x * 2)
    |> reduce(0, (acc, x) => acc + x)
```

### Closures
```apex
fn create_adder(n):
    return fn(x) => x + n

let add5 = create_adder(5)
print(add5(10))  # 15
```

---

## Error Handling

### Result Type
```apex
fn divide(a: i32, b: i32) -> Result<i32, str>:
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)

match divide(10, 2):
    Ok(result) => print("Result: ${result}"),
    Err(error) => print("Error: ${error}")
```

### Try Operator (planned)
```apex
fn example() -> Result<i32, str>:
    let result = divide(10, 2)?
    return Ok(result * 2)
```

### Panic
```apex
if not condition:
    panic("Something went wrong!")
```

---

## Modules & Imports

### Importing Standard Library
```apex
import std.io
import std.fs
import std.json
import std.math
```

### Importing Custom Modules
```apex
import "utils.apex" as utils
import "math/algebra.apex" as algebra

# Use
let result = utils.process(data)
```

### Exporting
```apex
# math_utils.apex
export fn add(a, b):
    return a + b

export fn multiply(a, b):
    return a * b
```

---

## Standard Library

### std.io
```apex
import std.io

io.print("Hello")
io.println("Hello with newline")
let line = io.input("Enter text: ")
io.print_error("Error message")
```

### std.fs
```apex
import std.fs

let content = fs.read("file.txt")
fs.write("output.txt", "content")
let exists = fs.exists("file.txt")
let files = fs.list_dir("./")
fs.delete("file.txt")
```

### std.json
```apex
import std.json

let obj = {name: "Alice", age: 30}
let json_str = json.stringify(obj)

let data = json.parse('{"name": "Bob"}')
print(data.name)
```

### std.math
```apex
import std.math

let pi = math.pi
let abs_val = math.abs(-5)
let sqrt_val = math.sqrt(16)
let power = math.pow(2, 8)
let min_val = math.min(3, 5)
let max_val = math.max(3, 5)
let random = math.random()
```

### std.string
```apex
import std.string

let upper = str.to_upper("hello")
let lower = str.to_lower("HELLO")
let trimmed = str.trim("  hello  ")
let parts = str.split("a,b,c", ",")
let joined = str.join(["a", "b", "c"], "-")
let replaced = str.replace("hello", "l", "L")
```

### std.array
```apex
import std.array

let reversed = arr.reverse([1, 2, 3])
let sorted = arr.sort([3, 1, 2])
let unique = arr.unique([1, 2, 2, 3])
let flattened = arr.flatten([[1, 2], [3, 4]])
let contains_val = arr.contains([1, 2, 3], 2)
```

---

## Best Practices

### 1. Use Meaningful Names
```apex
# Good
let user_name = "Alice"
let total_price = 99.99

# Bad
let x = "Alice"
let tp = 99.99
```

### 2. Prefer Immutability
```apex
# Good - immutable
let result = process(data)

# Avoid - unless necessary
mut counter = 0
counter = counter + 1
```

### 3. Use Type Annotations
```apex
# Good - clear types
fn calculate(price: f64, tax_rate: f64) -> f64:
    return price * (1.0 + tax_rate)

# Less clear
fn calculate(price, tax_rate):
    return price * (1.0 + tax_rate)
```

### 4. Write Small Functions
```apex
# Good - single responsibility
fn validate_email(email: str) -> bool:
    return email.contains("@") and email.contains(".")

fn send_verification(email: str):
    if validate_email(email):
        # send email
```

### 5. Use Pattern Matching
```apex
# Good - explicit handling
match value:
    Some(x) => process(x),
    None => handle_none(),
    _ => default()

# Less clear
if value != null:
    process(value)
```

### 6. Leverage Pipelines
```apex
# Good - clear flow
let result = data
    |> filter(valid)
    |> map(transform)
    |> reduce(combine)

# Less clear
let filtered = filter(data, valid)
let mapped = map(filtered, transform)
let result = reduce(mapped, combine)
```

### 7. Error Handling
```apex
# Good - explicit error handling
fn risky_operation() -> Result<i32, str>:
    if error_condition:
        return Err("Error message")
    return Ok(result)

# Avoid - ignoring errors
fn risky_operation():
    return result  # might be invalid
```

### 8. Documentation
```apex
# Good - documented
fn calculate_total(items: [], tax_rate: f64) -> f64:
    """
    Calculate total price including tax.
    
    Args:
        items: Array of prices
        tax_rate: Tax rate as decimal (0.1 = 10%)
    
    Returns:
        Total price with tax applied
    """
    let subtotal = reduce(items, 0, (acc, x) => acc + x)
    return subtotal * (1.0 + tax_rate)
```

---

## Operator Precedence

Highest to Lowest:
1. `()` `[]` `.` - Function call, indexing, member access
2. `^` - Exponentiation
3. `*` `/` `%` - Multiplication, Division, Modulo
4. `+` `-` - Addition, Subtraction
5. `<` `<=` `>` `>=` - Comparison
6. `==` `!=` - Equality
7. `&&` - Logical AND
8. `||` - Logical OR
9. `=` `+=` `-=` etc. - Assignment

---

## Common Patterns

### Builder Pattern
```apex
class User:
    name: str
    email: str
    age: i32
    
    fn builder():
        return {
            name: "",
            email: "",
            age: 0
        }

let user = User.builder()
    .with_name("Alice")
    .with_email("alice@example.com")
    .with_age(30)
    .build()
```

### Factory Pattern
```apex
class DatabaseConnection:
    pass

fn create_connection(db_type: str):
    match db_type:
        "postgres" => return PostgresConnection.new(),
        "mysql" => return MysqlConnection.new(),
        "sqlite" => return SqliteConnection.new(),
        _ => panic("Unknown database type")
```

### Decorator Pattern
```apex
fn with_logging(f):
    return fn(...args) =>
        print("Calling ${f}")
        let result = f(...args)
        print("Result: ${result}")
        return result

let logged_process = with_logging(process)
```

---

## Tips & Tricks

### String Interpolation
```apex
let name = "Apex"
let version = 2.0
print("Welcome to ${name} v${version}")
```

### Multiple Return Values
```apex
fn divide_with_remainder(a: i32, b: i32) -> (i32, i32):
    return (a / b, a % b)

let q, r = divide_with_remainder(17, 5)
```

### Default Function Parameters
```apex
fn greet(name: str, greeting: str = "Hello") -> str:
    return "${greeting}, ${name}!"

print(greet("Alice"))           # "Hello, Alice!"
print(greet("Bob", "Hi"))       # "Hi, Bob!"
```

### Conditional Assignment
```apex
let status = if is_admin then "admin" else "user"
```

### Array Spread
```apex
let arr1 = [1, 2, 3]
let arr2 = [4, 5, 6]
let combined = [...arr1, ...arr2]  # [1, 2, 3, 4, 5, 6]
```

---

## Troubleshooting

### Common Errors

**Error: Variable not found**
```apex
print(unknownVariable)  # Error: unknownVariable not found

# Fix: Make sure variable is defined
let unknownVariable = "value"
print(unknownVariable)  # Works
```

**Error: Type mismatch**
```apex
let x: i32 = "string"  # Error: Expected i32, got str

# Fix: Use correct type or convert
let x: str = "string"  # Works
let y: i32 = 42        # Works
```

**Error: Function not found**
```apex
undefined_function()  # Error: Function not found

# Fix: Make sure function is defined or imported
fn defined_function():
    print("Hello")

defined_function()  # Works
```

---

**Happy coding with Apex! 🚀**