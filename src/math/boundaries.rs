/// `FunctionBoundary` is an enum that represents a given function boundary in
/// the scope of the real numbers.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum FunctionBoundary {
    RealNumber(f64),
    NegativeInfinity,
    PositiveInfinity
}

/// `FunctionBoundaries` is a struct that contains both a lower boundary and an upper boundary which
/// are members of the real numbers. This struct is used to store the domain or range of a function in the
/// real numbers.
pub struct FunctionBoundaries {
    lower_bound: FunctionBoundary,
    upper_bound: FunctionBoundary
}

impl FunctionBoundaries {
    /// validates the `lower_bound` and `upper_bound` passed in and returns a `FunctionBoundaries` instance
    /// # Examples
    /// ```
    /// use bionic::math::boundaries::*;
    /// 
    /// let gaussian_function_range = FunctionBoundaries::new(FunctionBoundary::NegativeInfinity, FunctionBoundary::PositiveInfinity);
    /// assert_eq!(gaussian_function_range.get_lower(), FunctionBoundary::NegativeInfinity);
    /// assert_eq!(gaussian_function_range.get_upper(), FunctionBoundary::PositiveInfinity);
    /// ```
    /// # Panics
    /// * When `lower_bound` is `FunctionBoundary::PositiveInfinity`
    /// * When `upper_bound` is `FunctionBoundary::NegativeInfinity`
    /// * When `lower_bound` is greater than `upper_bound`
    pub fn new(lower_bound: FunctionBoundary, upper_bound: FunctionBoundary) -> FunctionBoundaries {
        if lower_bound == FunctionBoundary::PositiveInfinity {
            panic!("The lower bound cannot be positive infinity");
        }

        if upper_bound == FunctionBoundary::NegativeInfinity {
            panic!("The upper bound cannot be negative infinity");
        }

        if let FunctionBoundary::RealNumber(l) = lower_bound {
            if let FunctionBoundary::RealNumber(u) = upper_bound {
                if l > u {
                    panic!("the lower bound must be less than or equal to the upper bound");
                }
            }
        }

        FunctionBoundaries { lower_bound, upper_bound }
    }
    /// get the lower boundary
    pub fn get_lower(&self) -> FunctionBoundary {
        self.lower_bound
    }
    /// get the upper boundary
    pub fn get_upper(&self) -> FunctionBoundary {
        self.upper_bound
    }
}