#!/usr/bin/env apex
# ========================================
# ARRAYS - Comprehensive Guide
# Master array operations in Apex
# ========================================

fn print_header(title: str):
    """Print a section header."""
    print("\n" + "=" * 50)
    print("  " + title)
    print("=" * 50)

fn main():
    print_header("Array Creation & Access")
    
    # Create arrays
    let numbers = [1, 2, 3, 4, 5]
    let strings = ["apple", "banana", "cherry"]
    let mixed = [1, "two", 3.0, true]
    
    print("Numbers: ${numbers}")
    print("Strings: ${strings}")
    print("Mixed: ${mixed}")
    print("Length: ${len(numbers)}")
    print("First: ${numbers[0]}")
    print("Last: ${numbers[-1]}")
    
    # ========================================
    print_header("Array Modification")
    
    let arr = [10, 20, 30]
    arr.push(40)
    print("After push(40): ${arr}")
    
    arr.pop()
    print("After pop(): ${arr}")
    
    arr.unshift(5)
    print("After unshift(5): ${arr}")
    
    arr[1] = 25
    print("After arr[1] = 25: ${arr}")
    
    # ========================================
    print_header("Array Slicing & Searching")
    
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    print("Original: ${data}")
    print("slice(2, 5): ${slice(data, 2, 5)}")
    print("slice(5, -1): ${slice(data, 5, -1)}")
    print("index_of(7): ${index_of(data, 7)}")
    print("contains(5): ${contains(data, 5)}")
    
    # ========================================
    print_header("Array Transformation")
    
    let nums = [1, 2, 3, 4, 5]
    
    # Map - transform each element
    let doubled = map(nums, fn(x) => x * 2)
    print("Doubled: ${doubled}")
    
    # Filter - keep matching elements
    let evens = filter(nums, fn(x) => x % 2 == 0)
    print("Evens: ${evens}")
    
    # Reduce - combine elements
    let sum = reduce(nums, 0, fn(acc, x) => acc + x)
    print("Sum: ${sum}")
    
    let product = reduce(nums, 1, fn(acc, x) => acc * x)
    print("Product: ${product}")
    
    # ========================================
    print_header("Array Chaining (Pipelines)")
    
    let result = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        |> filter(x => x > 3)
        |> map(x => x * x)
        |> reduce(0, (acc, x) => acc + x)
    
    print("Complex pipeline result: ${result}")
    print("Steps:")
    print("  1. filter(x > 3): ${filter([1,2,3,4,5,6,7,8,9,10], fn(x) => x > 3)}")
    print("  2. map(x²): ${map(filter([1,2,3,4,5,6,7,8,9,10], fn(x) => x > 3), fn(x) => x * x)}")
    print("  3. reduce(sum): ${result}")
    
    # ========================================
    print_header("Array Sorting & Reversing")
    
    let unsorted = [5, 2, 8, 1, 9, 3]
    
    print("Original: ${unsorted}")
    print("Sorted: ${sort(unsorted)}")
    print("Reversed: ${reverse(unsorted)}")
    
    let words = ["zebra", "apple", "mango", "banana"]
    print("Words: ${words}")
    print("Sorted: ${sort(words)}")
    
    # ========================================
    print_header("Array Joining & Flattening")
    
    let parts = ["Hello", "Apex", "World"]
    print("Parts: ${parts}")
    print("Joined with ' ': ${join(parts, ' ')}")
    print("Joined with '-': ${join(parts, '-')}")
    
    let nested = [[1, 2], [3, 4], [5, 6]]
    let flat = flatten(nested)
    print("Nested: ${nested}")
    print("Flattened: ${flat}")
    
    # ========================================
    print_header("Array Comparison")
    
    let arr1 = [1, 2, 3]
    let arr2 = [1, 2, 3]
    let arr3 = [1, 2, 4]
    
    print("${arr1} == ${arr2}: ${arr1 == arr2}")
    print("${arr1} == ${arr3}: ${arr1 == arr3}")
    
    # ========================================
    print_header("2D Arrays (Matrix)")
    
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]
    
    print("Matrix:")
    for row in matrix:
        print("  ${row}")
    
    print("Element at [1][2]: ${matrix[1][2]}")
    
    # ========================================
    print("\n✅ Array operations mastered! 🎉")
