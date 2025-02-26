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
pub fn params(data: &[f64]) -> (Vec<f64>, Vec<usize>) {

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


pub fn histogram_plot(ax: &mut Axes2D, sample: &[f64]) {

    // get bin edges and frequency
    let (bin_edges, frequencies) = params(&sample);

    // options
    let options: [PlotOption<&str>; 3] = [Caption(""), Color("#66AAD7"),
                                          BorderColor("#000000")];

    // plot histogram
    ax.boxes(&bin_edges, &frequencies, &options);
}
