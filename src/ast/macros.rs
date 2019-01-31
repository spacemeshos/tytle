#[macro_export]
macro_rules! direction {
    ($d:ident) => {
        $crate::ast::statement::Direction::from(stringify!($d).to_uppercase().as_str())
    };
}

#[macro_export]
macro_rules! direct_lit_expr {
    ($dir:ident, $count:expr) => {
        Statement::Direction($crate::ast::statement::DirectionStmt {
            direction: direction!($dir),
            expr: $crate::ast::expression::Expression::Literal($crate::ast::expression::LiteralExpr::Int($count))
        })
    }
}


