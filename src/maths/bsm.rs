#![allow(non_snake_case)] // for math notation purposes
#![allow(dead_code)]      // i know when this code is used and not used

use super::stats::{normal_pdf, normal_cdf};


fn d_1(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return ((S_0 / K).ln() + (r + 0.5 * sigma * sigma) * tau) /
           (sigma * tau.sqrt());
}


fn d_2(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return ((S_0 / K).ln() + (r - 0.5 * sigma * sigma) * tau) /
           (sigma * tau.sqrt());
}


pub fn black_scholes_call_price(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    let Phi_1 = normal_cdf(d_1(S_0, K, sigma, r, tau), 0.0, 1.0);
    let Phi_2 = normal_cdf(d_2(S_0, K, sigma, r, tau), 0.0, 1.0);

    return S_0 * Phi_1 - K * (-r * tau).exp() * Phi_2;
}


pub fn black_scholes_put_price(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    let Phi_1 = normal_cdf(-d_1(S_0, K, sigma, r, tau), 0.0, 1.0);
    let Phi_2 = normal_cdf(-d_2(S_0, K, sigma, r, tau), 0.0, 1.0);

    return K * (-r * tau).exp() * Phi_2 - S_0 * Phi_1;
}


pub fn black_scholes_call_delta(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return normal_cdf(d_1(S_0, K, sigma, r, tau), 0.0, 1.0);
}


pub fn black_scholes_put_delta(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return normal_cdf(d_1(S_0, K, sigma, r, tau), 0.0, 1.0) - 1.0;
}


fn black_scholes_gamma(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return normal_pdf(d_1(S_0, K, sigma, r, tau), 0.0, 1.0) /
           (S_0 * sigma * tau.sqrt());
}


pub fn black_scholes_call_gamma(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return black_scholes_gamma(S_0, K, sigma, r, tau);
}


pub fn black_scholes_put_gamma(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    return black_scholes_gamma(S_0, K, sigma, r, tau);
}


pub fn black_scholes_call_theta(S_0: f64, K: f64, sigma: f64, r: f64, tau: f64) -> f64 {
    let phi_1 = normal_pdf(d_1(S_0, K, sigma, r, tau), 0.0, 1.0);
    let Phi_1 = normal_cdf(-d_1(S_0, K, sigma, r, tau), 0.0, 1.0);
    let Phi_2 = normal_cdf(-d_2(S_0, K, sigma, r, tau), 0.0, 1.0);

    return S_0 * phi_1 * sigma / (2.0 * tau.sqrt()) - r * K * (-r *
           tau).exp() * Phi_2 + S_0 * Phi_1;
}
