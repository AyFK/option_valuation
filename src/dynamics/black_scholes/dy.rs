use std::collections::HashMap;

use rand::prelude::thread_rng;
use rand_distr::{Distribution, Normal};

/// Use the stochastic differential equation of a Geometric
/// Brownain motion to generate increment of log-price: dY,
/// using 'process_params' from a `HashMap`.
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn invoke(process_params: &HashMap<String, f64>) -> f64 {

    // get needed param values
    let sigma = process_params["sigma"];
    let mu = process_params["mu"];

    // initialize random number generator and distribution
    let stdnormal = Normal::new(0.0, 1.0).unwrap();
    let dW = stdnormal.sample(&mut thread_rng());

    // calculate SDE drift and diffusion
    let diffusion = sigma * dW;
    let drift = mu - 0.5 * sigma * sigma;
    let SDE = drift + diffusion;

    // calculate the change in price (log return SDE for GBM)
    return SDE;
}
