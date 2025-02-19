use gnuplot::*;

pub fn plot(ax: &mut Axes2D, y: &[f64]) {

    // create uniform time parition 'x'
    let n: usize = std::cmp::max(y.len(),
                                 y.len());

    let x: &Vec<f64> = &(0..n).map(|v| v as f64).collect();

    // draw line on axes
    ax.lines(x, y, &[]);
}
