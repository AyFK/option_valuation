use std::cell::Cell;

use gnuplot::*;

use crate::maths::stats::standard_deviation;


fn get_min(elem1: f64, elem2: f64) -> f64 {
    if elem1 < elem2 { elem1 } else { elem2 }
}


fn get_max(elem1: f64, elem2: f64) -> f64 {
    if elem1 > elem2 { elem1 } else { elem2 }
}


/// Given data, return 'bin_edges' and 'frequencies'. Suppose
/// to be the exact histogram generation procedure used in MATLAB.
#[allow(dead_code)]
fn params(data: &[f64]) -> (Vec<f64>, Vec<usize>) {

    // calculate bin width using scotts rule
    let n = data.len() as f64;
    let sigma = standard_deviation(&data);
    let raw_bin_width = 3.5 * sigma / n.powf(1.0 / 3.0);

    // adjust histogram bin widths to "nice" round number
    let pow_of_ten = 10.0_f64.powf(raw_bin_width.log10().floor());
    let rel_size = raw_bin_width / pow_of_ten;

    let bin_width =
    if rel_size < 1.5 {
        1.0 * pow_of_ten
    }
    else if rel_size < 2.5 {
        2.0 * pow_of_ten
    }
    else if rel_size < 4.0 {
        3.0 * pow_of_ten
    }
    else if rel_size < 7.5 {
        5.0 * pow_of_ten
    }
    else {
        10.0 * pow_of_ten
    };

    // calculate number of bins based on 'bin_width'
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for idx in 0..data.len() {
        min = get_min(data[idx], min);
        max = get_max(data[idx], max);
    }

    // calculate bin edges using 'bin_width' and 'bin_count'
    let left_edge = (min / bin_width).floor() * bin_width;
    let bin_count = ((max - left_edge) / bin_width).ceil() as usize;
    let right_edge = left_edge + (bin_count as f64) * bin_width;

    let mut bin_edges = Vec::with_capacity(bin_count + 1);
    for i in 0..=bin_count {
        bin_edges.push(left_edge + i as f64 * bin_width);
    }

    // count frequency of data in each bin
    let mut frequencies = vec![0; bin_count];
    for &x in data {
        if x >= left_edge && x <= right_edge {
            let index = ((x - left_edge) / bin_width).floor() as usize;
            if index < bin_count {
                frequencies[index] += 1;
            }
        }
    }

    (bin_edges, frequencies)
}


fn f(ax: &mut Axes2D, y1: &[f64], y2: &[f64]) { // compare plot
    // x-axis values
    let n: usize = std::cmp::max(y1.len(), y2.len());
    let x: Vec<f64> = (0..n).map(|v| v as f64 / n as f64).collect();

    // calculate min/max over arrays
    let mut y1_min = f64::INFINITY;
    let mut y1_max = f64::NEG_INFINITY;
    let mut y2_min = f64::INFINITY;
    let mut y2_max = f64::NEG_INFINITY;

    for idx in 0..n {
        y1_min = get_min(y1[idx], y1_min);
        y1_max = get_max(y1[idx], y1_max);
        y2_min = get_min(y2[idx], y2_min);
        y2_max = get_max(y2[idx], y2_max);
    }

    // calculate the scaling factor, y2 should occupy the same relative range
    // as y1
    let y1_range = y1_max - y1_min;
    let y2_range = y2_max - y2_min;
    let scale_factor = y1_range / y2_range;

    // scale the y2 data
    let y2_scaled: Vec<f64> = y2.iter().map(|&v| (v - y2_min) * scale_factor + y1_min).collect();

    // definer color for y1, y2
    let color1: &str = "blue";
    let color2: &str = "red";

    // set title
    ax.set_title("sine and cosine functions", &[]);

    // x axis name
    ax.set_x_label("x", &[]);

    // y1 axis adjustments
    ax.set_y_label("sine", &[LabelOption::TextColor(color1)]);
    ax.set_y_ticks(Some((AutoOption::Auto, 0)), &[], &[LabelOption::TextColor(color1)]);
    ax.set_y_range(AutoOption::Fix(y1_min), AutoOption::Fix(y1_max));

    // y2 axis adjustments
    ax.set_y2_label("cosine", &[LabelOption::TextColor(color2)]);
    ax.set_y2_ticks(Some((AutoOption::Auto, 0)), &[], &[LabelOption::TextColor(color2)]);
    ax.set_y2_range(AutoOption::Fix(y2_min), AutoOption::Fix(y2_max));

    ax.lines(&x, y1, &[Color(color1)]);
    ax.lines(&x, &y2_scaled, &[Color(color2)]);
}
