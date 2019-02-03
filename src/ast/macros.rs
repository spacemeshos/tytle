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
macro_rules! var_lit_expr {
    ($s:expr) => {{
        use $crate::ast::expression::{Expression, LiteralExpr};
        Expression::Literal(LiteralExpr::Var($s.to_string()))
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
macro_rules! make_stmt {
    ($symbol:expr, $expr:expr) => {{
        use $crate::ast::statement::{MakeStmt, Statement};

        Statement::Make(MakeStmt {
            symbol: $symbol.to_string(),
            expr: $expr,
        })
    }};
}

#[macro_export]
macro_rules! binary_expr {
    ($op:expr, $lexpr:expr, $rexpr:expr) => {{
        use $crate::ast::expression::Expression;

        Expression::Binary($op, $lexpr, $rexpr)
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
    (name: $proc_name:expr, params: [$( $param:expr ),*], body: $block:expr) => {{
        use $crate::ast::statement::{Statement, ProcedureStmt};

        let mut params = vec![];
        $( params.push($param.to_string()); )*

        let block_stmt = $block;

        let proc_stmt = Statement::Procedure(ProcedureStmt {
            name: $proc_name.to_string(),
            params,
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
