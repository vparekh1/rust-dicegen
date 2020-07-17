pub mod calculator;

#[derive(Debug, Clone)]
pub struct TwoScalarCalculation {
    operation: Operation,
    first: RollScalar,
    second: RollScalar,
}

#[derive(Debug, Clone)]
pub enum RollScalar {
    Number(f64),
    Roll(Option<Vec<u64>>),
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Mul,
    Div,
    Sub,
}
