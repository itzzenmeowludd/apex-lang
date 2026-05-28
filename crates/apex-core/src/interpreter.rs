use crate::{
    ApexError, AssignOp, Environment, Expr, Function, ImportPath, Literal, Program, Result, Stmt, TokenKind, Value,
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    rc::Rc,
};

pub struct Interpreter {
    pub globals: crate::EnvRef,
    env: crate::EnvRef,
    loaded_files: HashSet<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub auto_main: bool,
}

#[derive(Debug)]
enum ControlFlow {
    None,
    Return(Value),
    Break,
    Continue,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new(None);
        let mut interp = Self {
            globals: globals.clone(),
            env: globals,
            loaded_files: HashSet::new(),
            current_file: None,
            auto_main: true,
        };
        crate::install_builtins(&interp.globals);
        interp
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Value> {
        let saved_env = self.env.clone();
        self.env = self.globals.clone();

        let mut result = Value::Nil;
        for stmt in program.statements {
            match self.exec_stmt(stmt)? {
                ControlFlow::None => {}
                ControlFlow::Return(v) => {
                    result = v;
                    break;
                }
                ControlFlow::Break | ControlFlow::Continue => {
                    self.env = saved_env;
                    return Err(self.runtime_error("break/continue outside loop"));
                }
            }
        }

        if self.auto_main {
            if let Some(Value::Function(func)) = Environment::get(&self.globals, "main") {
                let _ = self.call_function(&func, vec![])?;
            }
        }

        self.env = saved_env;
        Ok(result)
    }

    pub fn execute_source(&mut self, source: &str, file: Option<PathBuf>) -> Result<Value> {
        self.execute_source_with_mode(source, file, true)
    }

    pub fn execute_source_with_mode(
        &mut self,
        source: &str,
        file: Option<PathBuf>,
        auto_main: bool,
    ) -> Result<Value> {
        let prev_current = self.current_file.clone();
        let prev_auto_main = self.auto_main;
        self.current_file = file.clone();
        self.auto_main = auto_main;

        let outcome = (|| -> Result<Value> {
            let mut lexer = crate::Lexer::new(source.to_string());
            if let Some(f) = &file {
                lexer = lexer.with_file(f.display().to_string());
            }
            let tokens = lexer.tokenize()?;

            let mut parser = crate::Parser::new(tokens);
            if let Some(f) = &file {
                parser = parser.with_file(f.display().to_string());
            }
            let program = parser.parse_program()?;

            self.eval_program(program)
        })();

        self.current_file = prev_current;
        self.auto_main = prev_auto_main;
        outcome
    }

    pub fn execute_file(&mut self, path: impl AsRef<Path>) -> Result<Value> {
        let path = path.as_ref().canonicalize().unwrap_or_else(|_| path.as_ref().to_path_buf());
        if self.loaded_files.contains(&path) {
            return Ok(Value::Nil);
        }
        let source = std::fs::read_to_string(&path)
            .map_err(|e| ApexError::new(format!("cannot read file: {e}"), 0, 0).with_file(path.display().to_string()))?;
        self.loaded_files.insert(path.clone());
        self.execute_source_with_mode(&source, Some(path), false)
    }

