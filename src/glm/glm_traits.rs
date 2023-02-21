use nalgebra::base::DVector;
use crate::math::boundaries::FunctionBoundary;

pub trait LinkFunction {
    fn get_inverse_output(&self, model_parameters: DVector<f64>, input_features: DVector<f64>) -> f64;
    fn get_domain(&self) -> (FunctionBoundary, FunctionBoundary);
}

pub trait ProbabilityDistributionFunction {
    fn get_range(&self) -> (FunctionBoundary, FunctionBoundary);
    fn is_compatible_with_link_function(&self, link_function: Box<dyn LinkFunction>) -> bool;
}

pub trait GeneralizedLinearModel<L: LinkFunction, PDF: ProbabilityDistributionFunction> {

}