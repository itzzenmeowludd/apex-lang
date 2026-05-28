#!/usr/bin/env apex
# ========================================
# DATA STRUCTURES & ALGORITHMS
# Common patterns and techniques
# ========================================

# ========================================
# STACK Implementation
# ========================================

class Stack:
    items: []
    
    fn new():
        return {items: []}
    
    fn push(value):
        this.items.push(value)
    
    fn pop():
        if len(this.items) == 0:
            return null
        return this.items.pop()
    
    fn peek():
        if len(this.items) == 0:
            return null
        return this.items[-1]
    
    fn is_empty() -> bool:
        return len(this.items) == 0
    
    fn size() -> i32:
        return len(this.items)

# ========================================
# QUEUE Implementation
# ========================================

class Queue:
    items: []
    
    fn new():
        return {items: []}
    
    fn enqueue(value):
        this.items.push(value)
    
    fn dequeue():
        if len(this.items) == 0:
            return null
        return this.items.shift()
    
    fn front():
        if len(this.items) == 0:
            return null
        return this.items[0]
    
    fn is_empty() -> bool:
        return len(this.items) == 0
    
    fn size() -> i32:
        return len(this.items)

# ========================================
# LINKED LIST Node
# ========================================

class ListNode:
    value: any
    next: any
    
    fn new(value):
        return {value: value, next: null}

# ========================================
# BINARY SEARCH
# ========================================

fn binary_search(arr: [], target):
    """Find index of target in sorted array."""
    let left = 0
    let right = len(arr) - 1
    
    while left <= right:
        let mid = left + (right - left) / 2
        let mid_val = arr[mid]
        
        if mid_val == target:
            return mid
        else if mid_val < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1  # not found

# ========================================
# SORTING ALGORITHMS
# ========================================

# Bubble Sort
fn bubble_sort(arr: []):
    """Sort array using bubble sort."""
    let n = len(arr)
    for i in range(0, n):
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                let temp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = temp
    return arr

# Quick Sort
fn quick_sort(arr: []) -> []:
    """Sort array using quick sort."""
    if len(arr) <= 1:
        return arr
    
    let pivot = arr[0]
    let left = []
    let right = []
    
    for i in range(1, len(arr)):
        if arr[i] < pivot:
            left.push(arr[i])
        else:
            right.push(arr[i])
    
    return quick_sort(left) + [pivot] + quick_sort(right)

# Merge Sort
fn merge(left: [], right: []) -> []:
    """Merge two sorted arrays."""
    let result = []
    let i = 0
    let j = 0
    
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.push(left[i])
            i = i + 1
        else:
            result.push(right[j])
            j = j + 1
    
    # Add remaining elements
    while i < len(left):
        result.push(left[i])
        i = i + 1
    
    while j < len(right):
        result.push(right[j])
        j = j + 1
    
    return result

fn merge_sort(arr: []) -> []:
    """Sort array using merge sort."""
    if len(arr) <= 1:
        return arr
    
    let mid = len(arr) / 2
    let left = arr[0:mid]
    let right = arr[mid:]
    
    return merge(merge_sort(left), merge_sort(right))

# ========================================
# SEARCHING & FILTERING
# ========================================

fn linear_search(arr: [], target) -> i32:
    """Find index of element."""
    for i in range(0, len(arr)):
        if arr[i] == target:
            return i
    return -1

fn find_duplicates(arr: []) -> []:
    """Find all duplicate elements."""
    let seen = {}
    let duplicates = []
    
    for item in arr:
        if item in seen:
            if not (item in duplicates):
                duplicates.push(item)
        else:
            seen[item] = true
    
    return duplicates

fn remove_duplicates(arr: []) -> []:
    """Remove duplicate elements."""
    let seen = {}
    let result = []
    
    for item in arr:
        if not (item in seen):
            result.push(item)
            seen[item] = true
    
    return result

# ========================================
# MATH ALGORITHMS
# ========================================

fn gcd(a: i32, b: i32) -> i32:
    """Greatest Common Divisor using Euclidean algorithm."""
    if b == 0:
        return a
    return gcd(b, a % b)

fn lcm(a: i32, b: i32) -> i32:
    """Least Common Multiple."""
    return (a * b) / gcd(a, b)

fn is_prime(n: i32) -> bool:
    """Check if number is prime."""
    if n < 2:
        return false
    if n == 2:
        return true
    if n % 2 == 0:
        return false
    
    for i in range(3, n, 2):
        if n % i == 0:
            return false
    
    return true

fn sieve_of_eratosthenes(limit: i32) -> []:
    """Find all primes up to limit."""
    if limit < 2:
        return []
    
    let primes = []
    let is_prime_arr = [true] * (limit + 1)
    is_prime_arr[0] = false
    is_prime_arr[1] = false
    
    for i in range(2, limit + 1):
        if is_prime_arr[i]:
            primes.push(i)
            for j in range(i * i, limit + 1, i):
                is_prime_arr[j] = false
    
    return primes

# ========================================
# MAIN DEMO
# ========================================

fn main():
    print("=" * 60)
    print("Data Structures & Algorithms in Apex")
    print("=" * 60)
    
    # STACK
    print("\n1️⃣  STACK - Last In First Out (LIFO)")
    let stack = Stack.new()
    stack.push(1)
    stack.push(2)
    stack.push(3)
    print("Stack after push(1,2,3): [1, 2, 3]")
    print("Peek: ${stack.peek()}")
    print("Pop: ${stack.pop()}")
    print("Size: ${stack.size()}")
    
    # QUEUE
    print("\n2️⃣  QUEUE - First In First Out (FIFO)")
    let queue = Queue.new()
    queue.enqueue("first")
    queue.enqueue("second")
    queue.enqueue("third")
    print("Queue after enqueue: ['first', 'second', 'third']")
    print("Dequeue: ${queue.dequeue()}")
    print("Front: ${queue.front()}")
    print("Size: ${queue.size()}")
    
    # BINARY SEARCH
    print("\n3️⃣  BINARY SEARCH")
    let sorted = [1, 3, 5, 7, 9, 11, 13, 15]
    let target = 7
    let index = binary_search(sorted, target)
    print("Array: ${sorted}")
    print("Search for ${target}: Found at index ${index}")
    
    # SORTING
    print("\n4️⃣  SORTING ALGORITHMS")
    let unsorted = [64, 34, 25, 12, 22, 11, 90]
    print("Original: ${unsorted}")
    print("Bubble sort: ${bubble_sort(unsorted.copy())}")
    print("Quick sort: ${quick_sort(unsorted)}")
    print("Merge sort: ${merge_sort(unsorted)}")
    
    # SEARCH & FILTER
    print("\n5️⃣  SEARCH & FILTER")
    let data = [1, 2, 3, 2, 4, 1, 5]
    print("Data: ${data}")
    print("Linear search for 3: ${linear_search(data, 3)}")
    print("Duplicates: ${find_duplicates(data)}")
    print("Unique: ${remove_duplicates(data)}")
    
    # MATH ALGORITHMS
    print("\n6️⃣  MATH ALGORITHMS")
    print("GCD(48, 18): ${gcd(48, 18)}")
    print("LCM(4, 6): ${lcm(4, 6)}")
    print("Is 17 prime: ${is_prime(17)}")
    print("Is 20 prime: ${is_prime(20)}")
    print("Primes up to 30: ${sieve_of_eratosthenes(30)}")
    
    print("\n✅ Algorithms mastered! 🎓")