    fn exec_stmt(&mut self, stmt: Stmt) -> Result<ControlFlow> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let v = self.eval_expr(value)?;
                Environment::define(&self.env, name, v);
                Ok(ControlFlow::None)
            }
            Stmt::Function { name, params, body } => {
                let func = Function {
                    name: Some(name.clone()),
                    params: params.into_iter().map(|p| p.name).collect(),
                    body,
                    closure: self.env.clone(),
                };
                Environment::define(&self.env, name, Value::Function(Rc::new(func)));
                Ok(ControlFlow::None)
            }
            Stmt::Return(expr) => {
                let value = match expr {
                    Some(e) => self.eval_expr(e)?,
                    None => Value::Nil,
                };
                Ok(ControlFlow::Return(value))
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.eval_expr(condition)?.truthy() {
                    self.exec_block(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.exec_block(else_branch)
                } else {
                    Ok(ControlFlow::None)
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    if !self.eval_expr(condition.clone())?.truthy() {
                        break;
                    }
                    match self.exec_block(body.clone())? {
                        ControlFlow::None | ControlFlow::Continue => {}
                        ControlFlow::Break => break,
                        ControlFlow::Return(v) => return Ok(ControlFlow::Return(v)),
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::Break => Ok(ControlFlow::Break),
            Stmt::Continue => Ok(ControlFlow::Continue),
            Stmt::Import { path, alias } => {
                self.exec_import(path, alias)?;
                Ok(ControlFlow::None)
            }
            Stmt::Block(body) => self.exec_block(body),
            Stmt::Expr(expr) => {
                let _ = self.eval_expr(expr)?;
                Ok(ControlFlow::None)
            }
        }
    }

    fn exec_block(&mut self, body: Vec<Stmt>) -> Result<ControlFlow> {
        let previous = self.env.clone();
        self.env = Environment::new(Some(previous.clone()));

        for stmt in body {
            match self.exec_stmt(stmt)? {
                ControlFlow::None => {}
                flow => {
                    self.env = previous;
                    return Ok(flow);
                }
            }
        }

        self.env = previous;
        Ok(ControlFlow::None)
    }

    fn exec_import(&mut self, path: ImportPath, alias: Option<String>) -> Result<()> {
        match path {
            ImportPath::Module(parts) => {
                let value = self.lookup_module(&parts)?;
                let bind_name = alias.unwrap_or_else(|| parts.first().cloned().unwrap_or_default());
                if !bind_name.is_empty() {
                    Environment::define(&self.env, bind_name, value);
                }
            }
            ImportPath::File(p) => {
                let base = self.current_file.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                let resolved = if let Some(parent) = base.parent() {
                    parent.join(p)
                } else {
                    PathBuf::from(p)
                };
                let _ = self.execute_file(resolved)?;
            }
        }
        Ok(())
    }

    fn lookup_module(&self, parts: &[String]) -> Result<Value> {
        let mut value = Environment::get(&self.globals, &parts[0])
            .ok_or_else(|| self.runtime_error(format!("unknown module '{}'", parts[0])))?;

        for part in &parts[1..] {
            value = match value {
                Value::Object(obj) => obj
                    .borrow()
                    .get(part)
                    .cloned()
                    .ok_or_else(|| self.runtime_error(format!("module field '{}' not found", part)))?,
                _ => return Err(self.runtime_error(format!("'{}' is not a module", part))),
            };
        }

        Ok(value)
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Literal(lit) => Ok(match lit {
                Literal::Number(n) => Value::Number(n),
                Literal::Bool(b) => Value::Bool(b),
                Literal::String(s) => Value::String(s),
                Literal::Nil => Value::Nil,
            }),
            Expr::Variable(name) => Environment::get(&self.env, &name)
                .or_else(|| Environment::get(&self.globals, &name))
                .ok_or_else(|| self.runtime_error(format!("undefined variable '{name}'"))),
            Expr::Grouping(e) => self.eval_expr(*e),
            Expr::Unary { op, right } => {
                let r = self.eval_expr(*right)?;
                match op {
                    TokenKind::Minus => Ok(Value::Number(
                        -r.as_number().ok_or_else(|| self.runtime_error("expected number"))?
                    )),
                    TokenKind::Bang | TokenKind::Not => Ok(Value::Bool(!r.truthy())),
                    _ => Err(self.runtime_error("invalid unary operator")),
                }
            }
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(*left)?;
                let r = self.eval_expr(*right)?;
                self.eval_binary(l, op, r)
            }
            Expr::Logical { left, op, right } => {
                let l = self.eval_expr(*left)?;
                match op {
                    TokenKind::Or | TokenKind::OrOr => {
                        if l.truthy() {
                            Ok(l)
                        } else {
                            self.eval_expr(*right)
                        }
                    }
                    TokenKind::And | TokenKind::AndAnd => {
                        if !l.truthy() {
                            Ok(l)
                        } else {
                            self.eval_expr(*right)
                        }
                    }
                    _ => Err(self.runtime_error("invalid logical operator")),
                }
            }
            Expr::Assign { target, op, value } => self.assign_target(*target, op, *value),
            Expr::Call { callee, args } => {
                let callee = self.eval_expr(*callee)?;
                let mut values = Vec::with_capacity(args.len());
                for arg in args {
                    values.push(self.eval_expr(arg)?);
                }
                self.call_value(callee, values)
            }
            Expr::Index { object, index } => {
                let obj = self.eval_expr(*object)?;
                let idx = self.eval_expr(*index)?;
                self.eval_index(obj, idx)
            }
            Expr::Member { object, name } => {
                let obj = self.eval_expr(*object)?;
                self.eval_member(obj, &name)
            }
            Expr::Array(items) => {
                let mut vals = Vec::with_capacity(items.len());
                for item in items {
                    vals.push(self.eval_expr(item)?);
                }
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            }
            Expr::Object { name, fields } => {
                let mut map = HashMap::new();
                if let Some(name) = name {
                    map.insert("__type".to_string(), Value::String(name));
                }
                for (k, v) in fields {
                    map.insert(k, self.eval_expr(v)?);
                }
                Ok(Value::Object(Rc::new(RefCell::new(map))))
            }
        }
    }

    fn assign_target(&mut self, target: Expr, op: AssignOp, value: Expr) -> Result<Value> {
        let rhs = self.eval_expr(value)?;
        match target {
            Expr::Variable(name) => {
                let current = self.get_var(&name)?;
                let new_value = match op {
                    AssignOp::Assign => rhs,
                    AssignOp::AddAssign => self.add_values(current, rhs)?,
                    AssignOp::SubAssign => self.sub_values(current, rhs)?,
                    AssignOp::MulAssign => self.mul_values(current, rhs)?,
                    AssignOp::DivAssign => self.div_values(current, rhs)?,
                };
                if !Environment::assign(&self.env, &name, new_value.clone())
                    && !Environment::assign(&self.globals, &name, new_value.clone())
                {
                    return Err(self.runtime_error(format!("cannot assign to undefined variable '{name}'")));
                }
                Ok(new_value)
            }
            Expr::Member { object, name } => {
                let obj = self.eval_expr(*object)?;
                if let Value::Object(map) = obj {
                    let current = map.borrow().get(&name).cloned().unwrap_or(Value::Nil);
                    let new_value = match op {
                        AssignOp::Assign => rhs,
                        AssignOp::AddAssign => self.add_values(current, rhs)?,
                        AssignOp::SubAssign => self.sub_values(current, rhs)?,
                        AssignOp::MulAssign => self.mul_values(current, rhs)?,
                        AssignOp::DivAssign => self.div_values(current, rhs)?,
                    };
                    map.borrow_mut().insert(name, new_value.clone());
                    Ok(new_value)
                } else {
                    Err(self.runtime_error("member assignment requires object"))
                }
            }
            Expr::Index { object, index } => {
                let obj = self.eval_expr(*object)?;
                let idx = self.eval_expr(*index)?;
                if let (Value::Array(arr), Some(i)) = (obj, idx.as_number()) {
                    let mut vec = arr.borrow_mut();
                    let idx = i as usize;
                    if idx >= vec.len() {
                        return Err(self.runtime_error("index out of bounds"));
                    }
                    let current = vec[idx].clone();
                    let new_value = match op {
                        AssignOp::Assign => rhs,
                        AssignOp::AddAssign => self.add_values(current, rhs)?,
                        AssignOp::SubAssign => self.sub_values(current, rhs)?,
                        AssignOp::MulAssign => self.mul_values(current, rhs)?,
                        AssignOp::DivAssign => self.div_values(current, rhs)?,
                    };
                    vec[idx] = new_value.clone();
                    Ok(new_value)
                } else {
                    Err(self.runtime_error("index assignment requires array and numeric index"))
                }
            }
            _ => Err(self.runtime_error("invalid assignment target")),
        }
    }

    fn call_value(&mut self, callee: Value, args: Vec<Value>) -> Result<Value> {
        match callee {
            Value::NativeFunction(f) => f(self, args),
            Value::Function(func) => self.call_function(&func, args),
            other => Err(self.runtime_error(format!("value of type '{}' is not callable", other.type_name()))),
        }
    }

    fn call_function(&mut self, func: &Rc<Function>, args: Vec<Value>) -> Result<Value> {
        if args.len() != func.params.len() {
            return Err(self.runtime_error(format!(
                "expected {} args, got {}",
                func.params.len(),
                args.len()
            )));
        }

        let previous = self.env.clone();
        self.env = Environment::new(Some(func.closure.clone()));

        for (name, value) in func.params.iter().cloned().zip(args) {
            Environment::define(&self.env, name, value);
        }

        let mut ret = Value::Nil;
        for stmt in func.body.clone() {
            match self.exec_stmt(stmt)? {
                ControlFlow::None => {}
                ControlFlow::Return(v) => {
                    ret = v;
                    break;
                }
                ControlFlow::Break | ControlFlow::Continue => {
                    self.env = previous;
                    return Err(self.runtime_error("break/continue inside function body"));
                }
            }
        }

        self.env = previous;
        Ok(ret)
    }

    fn eval_member(&mut self, object: Value, name: &str) -> Result<Value> {
        match object {
            Value::Object(map) => map
                .borrow()
                .get(name)
                .cloned()
                .ok_or_else(|| self.runtime_error(format!("unknown field '{name}'"))),
            _ => Err(self.runtime_error("member access requires object")),
        }
    }

    fn eval_index(&mut self, object: Value, index: Value) -> Result<Value> {
        let i = index
            .as_number()
            .ok_or_else(|| self.runtime_error("index must be numeric"))? as usize;

        match object {
            Value::Array(arr) => arr
                .borrow()
                .get(i)
                .cloned()
                .ok_or_else(|| self.runtime_error("index out of bounds")),
            Value::String(s) => s
                .chars()
                .nth(i)
                .map(|c| Value::String(c.to_string()))
                .ok_or_else(|| self.runtime_error("index out of bounds")),
            Value::Object(map) => {
                let key = i.to_string();
                map.borrow()
                    .get(&key)
                    .cloned()
                    .ok_or_else(|| self.runtime_error("index not found"))
            }
            _ => Err(self.runtime_error("indexing requires array/string/object")),
        }
    }

    fn eval_binary(&self, left: Value, op: TokenKind, right: Value) -> Result<Value> {
        use TokenKind::*;
        match op {
            Plus => self.add_values(left, right),
            Minus => self.sub_values(left, right),
            Star => self.mul_values(left, right),
            Slash => self.div_values(left, right),
            Percent => self.mod_values(left, right),
            EqualEqual => Ok(Value::Bool(values_equal(&left, &right))),
            BangEqual => Ok(Value::Bool(!values_equal(&left, &right))),
            Less => self.cmp_values(left, right, |a, b| a < b),
            LessEqual => self.cmp_values(left, right, |a, b| a <= b),
            Greater => self.cmp_values(left, right, |a, b| a > b),
            GreaterEqual => self.cmp_values(left, right, |a, b| a >= b),
            _ => Err(self.runtime_error("unsupported binary operator")),
        }
    }

    fn add_values(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            (Value::String(a), b) => Ok(Value::String(a + &b.to_string())),
            (a, Value::String(b)) => Ok(Value::String(a.to_string() + &b)),
            (Value::Array(a), Value::Array(b)) => {
                let mut out = a.borrow().clone();
                out.extend(b.borrow().iter().cloned());
                Ok(Value::Array(Rc::new(RefCell::new(out))))
            }
            (a, b) => Ok(Value::String(format!("{a}{b}"))),
        }
    }

    fn sub_values(&self, left: Value, right: Value) -> Result<Value> {
        Ok(Value::Number(left.as_number().unwrap_or(0.0) - right.as_number().unwrap_or(0.0)))
    }

    fn mul_values(&self, left: Value, right: Value) -> Result<Value> {
        Ok(Value::Number(left.as_number().unwrap_or(0.0) * right.as_number().unwrap_or(0.0)))
    }

    fn div_values(&self, left: Value, right: Value) -> Result<Value> {
        Ok(Value::Number(left.as_number().unwrap_or(0.0) / right.as_number().unwrap_or(1.0)))
    }

    fn mod_values(&self, left: Value, right: Value) -> Result<Value> {
        Ok(Value::Number(left.as_number().unwrap_or(0.0) % right.as_number().unwrap_or(1.0)))
    }

    fn cmp_values<F: FnOnce(f64, f64) -> bool>(&self, left: Value, right: Value, f: F) -> Result<Value> {
        Ok(Value::Bool(f(
            left.as_number().unwrap_or(0.0),
            right.as_number().unwrap_or(0.0),
        )))
    }

    fn get_var(&self, name: &str) -> Result<Value> {
        Environment::get(&self.env, name)
            .or_else(|| Environment::get(&self.globals, name))
            .ok_or_else(|| self.runtime_error(format!("undefined variable '{name}'")))
    }

    fn runtime_error(&self, msg: impl Into<String>) -> ApexError {
        let mut e = ApexError::new(msg, 0, 0);
        if let Some(file) = &self.current_file {
            e.file = Some(file.display().to_string());
        }
        e
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Number(x), Value::Number(y)) => (x - y).abs() < f64::EPSILON,
        (Value::String(x), Value::String(y)) => x == y,
        _ => false,
    }
}
