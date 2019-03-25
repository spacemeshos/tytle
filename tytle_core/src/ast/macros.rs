#[macro_export]
macro_rules! direction {
    ($d:ident) => {{
        $crate::ast::statement::Direction::from(stringify!($d).to_uppercase().as_str())
    }};
}

#[macro_export]
macro_rules! int_lit_expr {
    ($num:expr) => {{
        int_lit_expr!($num, parens: false)
    }};

    ($num:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst, LiteralExpr};

        let ast = ExpressionAst::Literal(LiteralExpr::Int($num));
        Expression::adjust_parentheses(ast, $parens)
    }};
}

#[macro_export]
macro_rules! str_lit_expr {
    ($s:expr) => {{
        str_lit_expr!($s, parens: false)
    }};

    ($s:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst, LiteralExpr};

        let ast = ExpressionAst::Literal(LiteralExpr::Str($s.to_string()));
        Expression::adjust_parentheses(ast, $parens)
    }};
}

#[macro_export]
macro_rules! bool_lit_expr {
    ($bool:expr) => {{
        bool_lit_expr!($bool, parens: false)
    }};

    ($bool:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst, LiteralExpr};

        let ast = ExpressionAst::Literal(LiteralExpr::Bool($bool));
        Expression::adjust_parentheses(ast, $parens)
    }};
}

#[macro_export]
macro_rules! var_lit_expr {
    ($s:expr) => {{
        var_lit_expr!($s, parens: false)
    }};

    ($s:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst, LiteralExpr};

        let ast = ExpressionAst::Literal(LiteralExpr::Var($s.to_string(), None));
        Expression::adjust_parentheses(ast, $parens)
    }};
}

#[macro_export]
macro_rules! boxed_int_lit_expr {
    ($num:expr) => {{
        boxed_int_lit_expr!($num, parens: false)
    }};

    ($num:expr, parens: $parens:expr) => {{
        Box::new(int_lit_expr!($num, parens: $parens))
    }};
}

#[macro_export]
macro_rules! boxed_bool_lit_expr {
    ($bool:expr) => {{
        boxed_bool_lit_expr!($bool, parens: false)
    }};

    ($bool:expr, parens: $parens:expr) => {{
        Box::new(bool_lit_expr!($bool, parens: $parens))
    }};
}

#[macro_export]
macro_rules! boxed_var_lit_expr {
    ($s:expr) => {{
        boxed_var_lit_expr!($s, parens: false)
    }};

    ($s:expr, parens: $parens:expr) => {{
        Box::new(var_lit_expr!($s, parens: $parens))
    }};
}

#[macro_export]
macro_rules! boxed_expr {
    ($expr:expr) => {{
        Box::new($expr)
    }};
}

#[macro_export]
macro_rules! direct_lit_expr {
    ($dir:ident, $count:expr) => {{
        direct_lit_expr!($dir, $count, parens: false)
    }};

    ($dir:ident, $count:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst, LiteralExpr};

        let ast = ExpressionAst::Literal(LiteralExpr::Int($count));
        let expr = Expression::adjust_parentheses(ast, $parens);

        Statement::Direction($crate::ast::statement::DirectionStmt {
            direction: direction!($dir),
            expr: expr,
        })
    }};
}

#[macro_export]
macro_rules! expr_stmt {
    ($expr:expr) => {{
        use $crate::ast::statement::Statement;

        Statement::Expression($expr)
    }};
}

#[macro_export]
macro_rules! direct_stmt {
    ($dir:ident, $expr:expr) => {{
        use $crate::ast::statement::DirectionStmt;

        Statement::Direction(DirectionStmt {
            direction: direction!($dir),
            expr: $expr,
        })
    }};
}

#[macro_export]
macro_rules! command_stmt {
    ($cmd:ident) => {{
        use $crate::ast::statement::{Command, Statement};

        let cmd_enum = Command::parse(stringify!($cmd)).unwrap();
        Statement::Command(cmd_enum)
    }};
}

#[macro_export]
macro_rules! __make_stmt {
    ($kind:expr, $var_name:expr, $expr:expr) => {{
        use $crate::ast::statement::{MakeStmt, MakeStmtKind, Statement};

        Statement::Make(MakeStmt {
            var_id: None,
            kind: $kind,
            var_name: $var_name.to_string(),
            expr: $expr,
        })
    }};
}

#[macro_export]
macro_rules! make_global_stmt {
    ($var_name:expr, $expr:expr) => {{
        use $crate::ast::statement::MakeStmtKind;
        __make_stmt!(MakeStmtKind::Global, $var_name.to_string(), $expr)
    }};
}

#[macro_export]
macro_rules! make_local_stmt {
    ($var_name:expr, $expr:expr) => {{
        use $crate::ast::statement::MakeStmtKind;
        __make_stmt!(MakeStmtKind::Local, $var_name.to_string(), $expr)
    }};
}

#[macro_export]
macro_rules! make_stmt {
    ($var_name:expr, $expr:expr) => {{
        use $crate::ast::statement::MakeStmtKind;
        __make_stmt!(MakeStmtKind::Assign, $var_name.to_string(), $expr)
    }};
}

#[macro_export]
macro_rules! print_stmt {
    ($expr:expr) => {{
        use $crate::ast::statement::Statement;
        Statement::Print($expr)
    }};
}

#[macro_export]
macro_rules! with_parentheses {
    ($expr:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst};

        let ast = ExpressionAst::Parentheses(Box::new($expr));
        Expression::new(ast)
    }};
}

#[macro_export]
macro_rules! not_expr {
    ($expr:expr) => {{
        not_expr!($expr, parens: false)
    }};

    ($expr:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst};

        let ast = ExpressionAst::Not(Box::new($expr));
        Expression::adjust_parentheses(ast, $parens)
    }};
}

#[macro_export]
macro_rules! binary_expr {
    ($op_str:expr, $lexpr:expr, $rexpr:expr) => {{
        binary_expr!($op_str, $lexpr, $rexpr, parens: false)
    }};

    ($op_str:expr, $lexpr:expr, $rexpr:expr, parens: $parens:expr) => {{
        use $crate::ast::expression::{Expression, ExpressionAst};

        let op = BinaryOp::from($op_str);
        let ast = ExpressionAst::Binary(op, $lexpr, $rexpr);
        Expression::adjust_parentheses(ast, $parens)
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

        let return_type = stringify!($ret_type).to_string();

        let proc_stmt = Statement::Procedure(ProcedureStmt {
            id: None,
            params,
            name: $proc_name.to_string(),
            return_type,
            block: block_stmt,
        });

        proc_stmt
    }};
}

#[macro_export]
macro_rules! halt_stmt {
    () => {{
        let ret_stmt = ReturnStmt::new(None);

        Statement::Return(ret_stmt)
    }};
}

#[macro_export]
macro_rules! ret_stmt {
    ($expr:expr) => {{
        use $crate::ast::statement::{ReturnStmt, Statement};

        let ret_stmt = ReturnStmt::new(Some($expr));

        Statement::Return(ret_stmt)
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
            use $crate::ast::expression::{Expression, ExpressionAst};

            let mut params = Vec::<Expression>::new();
            $( params.push($param); )*

            let ast = ExpressionAst::ProcCall($proc_name.to_string(), params, None);
            Expression::new(ast)
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
