
use gnuplot::*;
use super::plot_tools::histogram::histogram_plot;

#[allow(dead_code)]
pub fn figure(sample: &[f64]) {

    let mut fg = Figure::new();
    let ax = fg.axes2d();


    histogram_plot(ax, sample);

    ax.set_x_label("Value", &[]);
    ax.set_y_label("Frequency", &[]);

    fg.set_terminal("wxt", "");
    fg.show().unwrap();
}
