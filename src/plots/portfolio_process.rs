use std::cell::Cell;

use gnuplot::*;

pub fn figure(portfolio_process: &[Cell<f64>]) {

    // convert Cell items into regular f64;s
    let y: Vec<f64> = portfolio_process.iter()
                       .map(|cell| cell.get())
                       .collect();

    // create uniform time parition 'x'
    let n: usize = std::cmp::max(portfolio_process.len(),
                                 portfolio_process.len());

    let x: Vec<f64> = (0..n).map(|v| v as f64).collect();

    // establish mutable gnuplot figure
    let mut fg = Figure::new();

    // borrow fg as a axes
    let ax1 = fg.axes2d();

    // draw line on axes
    ax1.lines(&x, &y, &[]);

    // set smoother gnuplot terminal and show it
    fg.set_terminal("wxt", "");
    fg.show().unwrap();
}
