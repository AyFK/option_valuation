use core::f64;
use std::usize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::brokerage::*;

#[allow(dead_code)]
pub enum Dynamics {
    BlackScholes,
    Binomial,
}

#[allow(dead_code)]
pub struct AssetProcess {
    pub broker: Rc<Broker>,
    pub process: Dynamics,
    pub params: HashMap<String, f64>,
    pub ticker: String,
    pub price_processes: RefCell<Vec<Vec<f64>>>,
    pub return_processes: RefCell<Vec<Vec<f64>>>,

}

#[allow(dead_code)]
impl AssetProcess {
    pub fn new(broker: Rc<Broker>, process: Dynamics, ticker: String,
               simuleations_total: usize, simulation_length: usize) -> Self {


        let price_processes = RefCell::new(vec![vec![0.0; simulation_length];
                                                        simuleations_total]);

        let return_processes = RefCell::new(vec![vec![0.0; simulation_length];
                                                         simuleations_total]);

        let params = HashMap::new();

        // broker.joinExchange()

        // kalla på get_data()
        // kalla på inference()?!

        Self { broker, process, params, ticker, price_processes, return_processes }
    }


    fn get_data() { // ska kallas i Asset::new()


    }

    fn inference(process: Dynamics) {
        let mut params = HashMap::new();

        match process {
            Dynamics::BlackScholes => {
                params.insert("x0".to_string(), 0.0);
                params.insert("mu".to_string(), 0.0);
                params.insert("sigma".to_string(), 0.0);
            }

            Dynamics::Binomial => {
                params.insert("u".to_string(), 0.0);
                params.insert("d".to_string(), 0.0);
            }
        }
    }

    //pub fn dX(process: Dynamics)

}

