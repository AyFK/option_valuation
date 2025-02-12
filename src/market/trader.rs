use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::brokerage::*;


#[allow(dead_code)]
pub enum Action {
    Lurker,
    ConstantCallHedger,
}


#[allow(dead_code)]
pub struct TraderProcess {
    pub broker: Rc<Broker>,
    pub strategy: Action,
    pub name: String,
    pub balance: Cell<f64>,
    pub portfolio_process: RefCell<Vec<Vec<f64>>>,
}


#[allow(dead_code)]
impl TraderProcess {

    pub fn new(broker: Rc<Broker>, strategy: Action, name: String, balance: f64,
               simulations_total: usize, simulation_length: usize) -> Self {

        let outcomes = vec![vec![0.0_f64; simulation_length];
                                          simulations_total];

        //self.join(broker.clone());

        Self { broker, strategy, name, balance: Cell::new(balance),
               portfolio_process: RefCell::new(outcomes) }
    }
}


