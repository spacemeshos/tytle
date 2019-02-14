#[macro_export]
macro_rules! direction {
    ($d:ident) => {{
        $crate::ast::statement::Direction::from(stringify!($d).to_uppercase().as_str())
    }};
}

#[macro_export]
macro_rules! int_lit_expr {
    ($num:expr) => {{
        use $crate::ast::expression::{Expression, LiteralExpr};
        Expression::Literal(LiteralExpr::Int($num))
    }};
}

#[macro_export]
macro_rules! boxed_int_lit_expr {
    ($num:expr) => {{
        Box::new(int_lit_expr!($num))
    }};
}

#[macro_export]
macro_rules! str_lit_expr {
    ($s:expr) => {{
        use $crate::ast::expression::{Expression, LiteralExpr};
        Expression::Literal(LiteralExpr::Str($s.to_string()))
    }};
}

#[macro_export]
macro_rules! var_lit_expr {
    ($s:expr) => {{
        use $crate::ast::expression::{Expression, LiteralExpr};
        Expression::Literal(LiteralExpr::Var($s.to_string()))
    }};
}

#[macro_export]
macro_rules! boxed_var_lit_expr {
    ($s:expr) => {{
        Box::new(var_lit_expr!($s))
    }};
}

#[macro_export]
macro_rules! direct_lit_expr {
    ($dir:ident, $count:expr) => {{
        Statement::Direction($crate::ast::statement::DirectionStmt {
            direction: direction!($dir),
            expr: $crate::ast::expression::Expression::Literal(
                $crate::ast::expression::LiteralExpr::Int($count),
            ),
        })
    }};
}

#[macro_export]
macro_rules! direct_stmt {
    ($dir:ident, $expr:expr) => {{
        Statement::Direction($crate::ast::statement::DirectionStmt {
            direction: direction!($dir),
            expr: $expr,
        })
    }};
}

#[macro_export]
macro_rules! command_stmt {
    ($cmd:ident) => {{
        let cmd_enum = $crate::ast::statement::CommandStmt::from(stringify!($cmd));
        $crate::ast::statement::Statement::Command(cmd_enum)
    }};
}

#[macro_export]
macro_rules! __make_stmt {
    ($kind:expr, $var:expr, $expr:expr) => {{
        use $crate::ast::statement::{MakeStmt, MakeStmtKind, Statement};

        Statement::Make(MakeStmt {
            kind: $kind,
            var: $var.to_string(),
            expr: $expr,
        })
    }};
}

#[macro_export]
macro_rules! make_global_stmt {
    ($var:expr, $expr:expr) => {{
        use $crate::ast::statement::MakeStmtKind;
        __make_stmt!(MakeStmtKind::Global, $var.to_string(), $expr)
    }};
}

#[macro_export]
macro_rules! make_local_stmt {
    ($var:expr, $expr:expr) => {{
        use $crate::ast::statement::MakeStmtKind;
        __make_stmt!(MakeStmtKind::Local, $var.to_string(), $expr)
    }};
}

#[macro_export]
macro_rules! make_stmt {
    ($var:expr, $expr:expr) => {{
        use $crate::ast::statement::MakeStmtKind;
        __make_stmt!(MakeStmtKind::Assign, $var.to_string(), $expr)
    }};
}

#[macro_export]
macro_rules! binary_expr {
    ($op_str:expr, $lexpr:expr, $rexpr:expr) => {{
        use $crate::ast::expression::Expression;

        let op = BinaryOp::from($op_str);
        Expression::Binary(op, $lexpr, $rexpr)
    }};
}

#[macro_export]
macro_rules! block_stmt {
    ($($stmt:expr),*) => {{
        use $crate::ast::statement::BlockStatement;

        let mut block = BlockStatement::new();
        $( block.add_statement($stmt); )*

        block
    }};
}

#[macro_export]
macro_rules! proc_stmt {
    (name: $proc_name:expr, params: [$( $proc_param:expr ),*], returns: $ret_type:ident, body: $block:expr) => {{
        use $crate::ast::statement::{Statement, ProcedureStmt};

        let mut params = vec![];
        $( params.push($proc_param); )*

        let block_stmt = $block;

        let return_type = match stringify!($ret_type) {
            "UNIT" => None,
            v => Some(v.to_string())
        };

        let proc_stmt = Statement::Procedure(ProcedureStmt {
            params,
            name: $proc_name.to_string(),
            return_type,
            block: block_stmt,
        });

        proc_stmt
    }};
}

#[macro_export]
macro_rules! if_stmt {
    (cond: $cond_expr:expr, when_true: $true_block:expr) => {{
        use $crate::ast::statement::{IfStmt, Statement};

        Statement::If(IfStmt {
            cond_expr: $cond_expr,
            true_block: $true_block,
            false_block: None,
        })
    }};

    (cond: $cond_expr:expr, when_true: $true_block:expr, when_false: $false_block:expr) => {{
        use $crate::ast::statement::{IfStmt, Statement};

        Statement::If(IfStmt {
            cond_expr: $cond_expr,
            true_block: $true_block,
            false_block: Some($false_block),
        })
    }};
}

#[macro_export]
macro_rules! repeat_stmt {
    ($count:expr, $block:expr) => {{
        use $crate::ast::statement::{RepeatStmt, Statement};

        Statement::Repeat(RepeatStmt {
            count_expr: $count,
            block: $block,
        })
    }};
}

#[macro_export]
macro_rules! ast {
    ($ ($stmt:expr) ,*) => {
        {
            use $crate::ast::Ast;

            let mut ast = Ast::default();
            $( ast.statements.push($stmt); )*

            ast
        }
    }
}

#[macro_export]
macro_rules! nop {
    () => {
        $crate::ast::statement::Statement::NOP
    };
}

#[macro_export]
macro_rules! eof {
    () => {
        $crate::ast::statement::Statement::EOF
    };
}

#[macro_export]
macro_rules! empty_block {
    () => {
        block_stmt! { nop!() }
    };
}

#[macro_export]
macro_rules! proc_call_expr {
    (name: $proc_name:expr, params: [$( $param:expr ),*]) => {
        {
            use $crate::ast::expression::Expression;

            let mut params = Vec::<Expression>::new();
            $( params.push($param); )*

            Expression::ProcCall($proc_name.to_string(), params)
        }
    };
}

#[macro_export]
macro_rules! proc_param {
    ($pname:expr, $ptype:expr) => {{
        use $crate::ast::statement::ProcParam;

        ProcParam {
            param_name: $pname.to_string(),
            param_type: $ptype.to_string(),
        }
    }};
}
