use crate::ast::*;

pub struct Transpiler;

impl Transpiler {
    // Expression
    fn transpile_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::Variable(name) => name.clone(),
            Expr::Binary { left, op, right } => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                };
                format!(
                    "({} {} {})",
                    Self::transpile_expr(left),
                    op_str,
                    Self::transpile_expr(right)
                )
            }
            // _ => "/* expr not implemented */".into(),
        }
    }

    // Statement
    fn transpile_stmt(stmt: &Stmt) -> String {
        match stmt {
            Stmt::Let { name, value } => {
                // DEV: type to do
                format!("int {} = {};", name, Self::transpile_expr(value))
            }
            Stmt::Return(expr) => {
                format!("return {};", Self::transpile_expr(expr))
            }
            // _ => "// stmt not implemented".into(),
        }
    }

    // Function
    pub fn transpile_function(func: &Function) -> String {
        let mut output = String::new();

        output.push_str(&format!("{} {}() {{\n", func.return_type, func.name));

        for stmt in &func.body {
            let line = Self::transpile_stmt(stmt);
            output.push_str("    ");
            output.push_str(&line);
            output.push('\n');
        }

        output.push_str("}\n");
        output
    }
}
