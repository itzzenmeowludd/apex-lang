#!/usr/bin/env apex
# ========================================
# FUNCTIONS & CLOSURES
# Master functional programming in Apex
# ========================================

# Basic function
fn add(a: i32, b: i32) -> i32:
    """Add two numbers."""
    return a + b

# Function with multiple returns
fn divmod(a: i32, b: i32) -> (i32, i32):
    """Return both quotient and remainder."""
    return (a / b, a % b)

# Function with default parameters
fn greet(name: str, greeting: str = "Hello") -> str:
    """Greet someone with a custom greeting."""
    return "${greeting}, ${name}!"

# Lambda (anonymous function)
let multiply = fn(x, y) => x * y

# Higher-order function - takes a function as parameter
fn twice(f, x):
    """Apply function f twice to x."""
    return f(f(x))

# Function that returns a function (closure)
fn create_adder(n):
    """Create a function that adds n to any number."""
    return fn(x) => x + n

# Function composition
fn compose(f, g):
    """Compose two functions: returns f(g(x))."""
    return fn(x) => f(g(x))

# Decorator pattern
fn with_timing(f):
    """Wrap function to measure execution time."""
    return fn(x) => 
        let start = clock()
        let result = f(x)
        let elapsed = clock() - start
        print("⏱️  Execution time: ${elapsed:.4f}s")
        return result

# ========================================
fn main():
    print("=" * 50)
    print("Functions & Closures in Apex")
    print("=" * 50)
    
    # Basic function call
    print("\n1️⃣  Basic Function Call")
    print("add(5, 3) = ${add(5, 3)}")
    
    # Multiple returns
    print("\n2️⃣  Multiple Return Values")
    let q, r = divmod(17, 5)
    print("divmod(17, 5) = (${q}, ${r})")
    
    # Default parameters
    print("\n3️⃣  Default Parameters")
    print("greet('Alice') = ${greet('Alice')}")
    print("greet('Bob', 'Hi') = ${greet('Bob', 'Hi')}")
    
    # Lambda functions
    print("\n4️⃣  Lambda Functions")
    print("multiply(4, 7) = ${multiply(4, 7)}")
    let square = fn(x) => x * x
    print("square(5) = ${square(5)}")
    
    # Higher-order functions
    print("\n5️⃣  Higher-Order Functions")
    let result = twice(fn(x) => x * 2, 3)
    print("twice(fn(x) => x*2, 3) = ${result}")
    
    # Closures
    print("\n6️⃣  Closures")
    let add5 = create_adder(5)
    let add10 = create_adder(10)
    print("add5(3) = ${add5(3)}")
    print("add10(3) = ${add10(3)}")
    
    # Function composition
    print("\n7️⃣  Function Composition")
    let increment = fn(x) => x + 1
    let double = fn(x) => x * 2
    let inc_then_double = compose(double, increment)
    print("compose(double, increment)(5) = ${inc_then_double(5)}")
    
    # Map over array with lambda
    print("\n8️⃣  Functional Array Operations")
    let numbers = [1, 2, 3, 4, 5]
    print("Original: ${numbers}")
    
    let squared = map(numbers, fn(x) => x * x)
    print("map(x²): ${squared}")
    
    let evens = filter(numbers, fn(x) => x % 2 == 0)
    print("filter(even): ${evens}")
    
    let sum = reduce(numbers, 0, fn(acc, x) => acc + x)
    print("reduce(+): ${sum}")
    
    # Pipeline with multiple functions
    print("\n9️⃣  Pipeline Pattern")
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    let processed = data
        |> filter(x => x > 3)           # Keep x > 3
        |> map(x => x * x)              # Square each
        |> filter(x => x < 50)          # Keep x < 50
        |> reduce(0, (a, x) => a + x)   # Sum all
    
    print("Pipeline: filter > map > filter > reduce")
    print("Result: ${processed}")
    
    # Function recursion
    print("\n🔟 Recursive Functions")
    let factorial = fn(n) =>
        if n <= 1 then 1 else n * factorial(n - 1)
    
    print("factorial(5) = ${factorial(5)}")
    print("factorial(10) = ${factorial(10)}")
    
    # Advanced: Function currying
    print("\n1️⃣1️⃣  Currying")
    let curry_add = fn(a) => fn(b) => fn(c) => a + b + c
    print("curry_add(1)(2)(3) = ${curry_add(1)(2)(3)}")
    
    # Advanced: Partial application
    print("\n1️⃣2️⃣  Partial Application")
    let add_and = fn(a, b) => fn(c) => a + b + c
    let partial = add_and(1, 2)
    print("add_and(1, 2)(3) = ${partial(3)}")
    
    print("\n✅ Functions mastered! 🚀")