use std::collections::HashMap;

use crate::maths::stats::{self};
use crate::dynamics::fetch_db;

/// Infere paramters: x0, μ, σ, used in the GBM price process.
/// Returns: `HashMap`, with key-value pairs:
/// "x0"    ->  starting value.
/// "mu"    ->  trend.
/// "sigma" ->  volatility.
pub fn invoke(ticker: &str) -> HashMap<String, f64> {

    let fetch_db::CloseData {price, log_return} = fetch_db::ts_close(
                                                  ticker, None);

    let x0 = *price.last().unwrap();
    let mu = stats::arithmetic_mean(&log_return);
    let sigma = stats::standard_deviation(&log_return);

    //println!("{} {} {}", x0, mu / 100.0, sigma / 100.0); // works

    let mut params = HashMap::new();
    params.insert(String::from("x0"), x0);
    params.insert(String::from("mu"), mu);
    params.insert(String::from("sigma"), sigma);

    return params;
}
