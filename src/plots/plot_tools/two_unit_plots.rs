use gnuplot::*;

fn get_min(elem1: f64, elem2: f64) -> f64 {
    if elem1 < elem2 { elem1 } else { elem2 }
}


fn get_max(elem1: f64, elem2: f64) -> f64 {
    if elem1 > elem2 { elem1 } else { elem2 }
}

/// Takes an `Axes2D` obeject and two time series: `&[f64]`,
/// `&[f64]` of two different units and plots them such that
/// both fit into the same axes.
#[allow(dead_code)]
pub fn re_scaled(ax: &mut Axes2D, y1: &[f64], y2: &[f64]) -> Vec<f64> {

    // create uniform time parition 'x'
    let n: usize = std::cmp::max(y1.len(), y2.len()); // must be of equal lenght... TODO: FIX

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
    let y2_scaled: Vec<f64> = y2.iter().map(|&v| (v - y2_min) *
                              scale_factor + y1_min).collect();

    // y1 axis adjustments on Axes2D
    ax.set_y_range(AutoOption::Fix(y1_min), AutoOption::Fix(y1_max));

    // y2 axis adjustments on Axes2D
    ax.set_y2_range(AutoOption::Fix(y2_min), AutoOption::Fix(y2_max));

    return y2_scaled;
}
