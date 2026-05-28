use crate::{ApexError, Expr, Stmt};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

pub type NativeFunc = fn(&mut crate::Interpreter, Vec<Value>) -> Result<Value, ApexError>;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
    Array(Rc<RefCell<Vec<Value>>>),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Function(Rc<Function>),
    NativeFunction(NativeFunc),
}

#[derive(Clone)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: EnvRef,
}

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Clone, Default)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub parent: Option<EnvRef>,
}

impl Environment {
    pub fn new(parent: Option<EnvRef>) -> EnvRef {
        Rc::new(RefCell::new(Self { values: HashMap::new(), parent }))
    }

    pub fn define(env: &EnvRef, name: impl Into<String>, value: Value) {
        env.borrow_mut().values.insert(name.into(), value);
    }

    pub fn get(env: &EnvRef, name: &str) -> Option<Value> {
        if let Some(v) = env.borrow().values.get(name) {
            return Some(v.clone());
        }
        let parent = env.borrow().parent.clone();
        parent.and_then(|p| Environment::get(&p, name))
    }

    pub fn assign(env: &EnvRef, name: &str, value: Value) -> bool {
        if env.borrow().values.contains_key(name) {
            env.borrow_mut().values.insert(name.to_string(), value);
            return true;
        }
        let parent = env.borrow().parent.clone();
        parent.map_or(false, |p| Environment::assign(&p, name, value))
    }
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(false) => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.borrow().is_empty(),
            Value::Object(o) => !o.borrow().is_empty(),
            _ => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Nil => "nil",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => "native",
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self { Value::Number(n) => Some(*n), _ => None }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s:?}"),
            Value::Nil => write!(f, "nil"),
            Value::Array(a) => write!(f, "{:?}", a.borrow()),
            Value::Object(o) => write!(f, "{:?}", o.borrow()),
            Value::Function(func) => write!(f, "<fn {}>", func.name.clone().unwrap_or_else(|| "<anon>".into())),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 { write!(f, "{}", *n as i64) } else { write!(f, "{n}") }
            }
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Nil => write!(f, "nil"),
            Value::Array(arr) => {
                let items = arr.borrow().iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                write!(f, "[{items}]")
            }
            Value::Object(obj) => {
                let items = obj.borrow().iter().map(|(k, v)| format!("{k}: {v}")).collect::<Vec<_>>().join(", ");
                write!(f, "{{{items}}}")
            }
            Value::Function(func) => write!(f, "<fn {}>", func.name.clone().unwrap_or_else(|| "<anon>".into())),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
        }
    }
}
