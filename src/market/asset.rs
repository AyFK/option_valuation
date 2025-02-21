use core::f64;
use std::cell::Cell;
//use std::collections::HashMap;
use std::rc::Rc;

use super::brokerage::*;
//use crate::dynamics::{fetch_db, black_scholes::{self}};

use crate::dynamics::enum_impl::Dynamics;




#[allow(dead_code)]
pub struct AssetProcess {
    pub broker: Rc<Broker>,

    pub process: Dynamics,
    pub ticker: String,

    pub price_processes: Vec<Vec<Cell<f64>>>,
    pub return_processes: Vec<Vec<Cell<f64>>>,

}


#[allow(dead_code)]
impl AssetProcess {
    pub fn new(broker: Rc<Broker>, mut process: Dynamics,
               ticker: String) -> Rc<Self> {

        // fetch number of simulation total and their length from broker
        let simulations_total = broker.simulations_total;
        let simulation_length = broker.simulation_length;

        // get process paramters
        let x0 = process.inference(&ticker);

        // matrix of price processes, all starting at 'x0'
        let price_outcomes = vec![Cell::new(0.0); simulation_length + 1];
        price_outcomes[0].set(x0);
        let price_processes = vec![price_outcomes; simulations_total];

        // matrix of return processes
        let return_outcomes= vec![Cell::new(0.0); simulation_length];
        let return_processes = vec![return_outcomes; simulations_total];

        // instantiate the object
        let instance = Self { broker: Rc::clone(&broker), process,
                              ticker, price_processes, return_processes };

        // make an 'Rc<_>' of 'instance'
        let rc_instance = Rc::new(instance);

        // call 'join' trait let 'broker' have ownership of instance too
        (&rc_instance).join(Rc::clone(&broker));

        // return ownership such that 'TradingProcess' can put ownership
        // into 'Mechanics'
        return rc_instance;
    }
}

