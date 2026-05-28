use apex_core::{format_program, Interpreter, Lexer, Parser, TokenKind};
use std::{env, fs, io::{self, Write}, path::{Path, PathBuf}, process};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || matches!(args[0].as_str(), "-h" | "--help" | "help") {
        print_help();
        return Ok(());
    }

    if args.len() == 1 && looks_like_file(&args[0]) {
        return run_file(Path::new(&args[0]));
    }

    let cmd = args.remove(0);
    match cmd.as_str() {
        "run" => {
            let file = args.get(0).ok_or("missing file")?;
            run_file(Path::new(file))
        }
        "check" => {
            let file = args.get(0).ok_or("missing file")?;
            check_file(Path::new(file))
        }
        "fmt" => {
            let file = args.get(0).ok_or("missing file")?;
            format_file(Path::new(file))
        }
        "tokens" => {
            let file = args.get(0).ok_or("missing file")?;
            dump_tokens(Path::new(file))
        }
        "ast" => {
            let file = args.get(0).ok_or("missing file")?;
            dump_ast(Path::new(file))
        }
        "repl" => repl(),
        "init" => {
            let name = args.get(0).cloned().unwrap_or_else(|| "my-apex-app".into());
            init_project(&name)
        }
        "--version" | "-V" | "version" => {
            println!("apm 0.2.0");
            Ok(())
        }
        _ => {
            if looks_like_file(&cmd) {
                run_file(Path::new(&cmd))
            } else {
                print_help();
                Err("unknown command".into())
            }
        }
    }
}

fn print_help() {
    println!(r#"Apex Code - by itzzenmeowludd

Usage:
  apm run <file>
  apm <file>
  apm check <file>
  apm fmt <file>
  apm tokens <file>
  apm ast <file>
  apm repl
  apm init <name>

Notes:
  - `apm hello.apex` also runs the file.
  - `fn main():` is auto-executed if present.
Comunnity:
https://discord.gg/MTuN483hdA
"#);
}

fn looks_like_file(s: &str) -> bool {
    s.ends_with(".apex") || Path::new(s).exists()
}

fn load_source(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(path)?)
}

fn run_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = load_source(path)?;
    let mut interp = Interpreter::new();
    interp.execute_source(&source, Some(path.to_path_buf()))?;
    Ok(())
}

fn check_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = load_source(path)?;
    let lexer = Lexer::new(source).with_file(path.display().to_string());
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens).with_file(path.display().to_string());
    let _program = parser.parse_program()?;
    println!("OK: {}", path.display());
    Ok(())
}

fn format_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = load_source(path)?;
    let lexer = Lexer::new(source).with_file(path.display().to_string());
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens).with_file(path.display().to_string());
    let program = parser.parse_program()?;
    let formatted = format_program(&program);
    fs::write(path, formatted)?;
    println!("formatted {}", path.display());
    Ok(())
}

fn dump_tokens(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = load_source(path)?;
    let tokens = Lexer::new(source).with_file(path.display().to_string()).tokenize()?;
    for t in tokens {
        println!("{:>4}:{:<3}  {:?}", t.line, t.column, t.kind);
    }
    Ok(())
}

fn dump_ast(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source = load_source(path)?;
    let tokens = Lexer::new(source).with_file(path.display().to_string()).tokenize()?;
    let mut parser = Parser::new(tokens).with_file(path.display().to_string());
    let program = parser.parse_program()?;
    println!("{:#?}", program);
    Ok(())
}

fn repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut interp = Interpreter::new();
    interp.auto_main = false;
    println!("Apex REPL. Ctrl-D to exit.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let tokens = Lexer::new(line).tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program()?;
        match interp.eval_program(program) {
            Ok(v) if !matches!(v, apex_core::Value::Nil) => println!("{v}"),
            Ok(_) => {}
            Err(e) => eprintln!("error: {e}"),
        }
    }
    Ok(())
}

fn init_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(name);
    fs::create_dir_all(dir.join("src"))?;
    fs::write(dir.join("apex.toml"), format!(r#"[package]
name = "{}"
version = "0.1.0"
"#, name))?;
    fs::write(dir.join("src/main.apex"), r#"fn main():
    print("Hello from Apex")
"#)?;
    println!("created {}", dir.display());
    Ok(())
}
