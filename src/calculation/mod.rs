pub mod dice_rolls;
pub mod math_ops;

#[derive(Debug, Clone)]
pub struct RollRequest<R: rand::Rng> {
    pub rng: R,
    pub result: Option<Vec<u64>>,
}

#[derive(Debug, Clone)]
struct TwoScalarCalculation {
    operation: Operation,
    first: RollScalar,
    second: RollScalar,
}

#[derive(Debug, Clone)]
enum RollScalar {
    Number(f64),
    Roll(Option<Vec<u64>>),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Div,
    Sub,
}
