use std::collections::HashMap;

use crate::maths::{self, *};
use crate::dynamics::fetchDB;

pub fn say_hi() {
    println!("hi");
}

/// Infere paramters: x0, μ, σ, used in the GBM price process.
/// Returns: `HashMap`, with key-value pairs:
/// "x0"    ->  starting value.
/// "mu"    ->  trend.
/// "sigma" ->  volatility.
pub fn invoke(ticker: &str) -> HashMap<String, f64> {

    let fetchDB::CloseData {price, log_return} = fetchDB::ts_close(ticker);


    let process_params = HashMap::new();


    return process_params;
}
