
//use serde::Deserialize;
use serde::Deserialize;
use serde_json::from_reader;

use std::fs::File;
use std::io::BufReader;


/// Represents a single OHLCV record.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OHLCV {
    #[serde(rename = "dt")]
    pub datetime: String,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: i64,
}

/// Represents a time series of OHLCV data.
#[allow(dead_code)]
pub struct TimeSeriesOHLCV {
    pub data: Vec<OHLCV>,
}


/// Represents close-price related data (price and log returns).
#[allow(dead_code)]
pub struct CloseData {
    pub price: Vec<f64>,
    pub log_return: Vec<f64>,
}


/// Returns a all OHLCV data from 'ticker' stored in our
/// local database.
#[allow(dead_code)]
pub fn parse_stock_data(ticker: &str) -> TimeSeriesOHLCV {
    let data_path = format!("../database/_{}.json", ticker);
    let file = File::open(&data_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let data: Vec<OHLCV> = from_reader(reader).expect("Failed to parse JSON");
    return TimeSeriesOHLCV { data };
}


/// Returns a both price and log returns as `Vec<f64>` time
/// series from 'ticker' data stored in our local database.
#[allow(dead_code)]
pub fn ts_close(ticker: &str) -> CloseData {
    let data_path = format!("../database/_{}.json", ticker);
    println!("{}", data_path);
    let file = File::open(&data_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let data: Vec<OHLCV> = from_reader(reader).expect("Failed to parse JSON");

    let mut log_return = Vec::with_capacity(data.len().saturating_sub(1));
    let mut price = Vec::with_capacity(data.len().saturating_sub(1));

    for window in data.windows(2) {
        let previous = window[0].close;
        let current = window[1].close;
        let change = current.ln() - previous.ln();
        log_return.push(100.0 * change);
        price.push(current);
    }

    return CloseData { price, log_return };
}
