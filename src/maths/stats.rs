use std::f64::consts::{PI, SQRT_2};
use libm::erfc;

/// Computes arithmetic sample mean.
#[allow(dead_code)]
pub fn arithmetic_mean(time_series: &[f64]) -> f64 {
    let mut sum = 0.0;
    let n = time_series.len();

    for i in 0..n {
        sum += time_series[i];
    }

    return sum / (n as f64);
}


/// Computes sample standard deviation.
#[allow(dead_code)]
pub fn standard_deviation(time_series: &[f64]) -> f64 {
    let mut sum_sq = 0.0;
    let n = time_series.len();
    let mean = arithmetic_mean(time_series);

    for i in 0..n {
        let diff = time_series[i] - mean;
        sum_sq += diff * diff;
    }

    return (sum_sq / ((n - 1) as f64)).sqrt();
}



/// Computes the probability density function (PDF) of a
/// normal distribution.
#[allow(dead_code)]
pub fn normal_pdf(x: f64, mean: f64, stddev: f64) -> f64 {
    (1.0 / (stddev * (2.0 * PI).sqrt())) * (-0.5 * ((x - mean) / stddev).powi(2)).exp()
}

/// Computes the cumulative distribution function (CDF) of
/// the standard normal distribution.
#[allow(dead_code)]
fn standard_normal_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / SQRT_2)
}

/// Computes the cumulative distribution function (CDF) of a
/// normal distribution.
#[allow(dead_code)]
pub fn normal_cdf(x: f64, mean: f64, stddev: f64) -> f64 {
    return standard_normal_cdf(x) * stddev + mean;
}

/// Computes the inverse of the standard normal CDF using
/// Peter John Acklam's method.
#[allow(dead_code)]
fn acklam_method(p: f64) -> f64 {
    // coefficients in rational approximations
    let a = [-39.696830, 220.946098, -275.928510, 138.357751,
             -30.664798, 2.506628];

    let b = [-54.476098, 161.585836, -155.698979, 66.801311,
             -13.280681];

    let c = [-0.007784894002, -0.32239645, -2.400758, -2.549732,
              4.374664, 2.938163];

    let d = [0.007784695709, 0.32246712, 2.445134, 3.754408];

    // break-points
    let plow = 0.02425;
    let phigh = 1.0 - plow;

    // rational approximation for lower region:
    if p < plow {
        let q = (-2.0 * p.ln()).sqrt();
        return (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q +
               c[4]) * q + c[5]) / ((((d[0] * q + d[1]) * q +
               d[2]) * q + d[3]) * q + 1.0);
    }

    // rational approximation for upper region:
    if p > phigh {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        return -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q +
               c[4]) * q + c[5]) / ((((d[0] * q + d[1]) * q +
               d[2]) * q + d[3]) * q + 1.0);
    }

    // rational approximation for central region:
    let q = p - 0.5;
    let r = q * q;
    return (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) *
           r + a[5]) * q / (((((b[0] * r + b[1]) * r + b[2]) * r +
           b[3]) * r + b[4]) * r + 1.0);
}

/// Computes the inverse of the normal cumulative
/// distribution function.
#[allow(dead_code)]
pub fn normal_inv_cdf(probability: f64, mean: f64, stddev: f64) -> f64 {
    acklam_method(probability) * stddev + mean
}
