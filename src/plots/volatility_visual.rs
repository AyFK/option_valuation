
use std::rc::Rc;

use gnuplot::*;


use crate::market::asset::AssetProcess;
use crate::database::fetch_db;
use crate::plots::plot_tools::stems::stem_plot;

#[allow(dead_code)]
pub fn figure(asset: &Rc<AssetProcess>, sim_idx: usize) {

    // find simulation length
    let n_data = asset.price_processes[sim_idx].len();

    // grab 'n_data' worth of real data
    let fetch_db::CloseData {price, log_return} =
    fetch_db::ts_close(&asset.ticker, Some(n_data));

    let simulated_price: Vec<f64> = asset.price_processes[sim_idx].iter()
                                         .map(|cell| cell.get())
                                         .collect();

    let simulated_returns: Vec<f64> = asset.return_processes[sim_idx].iter()
                                           .map(|cell| cell.get())
                                           .collect();

    // define start and end of partition
    let t0 = 0;
    let ti = price.len();

    // make partition
    let x1: Vec<f64> = (t0..ti).map(|v| v as f64).collect();

    // establish mutable gnuplot figure
    let mut fg = Figure::new();

    { // create the first axes within its own scope
        let ax1 = fg.axes2d(); // borrow fg

        // first row, first column
        ax1.set_pos_grid(2, 1, 0);
        ax1.set_x_grid(true);
        ax1.set_y_grid(true);

        let tj = price.len() - 1;
        let tn = price.len() - 1 + simulated_price.len();
        let x2: Vec<f64> = (tj..tn).map(|v| v as f64).collect();

        ax1.lines(&x2, &simulated_price, &[Color("#2CA02C")]);
        ax1.lines(&x1, &price, &[Color("#000000")]);

        ax1.set_y_label("S(t)", &[LabelOption::TextColor("#000000")]);
    } // ax1 is dropped, releasing fg

    { // create the second axes within its own scope
        let ax2 = fg.axes2d(); // borrow fg

        // second row, first column
        ax2.set_pos_grid(2, 1, 1);
        ax2.set_x_grid(true);
        ax2.set_y_grid(true);

        let tj = price.len();
        let tn = price.len() - 1 + simulated_price.len();
        let x2: Vec<f64> = (tj..tn).map(|v| v as f64).collect();

        stem_plot(ax2, &x2, &simulated_returns, Some("#2CA02C"), None);
        stem_plot(ax2, &x1, &log_return, Some("#000000"), None);

        // set y-label
        ax2.set_y_label("% (bp)", &[LabelOption::TextColor("#000000")]);
    } // ax2 is dropped, releasing fg




    // set smoother gnuplot terminal and show it
    fg.set_terminal("wxt", "");
    fg.show().unwrap();
}
