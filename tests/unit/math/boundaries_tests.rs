
mod function_boundaries_tests {

    use bionic::math::boundaries::{FunctionBoundary, FunctionBoundaries};

    #[test]
    #[should_panic]
    fn should_panic_when_lower_boundary_is_positive_infinity() {
        let func_bounds = FunctionBoundaries::new(
            FunctionBoundary::PositiveInfinity,
            FunctionBoundary::PositiveInfinity
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_when_upper_boundary_is_negative_infinity() {
        let func_bounds = FunctionBoundaries::new(
            FunctionBoundary::NegativeInfinity,
            FunctionBoundary::NegativeInfinity
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_when_lower_boundary_real_number_is_greater_than_upper_boundary_real_number() {
        let func_bounds = FunctionBoundaries::new(
            FunctionBoundary::RealNumber(1.0),
            FunctionBoundary::RealNumber(0.0)
        );
    }

    #[test]
    fn should_return_constructed_function_boundaries_when_lower_is_negative_inf_and_upper_is_positive_inf() {
        let func_bounds = FunctionBoundaries::new(
            FunctionBoundary::NegativeInfinity,
            FunctionBoundary::PositiveInfinity
        );
    }


}