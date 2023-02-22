//! glm_traits
//! 
//! Contains different traits that describe abstract concepts within GLM theory.
//! Go [here](http://web.vu.lt/mif/a.buteikis/wp-content/uploads/PE_Book/5-1-GLM.html) for an overview of the theory.

use nalgebra::base::DVector;
use crate::math::boundaries::FunctionBoundaries;

/// In GLM theory, a link function is a function that maps the response variable of a linear predictor to the mean of
/// a probability distribution function. This allows for data that is non-linear such as exponential or categorical data to still be
/// adequately modeled by a GLM.
pub trait LinkFunction {
    fn get_inverse_output(&self, model_parameters: DVector<f64>, input_features: DVector<f64>) -> f64;
    fn get_domain(&self) -> FunctionBoundaries;
}
/// A probability distribution function describes the distribution of the observed data. Some data follows a Gaussian
/// distribution, for example, if the data points are continous or real-valued such as distance or time.
pub trait ProbabilityDistributionFunction {
    fn get_range(&self) -> FunctionBoundaries;
    fn is_compatible_with_link_function(&self, link_function: Box<dyn LinkFunction>) -> bool;
}

pub trait GeneralizedLinearModel<L: LinkFunction, PDF: ProbabilityDistributionFunction> {

}