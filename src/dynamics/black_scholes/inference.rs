
use crate::maths::stats::{self};

/// Infer parameters: μ, σ, used in the GBM price process
/// and return them as an `array`.
pub fn invoke(log_returns: &[f64]) -> [f64; 2] {

    // calculate μ and σ
    let mu = stats::arithmetic_mean(&log_returns) / 100.0;
    let sigma = stats::standard_deviation(&log_returns) / 100.0;

    return [mu, sigma];
}
