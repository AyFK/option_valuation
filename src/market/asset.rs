use core::f64;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use super::brokerage::*;
use crate::dynamics::black_scholes::{self};



#[allow(dead_code)]
pub enum Dynamics {
    BlackScholes,
    Binomial,
}

#[allow(dead_code)]
pub struct AssetProcess {
    pub broker: Rc<Broker>,

    pub process: Dynamics,
    pub process_params: HashMap<String, f64>,

    pub ticker: String,

    //pub spot_price: Vec<Cell<f64>>,
    pub price_processes: Vec<Vec<Cell<f64>>>,
    pub return_processes: Vec<Vec<Cell<f64>>>,

}

#[allow(dead_code)]
impl AssetProcess {
    pub fn new(broker: Rc<Broker>, process: Dynamics, ticker: String) {


        // fetch number of simulation total and their length from broker
        let simulations_total = broker.simulations_total;
        let simulation_length = broker.simulation_length;

        let process_params = black_scholes::inference::invoke(&ticker);

        // matrix of price processes, all starting at 'x0'
        let price_outcomes = vec![Cell::new(0.0); simulation_length + 1];
        price_outcomes[0].set(process_params["x0"]);
        let price_processes = vec![price_outcomes; simulations_total];

        // matrix of return processes
        let return_outcomes= vec![Cell::new(0.0); simulation_length];
        let return_processes = vec![return_outcomes; simulations_total];


        // instantiate the object
        let instance = Self { broker: Rc::clone(&broker), process,
                              process_params, ticker, price_processes,
                              return_processes };

        // call join trait
        instance.join(Rc::clone(&broker));
    }


    fn get_data() { // ska kallas i Asset::new()


    }

    fn inference(&self) -> HashMap<String, f64> {

        match self.process {
            Dynamics::BlackScholes => {
                return black_scholes::inference::invoke(&self.ticker);
            }

            Dynamics::Binomial => {
                let mut params = HashMap::new();
                params.insert("x0".to_string(), 0.0);
                params.insert("u".to_string(), 0.0);
                params.insert("d".to_string(), 0.0);
                return params;
            }
        }
    }


    pub fn dy(&self) -> f64 {

        match self.process {
            Dynamics::BlackScholes => {
                black_scholes::dy::invoke(&self.process_params);
            }

            Dynamics::Binomial => {
            }
        }
        return 0.0;
    }
}

