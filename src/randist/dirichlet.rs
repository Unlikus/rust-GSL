//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use ffi;
use types::Rng;

/// This function returns an array of K random variates from a Dirichlet distribution of order K-1. The distribution function is
/// 
/// p(\theta_1, ..., \theta_K) d\theta_1 ... d\theta_K = 
/// 
///   (1/Z) \prod_{i=1}^K \theta_i^{\alpha_i - 1} \delta(1 -\sum_{i=1}^K \theta_i) d\theta_1 ... d\theta_K
/// 
/// for theta_i >= 0 and alpha_i > 0. The delta function ensures that \sum \theta_i = 1. The normalization factor Z is
/// 
/// Z = {\prod_{i=1}^K \Gamma(\alpha_i)} / {\Gamma( \sum_{i=1}^K \alpha_i)}
/// 
/// The random variates are generated by sampling K values from gamma distributions with parameters a=alpha_i, b=1, and renormalizing. See A.M. Law, W.D. Kelton, Simulation Modeling and Analysis (1991).
pub fn dirichlet(r: &Rng, alpha: &[f64], theta: &mut [f64]) {
    unsafe { ffi::gsl_ran_dirichlet(ffi::FFI::unwrap(r) as *const ffi::gsl_rng, alpha.len() as u64, alpha.as_ptr(), theta.as_mut_ptr()) }
}

/// This function computes the probability density p(\theta_1, ... , \theta_K) at theta[K] for a Dirichlet distribution with parameters alpha[K], using the formula given above.
pub fn dirichlet_pdf(alpha: &[f64], theta: &[f64]) -> f64 {
    unsafe { ffi::gsl_ran_dirichlet_pdf(alpha.len() as u64, alpha.as_ptr(), theta.as_ptr()) }
}

/// This function computes the logarithm of the probability density p(\theta_1, ... , \theta_K) for a Dirichlet distribution with parameters alpha[K].
pub fn dirichlet_lnpdf(alpha: &[f64], theta: &[f64]) -> f64 {
    unsafe { ffi::gsl_ran_dirichlet_lnpdf(alpha.len() as u64, alpha.as_ptr(), theta.as_ptr()) }
}