
use gnuplot::*;

pub fn stem_segments(x: &[f64], y: &[f64]) -> (Vec<f64>, Vec<f64>) {
    // vectors to hold the line segments
    let mut x_segments = Vec::new();
    let mut y_segments = Vec::new();

    for (&x_i, &y_i) in x.iter().zip(y.iter()) {
        // start at the x-axis (0)
        x_segments.push(x_i);
        y_segments.push(0.0);

        // go to the data point (x_i, y_i)
        x_segments.push(x_i);
        y_segments.push(y_i);

        // insert a break to separate segments
        x_segments.push(f64::NAN);
        y_segments.push(f64::NAN);
    }


    return (x_segments, y_segments);
}


pub fn stem_plot(ax: &mut Axes2D, x: &[f64], y: &[f64],
                 color: Option<&str>, caption: Option<&str>) {

    // default plotting options
    let col = color.unwrap_or("#000000");   // default color black
    let cap = caption.unwrap_or("");        // default caption empty

    // get line segments for stem plot
    let (x_segments, y_segments) = stem_segments(&x, &y);

    // put line segements into plot
    ax.lines(&x_segments, &y_segments, &[Color(col)]);

    // define scatter plot options
    let scatter_options: [PlotOption<&str>; 3] = [Color(col), PointSymbol('o'),
                                                  Caption(cap)];

    // put scatter plot on top of lines to complete the stem plot
    ax.points(x, y, &scatter_options);

    // set color of ticks equal to the graph
    ax.set_y_ticks(Some((AutoOption::Auto, 0)), &[],
                   &[LabelOption::TextColor(col)]);
}
