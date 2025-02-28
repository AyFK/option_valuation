
use gnuplot::*;
use crate::datastructs::min_max_list::MinMaxList;

use super::plot_tools::histogram::histogram_plot;

#[allow(dead_code)]
//pub fn figure(sample: &[f64]) {
pub fn figure(sample: &MinMaxList) {

    let list = &sample.list;


    let sample: Vec<f64> = list.iter()
                           .map(|opt| opt.get().unwrap_or(0.0))
                           .collect();




    let mut fg = Figure::new();
    let ax = fg.axes2d();


    histogram_plot(ax, &sample);

    ax.set_x_label("Value", &[]);
    ax.set_y_label("Frequency", &[]);

    ax.set_x_grid(true);
    ax.set_y_grid(true);

    fg.set_terminal("wxt", "");

    // spawn plot on new thread to let Rust code continue
    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}
