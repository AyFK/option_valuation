
use gnuplot::*;

use rand::prelude::thread_rng;
use rand_distr::{Distribution, Normal};

use crate::maths::stats::normal_inv_cdf;


/// Make a normal distribution Q-Q plot of both real, and
/// random sample data.
pub fn normal_distribution_qq(ax: &mut Axes2D, x: &[f64], mean: f64, stddev: f64) {
    let col1 = "#0492C2";         // default color blue
    let cap1 = "Real sample";     // default caption empty
    let col2 = "#FF8243";         // default color orange
    let cap2 = "Random sample";   // default caption empty


    let n = x.len();

    if n == 0 {
        return;
    }

    // uniform partition in (0,1), excluding the endpoints
    let start = 1.0 / (n as f64);
    let end = 1.0 - 1.0 / (n as f64);
    let quantile_step = (end - start) / ((n - 1) as f64);

    let mut quantile_vals = Vec::with_capacity(n);
    let mut random_sample = Vec::with_capacity(n);

    let stdnormal = Normal::new(0.0, 1.0).unwrap();

    // compute the theoretical quantiles and generate a random sample
    for i in 0..n {
        let p = start + (i as f64) * quantile_step;

        // random sample
        let standard_sample: f64 = stdnormal.sample(&mut thread_rng());
        random_sample.push(standard_sample * stddev + mean);

        // real sample
        quantile_vals.push(normal_inv_cdf(p, mean, stddev));
    }

    // sort the provided sample data and the random sample
    let mut sorted_x = x.to_vec();
    sorted_x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut sorted_samples = random_sample.clone();
    sorted_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());


    let x_extreme = vec![quantile_vals[0], quantile_vals[n-1]];
    let y_extreme = vec![sorted_x[0], sorted_x[n-1]];
    let y_extreme_2 = vec![sorted_samples[0], sorted_samples[n-1]];


    // plot the QQ-plot for real sample
    let line1_options: [PlotOption<&str>; 4] = [Color(&col1), LineStyle(
                                                DashType::Dash), LineWidth(1.5),
                                                Caption(&cap1)];

    let scatter1_options: [PlotOption<&str>; 3] = [Color(&col1),
                                                   PointSymbol('o'),
                                                   PointSize(0.7)];

    ax.lines(&x_extreme, &y_extreme, &line1_options);
    ax.points(&quantile_vals, &sorted_x, &scatter1_options);


    // plot the QQ-plot for random sample
    let line2_options: [PlotOption<&str>; 4] = [Color(&col2), LineStyle(
                                                DashType::Dash), LineWidth(1.5),
                                                Caption(&cap2)];

    let scatter2_options: [PlotOption<&str>; 3] = [Color(&col2),
                                                   PointSymbol('o'),
                                                   PointSize(0.7)];

    ax.points(&quantile_vals, &sorted_samples, &scatter2_options);
    ax.lines(&x_extreme, &y_extreme_2, &line2_options);
}
