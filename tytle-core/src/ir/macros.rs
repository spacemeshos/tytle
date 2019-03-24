#[macro_export]
macro_rules! int_ins {
    ($num:expr) => {{
        use $crate::ir::CfgInstruction;
        CfgInstruction::Int($num)
    }};
}

#[macro_export]
macro_rules! bool_ins {
    ($bool:expr) => {{
        use $crate::ir::CfgInstruction;
        CfgInstruction::Bool($bool)
    }};
}

#[macro_export]
macro_rules! str_ins {
    ($str:expr) => {{
        use $crate::ir::CfgInstruction;
        CfgInstruction::Str($str.to_string())
    }};
}

#[macro_export]
macro_rules! add_ins {
    () => {{
        $crate::ir::CfgInstruction::Add
    }};
}

#[macro_export]
macro_rules! mul_ins {
    () => {{
        $crate::ir::CfgInstruction::Mul
    }};
}

#[macro_export]
macro_rules! not_ins {
    () => {{
        $crate::ir::CfgInstruction::Not
    }};
}

#[macro_export]
macro_rules! and_ins {
    () => {{
        $crate::ir::CfgInstruction::And
    }};
}

#[macro_export]
macro_rules! or_ins {
    () => {{
        $crate::ir::CfgInstruction::Or
    }};
}

#[macro_export]
macro_rules! gt_ins {
    () => {{
        $crate::ir::CfgInstruction::GT
    }};
}

#[macro_export]
macro_rules! lt_ins {
    () => {{
        $crate::ir::CfgInstruction::LT
    }};
}

#[macro_export]
macro_rules! store_ins {
    ($symbol_id:expr) => {{
        $crate::ir::CfgInstruction::Store($symbol_id)
    }};
}

#[macro_export]
macro_rules! load_ins {
    ($symbol_id:expr) => {{
        $crate::ir::CfgInstruction::Load($symbol_id)
    }};
}

#[macro_export]
macro_rules! cmd_ins {
    ($cmd:ident) => {{
        use $crate::ast::statement::Command;
        use $crate::ir::CfgInstruction;

        let cmd = Command::parse(stringify!($cmd)).unwrap();
        CfgInstruction::Command(cmd)
    }};
}

#[macro_export]
macro_rules! direct_ins {
    ($direct:ident) => {{
        use $crate::ast::statement::Direction;
        use $crate::ir::CfgInstruction;

        let direct = Direction::from(stringify!($direct));
        CfgInstruction::Direction(direct)
    }};
}

#[macro_export]
macro_rules! call_ins {
    ($node_id:expr) => {{
        use $crate::ir::CfgInstruction;

        CfgInstruction::Call($node_id)
    }};
}

#[macro_export]
macro_rules! ret_ins {
    () => {{
        $crate::ir::CfgInstruction::Return
    }};
}

#[macro_export]
macro_rules! trap_ins {
    () => {{
        $crate::ir::CfgInstruction::Trap
    }};
}

#[macro_export]
macro_rules! eoc_ins {
    () => {{
        $crate::ir::CfgInstruction::EOC
    }};
}

#[macro_export]
macro_rules! print_ins {
    () => {{
        $crate::ir::CfgInstruction::Print
    }};
}

#[macro_export]
macro_rules! node {
    ($node_id:expr) => {{
        use $crate::ir::CfgNode;
        let mut node = CfgNode::new($node_id);

        CfgElement::Node(node)
    }};

    ($node_id:expr, $ ($ins:expr) ,*) => {
        {
            use $crate::ir::CfgNode;

            let mut node = CfgNode::new($node_id);
            $( node.append_inst($ins); )*

            CfgElement::Node(node)
        }
    }
}

#[macro_export]
macro_rules! edge_true_jmp {
    ($src_id:expr, $dst_id:expr) => {{
        use $crate::ir::{CfgElement, CfgJumpType};

        CfgElement::Edge($src_id, $dst_id, CfgJumpType::WhenTrue)
    }};
}

#[macro_export]
macro_rules! edge_fallback_jmp {
    ($src_id:expr, $dst_id:expr) => {{
        use $crate::ir::{CfgElement, CfgJumpType};

        CfgElement::Edge($src_id, $dst_id, CfgJumpType::Fallback)
    }};
}

#[macro_export]
macro_rules! edge_always_jmp {
    ($src_id:expr, $dst_id:expr) => {{
        use $crate::ir::{CfgElement, CfgJumpType};

        CfgElement::Edge($src_id, $dst_id, CfgJumpType::Always)
    }};
}

#[macro_export]
macro_rules! cfg_graph {
    ($ ($elem:expr) ,*) => {
        {
            use $crate::ir::CfgGraph;

            let mut graph = CfgGraph::new();
            $( graph.add_element($elem); )*

            graph
        }
    }
}

#[macro_export]
macro_rules! compile_cfg_obj {
    ($code: expr) => {{
        let mut ast = TytleParser.parse($code).unwrap();
        let generator = SymbolTableGenerator::new();

        let mut env = generator.generate(&mut ast).unwrap();
        let mut checker = AstTypeCheck::new(&mut env);

        let res = checker.check(&mut ast);
        assert!(res.is_ok());

        let builder = CfgBuilder::new(&mut env);
        let cfg_obj = builder.build(&ast);

        cfg_obj
    }};
}

#[macro_export]
macro_rules! compile_cfg_graph {
    ($code: expr) => {{
        let cfg_obj = compile_cfg_obj!($code);

        cfg_obj.graph
    }};
}
