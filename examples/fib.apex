#!/usr/bin/env apex
# ========================================
# FIBONACCI - Multiple Implementations
# Comparing performance & elegance
# ========================================

# Classic recursive implementation
fn fib_recursive(n: i32) -> i32:
    """Recursive Fibonacci - simple but slow."""
    if n <= 1:
        return n
    return fib_recursive(n - 1) + fib_recursive(n - 2)

# Memoized recursive implementation
fn fib_memo(n: i32, memo: {}) -> i32:
    """Optimized recursive with memoization."""
    if n in memo:
        return memo[n]
    
    if n <= 1:
        return n
    
    memo[n] = fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
    return memo[n]

# Iterative implementation (most efficient)
fn fib_iterative(n: i32) -> i32:
    """Iterative Fibonacci - fastest!"""
    if n <= 1:
        return n
    
    mut prev = 0
    mut curr = 1
    
    for i in range(2, n + 1):
        let next = prev + curr
        prev = curr
        curr = next
    
    return curr

# Functional approach with fold
fn fib_functional(n: i32) -> i32:
    """Functional Fibonacci using reduce."""
    if n <= 1:
        return n
    
    let result = range(0, n)
        |> reduce({a: 0, b: 1}, fn(acc, _) => {
            a: acc.b,
            b: acc.a + acc.b
        })
    
    return result.b

fn main():
    let n = 30
    
    print("=" * 50)
    print("Fibonacci Sequence Comparison")
    print("=" * 50)
    
    # Test small value with recursive
    print("\n📊 Method 1: Recursive (Simple)")
    let start1 = clock()
    let fib1 = fib_recursive(20)
    let time1 = clock() - start1
    print("  fib(20) = ${fib1} (${time1:.4f}s)")
    
    # Test with memoization
    print("\n🚀 Method 2: Memoized Recursive (Better)")
    let start2 = clock()
    let fib2 = fib_memo(n, {})
    let time2 = clock() - start2
    print("  fib(${n}) = ${fib2} (${time2:.4f}s)")
    
    # Test iterative
    print("\n⚡ Method 3: Iterative (Fastest)")
    let start3 = clock()
    let fib3 = fib_iterative(n)
    let time3 = clock() - start3
    print("  fib(${n}) = ${fib3} (${time3:.4f}s)")
    
    # Test functional
    print("\n🎨 Method 4: Functional (Elegant)")
    let start4 = clock()
    let fib4 = fib_functional(n)
    let time4 = clock() - start4
    print("  fib(${n}) = ${fib4} (${time4:.4f}s)")
    
    # Print sequence
    print("\n📈 First 15 Fibonacci Numbers:")
    let fibs = []
    for i in range(0, 15):
        fibs.push(fib_iterative(i))
    print("  ${fibs}")
    
    print("\n✅ All methods work! Choose based on your needs.")
    print("   • Recursive: Educational & elegant")
    print("   • Memoized: Good balance")
    print("   • Iterative: Best performance")
    print("   • Functional: Most Apex-like")
