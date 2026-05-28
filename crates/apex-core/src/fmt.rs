use crate::{AssignOp, Expr, Literal, Program, Stmt, TokenKind};

pub fn format_program(program: &Program) -> String {
    let mut out = String::new();
    for stmt in &program.statements {
        fmt_stmt(stmt, 0, &mut out);
    }
    out
}

fn fmt_stmt(stmt: &Stmt, indent: usize, out: &mut String) {
    let pad = "    ".repeat(indent);
    match stmt {
        Stmt::Let { name, value, .. } => {
            out.push_str(&format!("{pad}let {name} = {};\n", fmt_expr(value)));
        }
        Stmt::Function { name, params, body } => {
            let args = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ");
            out.push_str(&format!("{pad}fn {name}({args}) {{\n"));
            for s in body {
                fmt_stmt(s, indent + 1, out);
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Stmt::Return(expr) => {
            match expr {
                Some(e) => out.push_str(&format!("{pad}return {};\n", fmt_expr(e))),
                None => out.push_str(&format!("{pad}return;\n")),
            }
        }
        Stmt::If { condition, then_branch, else_branch } => {
            out.push_str(&format!("{pad}if {} {{\n", fmt_expr(condition)));
            for s in then_branch {
                fmt_stmt(s, indent + 1, out);
            }
            out.push_str(&format!("{pad}}}"));
            if let Some(else_branch) = else_branch {
                out.push_str(" else {\n");
                for s in else_branch {
                    fmt_stmt(s, indent + 1, out);
                }
                out.push_str(&format!("{pad}}}\n"));
            } else {
                out.push('\n');
            }
        }
        Stmt::While { condition, body } => {
            out.push_str(&format!("{pad}while {} {{\n", fmt_expr(condition)));
            for s in body {
                fmt_stmt(s, indent + 1, out);
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Stmt::Break => out.push_str(&format!("{pad}break;\n")),
        Stmt::Continue => out.push_str(&format!("{pad}continue;\n")),
        Stmt::Import { path, alias } => {
            let p = match path {
                crate::ImportPath::File(s) => format!("\"{s}\""),
                crate::ImportPath::Module(parts) => parts.join("."),
            };
            if let Some(alias) = alias {
                out.push_str(&format!("{pad}import {p} as {alias};\n"));
            } else {
                out.push_str(&format!("{pad}import {p};\n"));
            }
        }
        Stmt::Block(body) => {
            out.push_str(&format!("{pad}{{\n"));
            for s in body {
                fmt_stmt(s, indent + 1, out);
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Stmt::Expr(expr) => {
            out.push_str(&format!("{pad}{};\n", fmt_expr(expr)));
        }
    }
}

fn fmt_expr(expr: &Expr) -> String {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Number(n) => {
                if n.fract() == 0.0 { format!("{}", *n as i64) } else { n.to_string() }
            }
            Literal::Bool(b) => b.to_string(),
            Literal::String(s) => format!("{s:?}"),
            Literal::Nil => "nil".into(),
        },
        Expr::Variable(name) => name.clone(),
        Expr::Unary { op, right } => format!("{}{}", op, fmt_expr(right)),
        Expr::Binary { left, op, right } | Expr::Logical { left, op, right } => {
            format!("{} {} {}", fmt_expr(left), op, fmt_expr(right))
        }
        Expr::Grouping(e) => format!("({})", fmt_expr(e)),
        Expr::Assign { target, op, value } => {
            let op_s = match op {
                AssignOp::Assign => "=",
                AssignOp::AddAssign => "+=",
                AssignOp::SubAssign => "-=",
                AssignOp::MulAssign => "*=",
                AssignOp::DivAssign => "/=",
            };
            format!("{} {op_s} {}", fmt_expr(target), fmt_expr(value))
        }
        Expr::Call { callee, args } => {
            let args = args.iter().map(fmt_expr).collect::<Vec<_>>().join(", ");
            format!("{}({})", fmt_expr(callee), args)
        }
        Expr::Index { object, index } => format!("{}[{}]", fmt_expr(object), fmt_expr(index)),
        Expr::Member { object, name } => format!("{}.{}", fmt_expr(object), name),
        Expr::Array(items) => {
            let items = items.iter().map(fmt_expr).collect::<Vec<_>>().join(", ");
            format!("[{}]", items)
        }
        Expr::Object { name, fields } => {
            let mut out = String::new();
            if let Some(name) = name { out.push_str(name); out.push(' '); }
            let fields = fields.iter().map(|(k, v)| format!("{k}: {}", fmt_expr(v))).collect::<Vec<_>>().join(", ");
            out.push('{');
            out.push_str(&fields);
            out.push('}');
            out
        }
    }
}
