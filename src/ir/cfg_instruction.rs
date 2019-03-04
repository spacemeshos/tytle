#[derive(Debug, Clone, PartialEq)]
pub enum CfgInstruction {
    // Command(CommandInst),
    // Direction(DirectionInst),
    // Assign(AssignInst),
    // Call(CallInst),
    // If(IfInst),
    // Loop(LoopInst),
}


#[derive(Debug, Clone, PartialEq)]
pub struct CommandInst {
    pub cmd: CommandStmt
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionInst {
    pub direct: Direction
    pub expr: ExprInst
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignInst {
    pub var: VarRef,
    pub expr: Expression,
}
