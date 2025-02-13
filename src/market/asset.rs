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
               simuleations_total: usize, simulation_length: usize) {


        let price_processes = RefCell::new(vec![vec![0.0; simulation_length];
                                                        simuleations_total]);

        let return_processes = RefCell::new(vec![vec![0.0; simulation_length];
                                                         simuleations_total]);

        let params = HashMap::new();


        // kalla på get_data()
        // kalla på inference()?!

        let instance = Self { broker: Rc::clone(&broker), process, params,
                              ticker, price_processes, return_processes };

        instance.join(Rc::clone(&broker));
    }


    fn get_data() { // ska kallas i Asset::new()


    }

    fn inference(&self) -> HashMap<String, f64> {
        let mut params = HashMap::new();

        match self.process {
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
        params
    }


    pub fn dy(&self) -> f64 {

        match self.process {
            Dynamics::BlackScholes => {
                0.0
            }

            Dynamics::Binomial => {
                0.0
            }
        }
    }
}

