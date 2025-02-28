use crate::maths::stats::{self};

/// Infer parameters: σ, used in the delta hedging
/// procedure.
pub fn invoke(log_returns: &[f64]) -> f64 {

    // calculate μ and σ
    let implied_sigma = stats::standard_deviation(&log_returns) / 100.0;

    return implied_sigma;
}
