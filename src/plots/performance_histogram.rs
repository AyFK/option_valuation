
use gnuplot::*;
use crate::datastructs::min_max_list::MinMaxList;

use super::plot_tools::{histogram::histogram_plot, qq_plot::normal_distribution_qq};
use crate::maths::stats::{arithmetic_mean, standard_deviation};

#[allow(dead_code)]
//pub fn figure(sample: &[f64]) {
pub fn figure(sample: &MinMaxList) {

    let list = &sample.list;


    let sample: Vec<f64> = list.iter()
                           .map(|opt| opt.get().unwrap_or(0.0))
                           .collect();




    let mut fg = Figure::new();

    { // create the first axes within its own scope
        let ax1 = fg.axes2d(); // borrow fg

        // first row, first column
        ax1.set_pos_grid(2, 1, 0);
        ax1.set_x_grid(true);
        ax1.set_y_grid(true);

        histogram_plot(ax1, &sample);
        ax1.set_x_label("Value", &[]);
        ax1.set_y_label("Frequency", &[]);

    } // ax1 is dropped, releasing fg


    { // create the second axes within its own scope
        let ax2 = fg.axes2d(); // borrow fg

        // second row, first column
        ax2.set_pos_grid(2, 1, 1);
        ax2.set_x_grid(true);
        ax2.set_y_grid(true);

        let mean = arithmetic_mean(&sample);
        let stddev = standard_deviation(&sample);
        normal_distribution_qq(ax2, &sample, mean, stddev);

        ax2.set_legend(Coordinate::Graph(0.1), Coordinate::Graph(0.95),
                       &[], &[]);

        ax2.set_x_label("Theoretical", &[]);
        ax2.set_y_label("Sample", &[]);
    } // ax2 is dropped, releasing fg



    fg.set_terminal("wxt", "");

    // spawn plot on new thread to let Rust code continue
    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}
