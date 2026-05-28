use crate::{Environment, Value};
use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

pub fn install_builtins(env: &crate::EnvRef) {
    define(env, "print", Value::NativeFunction(native_print));
    define(env, "println", Value::NativeFunction(native_println));
    define(env, "len", Value::NativeFunction(native_len));
    define(env, "str", Value::NativeFunction(native_str));
    define(env, "num", Value::NativeFunction(native_num));
    define(env, "type", Value::NativeFunction(native_type));
    define(env, "assert", Value::NativeFunction(native_assert));
    define(env, "clock", Value::NativeFunction(native_clock));
    define(env, "keys", Value::NativeFunction(native_keys));
    define(env, "read", Value::NativeFunction(native_read));
    define(env, "write", Value::NativeFunction(native_write));
    define(env, "exists", Value::NativeFunction(native_exists));
    define(env, "std", std_module());
}

fn define(env: &crate::EnvRef, name: &str, value: Value) {
    Environment::define(env, name, value);
}

fn native_print(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let out = args.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
    println!("{out}");
    Ok(Value::Nil)
}

fn native_println(i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    native_print(i, args)
}

fn native_len(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let Some(v) = args.first() else { return Ok(Value::Number(0.0)); };
    let n = match v {
        Value::String(s) => s.chars().count() as f64,
        Value::Array(a) => a.borrow().len() as f64,
        Value::Object(o) => o.borrow().len() as f64,
        _ => 0.0,
    };
    Ok(Value::Number(n))
}

fn native_str(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    Ok(Value::String(args.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ")))
}

fn native_num(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let v = args.first().cloned().unwrap_or(Value::Nil);
    let n = match v {
        Value::Number(n) => n,
        Value::Bool(b) => if b { 1.0 } else { 0.0 },
        Value::String(s) => s.trim().parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    };
    Ok(Value::Number(n))
}

fn native_type(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    Ok(Value::String(args.first().map(|v| v.type_name().to_string()).unwrap_or_else(|| "nil".into())))
}

fn native_assert(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let ok = args.first().map(|v| v.truthy()).unwrap_or(false);
    if ok { return Ok(Value::Nil); }
    let msg = args.get(1).map(|v| v.to_string()).unwrap_or_else(|| "assertion failed".into());
    Err(crate::ApexError::new(msg, 0, 0))
}

fn native_clock(_i: &mut crate::Interpreter, _args: Vec<Value>) -> crate::Result<Value> {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64();
    Ok(Value::Number(secs))
}

fn native_keys(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    if let Some(Value::Object(obj)) = args.first() {
        let keys = obj.borrow().keys().cloned().map(Value::String).collect::<Vec<_>>();
        return Ok(Value::Array(Rc::new(RefCell::new(keys))));
    }
    Ok(Value::Array(Rc::new(RefCell::new(vec![]))))
}

fn native_read(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let path = args.first().map(ToString::to_string).unwrap_or_default();
    let data = fs::read_to_string(path).unwrap_or_default();
    Ok(Value::String(data))
}

fn native_write(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let path = args.get(0).map(ToString::to_string).unwrap_or_default();
    let data = args.get(1).map(ToString::to_string).unwrap_or_default();
    if !path.is_empty() {
        let _ = fs::write(path, data);
    }
    Ok(Value::Bool(true))
}

fn native_exists(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let path = args.first().map(ToString::to_string).unwrap_or_default();
    Ok(Value::Bool(PathBuf::from(path).exists()))
}

fn std_module() -> Value {
    let mut root = HashMap::new();

    let mut io = HashMap::new();
    io.insert("print".into(), Value::NativeFunction(native_print));
    io.insert("println".into(), Value::NativeFunction(native_println));
    io.insert("input".into(), Value::NativeFunction(native_input));
    root.insert("io".into(), Value::Object(Rc::new(RefCell::new(io))));

    let mut math = HashMap::new();
    math.insert("abs".into(), Value::NativeFunction(native_abs));
    math.insert("sqrt".into(), Value::NativeFunction(native_sqrt));
    math.insert("floor".into(), Value::NativeFunction(native_floor));
    math.insert("ceil".into(), Value::NativeFunction(native_ceil));
    math.insert("min".into(), Value::NativeFunction(native_min));
    math.insert("max".into(), Value::NativeFunction(native_max));
    math.insert("pow".into(), Value::NativeFunction(native_pow));
    root.insert("math".into(), Value::Object(Rc::new(RefCell::new(math))));

    let mut fsmod = HashMap::new();
    fsmod.insert("read".into(), Value::NativeFunction(native_read));
    fsmod.insert("write".into(), Value::NativeFunction(native_write));
    fsmod.insert("exists".into(), Value::NativeFunction(native_exists));
    root.insert("fs".into(), Value::Object(Rc::new(RefCell::new(fsmod))));

    let mut timemod = HashMap::new();
    timemod.insert("clock".into(), Value::NativeFunction(native_clock));
    root.insert("time".into(), Value::Object(Rc::new(RefCell::new(timemod))));

    Value::Object(Rc::new(RefCell::new(root)))
}

fn native_input(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    use std::io::{self, Write};
    if let Some(prompt) = args.first() {
        print!("{prompt}");
        let _ = io::stdout().flush();
    }
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    Ok(Value::String(s.trim_end().to_string()))
}

fn native_abs(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    Ok(Value::Number(args.first().and_then(|v| v.as_number()).unwrap_or(0.0).abs()))
}
fn native_sqrt(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    Ok(Value::Number(args.first().and_then(|v| v.as_number()).unwrap_or(0.0).sqrt()))
}
fn native_floor(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    Ok(Value::Number(args.first().and_then(|v| v.as_number()).unwrap_or(0.0).floor()))
}
fn native_ceil(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    Ok(Value::Number(args.first().and_then(|v| v.as_number()).unwrap_or(0.0).ceil()))
}
fn native_min(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let a = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0);
    let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
    Ok(Value::Number(a.min(b)))
}
fn native_max(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let a = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0);
    let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
    Ok(Value::Number(a.max(b)))
}
fn native_pow(_i: &mut crate::Interpreter, args: Vec<Value>) -> crate::Result<Value> {
    let a = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0);
    let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
    Ok(Value::Number(a.powf(b)))
}
