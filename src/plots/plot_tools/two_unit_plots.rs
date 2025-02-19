use gnuplot::*;

use super::custom_colors;
use super::stems::*;
use super::custom_colors::*;

fn get_min(elem1: f64, elem2: f64) -> f64 {
    if elem1 < elem2 { elem1 } else { elem2 }
}


fn get_max(elem1: f64, elem2: f64) -> f64 {
    if elem1 > elem2 { elem1 } else { elem2 }
}

/// Takes an `Axes2D` obeject and two time series: `&[f64]`,
/// `&[f64]` of two different units and plots them such that
/// both fit into the same axes.
fn re_scaled(ax: &mut Axes2D, y1: &[f64], y2: &[f64]) -> Vec<f64> {

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

/// Plots two different time series onto the same `Axes2D`
/// obeject when both time series are of entirely different
/// scales.
#[allow(dead_code)]
pub fn compare_ts_plot(ax: &mut Axes2D, x: &[f64], y1: &[f64], y2: &[f64],
                       color1: Option<&str>, caption1: Option<&str>,
                       color2: Option<&str>, caption2: Option<&str>) {

    // default plotting options
    let col1 = color1.unwrap_or("#000000");   // default color black
    let cap1 = caption1.unwrap_or("");        // default caption empty
    let col2 = color2.unwrap_or("#FF0000");   // default color black
    let cap2 = caption2.unwrap_or("");        // default caption empty


    let line1_options: [PlotOption<&str>; 4] = [Color(col1), LineStyle(
                                                DashType::Solid), LineWidth(2.0),
                                                Caption(cap1)];
    ax.lines(x, y1, &line1_options);

    ax.set_y_ticks(Some((AutoOption::Auto, 0)), &[],
                    &[LabelOption::TextColor(col1)]);


    let line2_options: [PlotOption<&str>; 4] = [Color(col2), LineStyle(
                                                DashType::Dash), LineWidth(2.0),
                                                Caption(cap2)];
    let y2_scaled = re_scaled(ax, y1, y2);
    ax.lines(x, y2_scaled, &line2_options);

    ax.set_y2_ticks(Some((AutoOption::Auto, 0)), &[],
                    &[LabelOption::TextColor(col2)]);
}


/// Plots two different stem plots onto the same `Axes2D`
/// obeject when both stem plots are of entirely different
/// scales.
#[allow(dead_code)]
pub fn compare_stem_plot(ax: &mut Axes2D, x: &[f64], y1: &[f64], y2: &[f64],
                       color1: Option<&str>, caption1: Option<&str>,
                       color2: Option<&str>, caption2: Option<&str>) {

    // default plotting options
    let col1 = color1.unwrap_or("#000000");   // default color black
    let cap1 = caption1.unwrap_or("");        // default caption empty
    let col2 = color2.unwrap_or("#FF0000");   // default color black
    let cap2 = caption2.unwrap_or("");        // default caption empty

    // get line segments for stem plot
    let (x_segments, y_segments) = stem_segments(&x, &y1);

    // put line segments into plot
    ax.lines(&x_segments, &y_segments, &[Color(col1)]);

    // define scatter plot options
    let scatter1_options: [PlotOption<&str>; 3] = [Color(col1),
                                                   PointSymbol('o'),
                                                   Caption(cap1)];

    // put scatter plot on top of lines to complete the stem plot
    ax.points(x, y1, &scatter1_options);

    // set color of ticks equal to the graph
    ax.set_y_ticks(Some((AutoOption::Auto, 0)), &[],
                   &[LabelOption::TextColor(col1)]);


    // get line segments for stem plot
    let y2_scaled = re_scaled(ax, y1, y2);
    let (x_segments, y_segments) = stem_segments(&x, &y2_scaled);

    // make 'col2' transparent
    let alpha_col2 = custom_colors::alpha_hex(col2, 0.75);

    // put line segments into plot
    ax.lines(&x_segments, &y_segments, &[Color(&alpha_col2)]);

    // define scatter plot options
    let scatter2_options: [PlotOption<&str>; 3] = [Color(&alpha_col2),
                                                   PointSymbol('o'),
                                                   Caption(cap2)];

    // put scatter plot on top of lines to complete the stem plot
    ax.points(x, y2_scaled, &scatter2_options);

    // set color of ticks equal to the graph
    ax.set_y2_ticks(Some((AutoOption::Auto, 0)), &[],
                    &[LabelOption::TextColor(col2)]);
}



/*
pub fn compare_ts_and_stem(ax: &mut Axes2D, x1: &[f64], y1: &[f64],
                           x2: &[f64], y2: &[f64],
                           color1: Option<&str>, caption1: Option<&str>,
                           color2: Option<&str>, caption2: Option<&str>) {

    // default plotting options
    let col1 = color1.unwrap_or("#000000");   // default color black
    let cap1 = caption1.unwrap_or("");        // default caption empty
    let col2 = color2.unwrap_or("#FF0000");   // default color black
    let cap2 = caption2.unwrap_or("");        // default caption empty

    // get line segments for stem plot
    let (x_segments, y_segments) = stem_segments(&x1, &y1);

    // put line segments into plot
    ax.lines(&x_segments, &y_segments, &[Color(col1)]);

    // define scatter plot options
    let scatter1_options: [PlotOption<&str>; 3] = [Color(col1),
                                                   PointSymbol('o'),
                                                   Caption(cap1)];

    // put scatter plot on top of lines to complete the stem plot
    ax.points(x1, y1, &scatter1_options);

    // set color of ticks equal to the graph
    ax.set_y_ticks(Some((AutoOption::Auto, 0)), &[],
                   &[LabelOption::TextColor(col1)]);


    let line2_options: [PlotOption<&str>; 4] = [Color(col2), LineStyle(
                                                DashType::Solid), LineWidth(2.0),
                                                Caption(cap2)];
    let y2_scaled = re_scaled(ax, y1, y2);
    ax.lines(x2, y2_scaled, &line2_options);

    ax.set_y2_ticks(Some((AutoOption::Auto, 0)), &[],
                    &[LabelOption::TextColor(col2)]);
}
*/
