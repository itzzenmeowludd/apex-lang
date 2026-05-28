# 🤝 Contributing to Apex

First off, thanks for taking the time to contribute! ❤️

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps which reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed after following the steps**
* **Explain which behavior you expected to see instead and why**
* **Include screenshots and animated GIFs if possible**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a step-by-step description of the suggested enhancement**
* **Provide specific examples to demonstrate the steps**
* **Describe the current behavior and expected behavior**
* **Explain why this enhancement would be useful**

### Pull Requests

* Fill in the required template
* Follow the Apex styleguides
* Include appropriate test cases
* Update documentation as needed
* End all files with a newline

## Styleguides

### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line
* Consider starting the commit message with an applicable emoji:
  * 🎨 `:art:` when improving the format/structure of the code
  * 🚀 `:rocket:` when improving performance
  * 📝 `:memo:` when writing docs
  * 🐛 `:bug:` when fixing a bug
  * ✨ `:sparkles:` when introducing new features
  * 🔒 `:lock:` when dealing with security
  * ⬆️ `:arrow_up:` when upgrading dependencies
  * ⬇️ `:arrow_down:` when downgrading dependencies
  * 🧪 `:test_tube:` when adding tests
  * 🎯 `:dart:` when improving focus
  * 🧹 `:broom:` when cleaning up code

### Apex Code Style

```apex
# Use meaningful variable names
let user_count = 10  # Good
let uc = 10          # Bad

# Use comments for complex logic
# Calculate total with tax
let total = subtotal * (1.0 + tax_rate)

# Prefer immutability
let result = process(data)  # Good
mut result = data           # Avoid unless necessary

# Use type annotations
fn calculate(a: i32, b: i32) -> i32:  # Good
fn calculate(a, b):                   # Less clear

# Format functions with newline after signature
fn process(data):
    return transform(data)

# Use meaningful function names
fn validate_email(email: str) -> bool:  # Good
fn check(email: str) -> bool:           # Less clear
```

### Documentation Style

```apex
# Use docstrings for functions
fn process_file(path: str) -> str:
    """
    Process a file and return its transformed content.
    
    Args:
        path: Path to the file to process
    
    Returns:
        The processed file content
    
    Throws:
        FileNotFoundError: If the file doesn't exist
    """
    # implementation
```

## Development Setup

### Prerequisites
* Rust 1.56+
* Git

### Setup Instructions

1. Fork the repository
2. Clone your fork: `git clone https://github.com/itzzenmeowludd/apex-lang`
3. Add upstream: `git remote add upstream https://github.com/apex-lang/apex`
4. Create a branch: `git checkout -b feature/your-feature-name`
5. Build: `cargo build`
6. Test: `cargo test`
7. Make your changes
8. Format: `cargo fmt`
9. Lint: `cargo clippy`
10. Commit: `git commit -am 'Add feature'`
11. Push: `git push origin feature/your-feature-name`
12. Create a Pull Request

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test specific_test_name

# Run benchmarks
cargo bench
```

### Building Documentation

```bash
cargo doc --open
```

## Directory Structure

```
apex/
├── crates/
│   ├── apex-core/          # Core interpreter & runtime
│   │   └── src/
│   │       ├── lib.rs      # Library root
│   │       ├── lexer.rs    # Tokenization
│   │       ├── parser.rs   # AST generation
│   │       ├── interpreter.rs
│   │       ├── ast.rs
│   │       ├── token.rs
│   │       ├── value.rs
│   │       ├── error.rs
│   │       ├── builtins.rs
│   │       └── fmt.rs
│   └── apm/                # Package manager CLI
│       └── src/
│           └── main.rs
├── docs/                   # Documentation
├── examples/               # Example programs
└── README.md
```

## Getting Help

* Check the [documentation](https://docs.apex-lang.org)
* Ask on [GitHub Discussions](https://github.com/apex-lang/apex/discussions)
* Join our [Discord community](https://discord.gg/apexlang)
* Open an [issue](https://github.com/apex-lang/apex/issues)

## Recognition

Contributors will be recognized in:
* CONTRIBUTORS.md
* Release notes
* GitHub's contributor graph
* Project README

Thank you for contributing to Apex! 🎉