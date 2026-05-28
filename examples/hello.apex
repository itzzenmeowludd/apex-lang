#!/usr/bin/env apex
# ========================================
# HELLO WORLD - Classic Example
# Welcome to Apex v2.0!
# ========================================

fn greet(name: str) -> str:
    """Generate a personalized greeting."""
    return "Hello, ${name}! 👋"

fn print_banner(text: str):
    """Print a formatted banner."""
    let width = len(text) + 4
    let border = "*" * width
    
    print(border)
    print("* ${text} *")
    print(border)

fn main():
    print_banner("🚀 Welcome to Apex v2.0")
    
    let greeting = greet("World")
    print(greeting)
    
    print("\nTop Features:")
    let features = [
        "Modern, intuitive syntax",
        "Lightning-fast execution",
        "Strong type safety",
        "Easy to learn & master"
    ]
    
    for i, feature in enumerate(features):
        print("  ${i + 1}. ${feature}")
    
    print("\n✨ Start coding with Apex today!")