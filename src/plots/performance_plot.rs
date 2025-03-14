use std::rc::Rc;
use std::cell::Cell;

use gnuplot::*;

use super::plot_tools::{stems::{self, *}, time_series::{self, *}, two_unit_plots::{self, *}};

/// Plot replicating portfolio performance
pub fn figure(portfolio_process: &Vec<Cell<f64>>, price_process: &Vec<Cell<f64>>,
              return_process: &Vec<Cell<f64>>) {

    //let sim_idx = 999;

    // convert Cell items into regular f64;s for processes of interest
    let portfolio_price: Vec<f64> = portfolio_process.iter()
                                          .map(|cell| cell.get())
                                          .collect();

    let price_process: Vec<f64> = price_process.iter()
                                       .map(|cell| cell.get())
                                       .collect();

    let return_process: Vec<f64> = return_process.iter()
                                        .map(|cell| cell.get())
                                        .collect();
    //return_process.insert(0, 0.0);


    // create uniform time parition 'x'
    let n: usize = std::cmp::max(price_process.len(),
                                 price_process.len());

    let x: Vec<f64> = (0..n).map(|v| v as f64).collect();

    // establish mutable gnuplot figure
    let mut fg = Figure::new();

    { // create the first axes within its own scope
        let ax1 = fg.axes2d(); // borrow fg
        let plot_col2 = "#0492C2";

        // first row, first column
        ax1.set_pos_grid(1, 2, 0);
        ax1.set_x_grid(true);
        ax1.set_y_grid(true);

        // plot stock price and replicating portfolio
        two_unit_plots::compare_ts_plot(ax1, &x, &price_process, &portfolio_price,
                                        None, None, Some(plot_col2), None);


        // plot strike-price line
        let line_option : [PlotOption<&str>; 4] = [Color("#000000"), LineStyle(
                                                   DashType::Dot), LineWidth(2.0),
                                                   Caption("Strike")];

        let strike = vec![6100.0; n];
        ax1.lines(&x, &strike, &line_option);

        ax1.set_y_label("S(t)", &[LabelOption::TextColor("#000000")]);
        ax1.set_y2_label("Π(t)", &[LabelOption::TextColor(plot_col2)]);
    } // ax1 is dropped, releasing fg

    { // create the second axes within its own scope
        let ax2 = fg.axes2d(); // borrow fg

        // first row, second column
        ax2.set_pos_grid(1, 2, 1);
        ax2.set_x_grid(true);
        ax2.set_y_grid(true);

        // make a stem plot
        stems::stem_plot(ax2, &x, &return_process, None, None);

        // set y-label
        ax2.set_y_label("% (bp)", &[LabelOption::TextColor("#000000")]);
    } // ax2 is dropped, releasing fg


    // set smoother gnuplot terminal and show it
    fg.set_terminal("wxt", "");

    // spawn plot on new thread to let Rust code continue
    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}
