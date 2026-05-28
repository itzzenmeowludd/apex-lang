# Apex Lang

Apex is a small, friendly language with:

- `let`, `fn`, `if`, `while`, `return`
- strings, numbers, booleans, arrays, and objects
- built-in `print`, `len`, `assert`, `clock`, `std.io`, `std.fs`, `std.math`
- `import std.io` and `import "file.apex"`
- automatic `main()` entrypoint support

## Quick start

```bash
./apm run examples/hello.apex
./apm hello.apex
./apm repl
```

## Example

```apex
fn main():
    print("Hola desde Apex")
    let nums = [1, 2, 3]
    print(len(nums))
```

## Commands

- `apm run <file>`
- `apm <file>` shorthand for run
- `apm check <file>`
- `apm fmt <file>`
- `apm tokens <file>`
- `apm ast <file>`
- `apm repl`
- `apm init <name>`
# apex-lang
