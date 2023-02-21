#[derive(PartialEq)]
pub enum FunctionBoundary {
    RealNumber(f64),
    NegativeInfinity,
    PositiveInfinity
}

struct FunctionBoundaries {
    lower_bound: FunctionBoundary,
    upper_bound: FunctionBoundary
}