use std::rc::Rc;

use gnuplot::*;

use super::plot_tools::{stems::{self, *}, time_series::{self, *}, two_unit_plots::{self, *}};
use crate::market::{asset::AssetProcess, trader::TraderProcess};

pub fn figure(asset: &Rc<AssetProcess>, trader: &Rc<TraderProcess>) {

    let sim_idx = 0;

    // convert Cell items into regular f64;s for processes of interest
    let portfolio_price: Vec<f64> = trader.portfolio_processes[sim_idx].iter()
                                          .map(|cell| cell.get())
                                          .collect();

    let price_process: Vec<f64> = asset.price_processes[sim_idx].iter()
                                       .map(|cell| cell.get())
                                       .collect();

    let return_process: Vec<f64> = asset.return_processes[sim_idx].iter()
                                        .map(|cell| cell.get())
                                        .collect();
    //return_process.insert(0, 0.0);


    // create uniform time parition 'x'
    let n: usize = std::cmp::max(price_process.len(),
                                 price_process.len());

    let x: Vec<f64> = (0..n).map(|v| v as f64).collect();

    // establish mutable gnuplot figure
    let mut fg = Figure::new();

    let color1: &str = "blue";
    let color2: &str = "red";

    { // create the first axes within its own scope
        let ax1 = fg.axes2d(); // borrow fg

        // first row, first column
        ax1.set_pos_grid(1, 2, 0);
        ax1.set_x_grid(true);
        ax1.set_y_grid(true);

        // rescale 'portfolio_price' to fit into 'ax1'
        let portfolio_price_scale = two_unit_plots::re_scaled(ax1,
                                    &price_process, &portfolio_price);

        ax1.lines(&x, &price_process, &[Color(color1)]);

        ax1.set_y_ticks(Some((AutoOption::Auto, 0)), &[],
                            &[LabelOption::TextColor(color1)]);

        ax1.set_y_label("S(t)", &[LabelOption::TextColor(color1)]);

        ax1.lines(&x, &portfolio_price_scale, &[Color(color2),
                  LineStyle(DashType::Dash)]);

        ax1.set_y2_ticks(Some((AutoOption::Auto, 0)), &[],
                        &[LabelOption::TextColor(color2)]);
        ax1.set_y2_label("Π(t)", &[LabelOption::TextColor(color2)]);
    } // ax1 is dropped, releasing fg

    { // create the second axes within its own scope
        let ax2 = fg.axes2d(); // borrow fg

        // first row, second column
        ax2.set_pos_grid(1, 2, 1);
        ax2.set_x_grid(true);
        ax2.set_y_grid(true);

        //time_series::plot(ax2, &return_process);

        let (x_segments, y_segments) = stems::stem_segments(&return_process);


        ax2.lines(&x_segments, &y_segments, &[]);

        // use plot method to draw dots on top of segments
        //let options: [PlotOption<&str>; 3] = [Caption("Dot"), Color("#66AAD7"),
        //                                      PointSymbol('o')];
        //ax2.points(&x, y, &options);


        ax2.set_y_ticks(Some((AutoOption::Auto, 0)), &[],
                        &[LabelOption::TextColor("black")]);

        ax2.set_y_label("% (bp)", &[LabelOption::
                        TextColor("black")]);
    } // ax2 is dropped, releasing fg



    // set smoother gnuplot terminal and show it
    fg.set_terminal("wxt", "");
    fg.show().unwrap();
}
