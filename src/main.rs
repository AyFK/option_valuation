use std::rc::Rc;
use std::io::{self, Write};

mod market;
#[allow(unused_imports)]
use market::asset::*;
use market::trader::*;
use market::brokerage::*;

mod dynamics;  // imports used in asset.rs
mod mechanics; // imports used in trader.rs
mod maths;     // import used throughout proj
mod plots;
mod database;
mod datastructs;
mod derivatives;

use mechanics::enum_impl::Mechanics;
use dynamics::enum_impl::Dynamics;


#[allow(dead_code)]
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}


#[allow(unused_variables)]
fn main() {


    let simulations_total = 1000;
    let simulation_length = 255;

    // broker is responsible for the simulation
    let broker = Rc::new(Broker::new(simulations_total, simulation_length));

    let spx = AssetProcess::new(Rc::clone(&broker), Dynamics::BlackScholes(
                                None,None), String::from("SPX"));

    let bob = TraderProcess::new(Rc::clone(&broker), Mechanics::LongCallConstHedger(
                       Rc::clone(&spx), 6100.0, 200, None), String::from("Bob"),
                       0.0);

    let tom = TraderProcess::new(Rc::clone(&broker), Mechanics::ShortCallConstHedger(
                       Rc::clone(&spx), 6100.0, 200, None), String::from("Tom"), 0.0);



    // run simulation
    broker.open();

    plots::plot_long_vs_short::invoke(&spx, &bob, &tom);

    let _ = input("\n[Enter]");
}
