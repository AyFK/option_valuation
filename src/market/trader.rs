use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::brokerage::*;


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
               simulations_total: usize, simulation_length: usize) {

        let outcomes = vec![vec![0.0; simulation_length];
                                      simulations_total];

        let instance = Self { broker: Rc::clone(&broker), strategy, name,
                              balance: Cell::new(balance), portfolio_process:
                              RefCell::new(outcomes) };

        instance.join(Rc::clone(&broker));
    }
}


#[allow(dead_code)]
pub enum Action {
    // does nothing
    Lurker,

    // hedges a (strike, maturity) call option at every possible time unit
    CallConstHedger(f64, usize),
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl Action {

    pub fn strategy(&self) {

        match self {
            Action::Lurker => {
            },

            Action::CallConstHedger(strike, maturity) => {
            },
        }
    }
}
