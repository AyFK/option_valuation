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
#[allow(unused_must_use)]
impl TraderProcess {

    /*
    pub fn add(&mut self) { // almost...
        Rc::clone(self).join(Rc::clone(&self.broker));
    }
    */

    /*
    pub fn new(broker: Rc<Broker>, strategy: Action, name: String, balance: f64,
               simulations_total: usize, simulation_length: usize) -> Self {

        let outcomes = vec![vec![0.0_f64; simulation_length];
                                          simulations_total];

        //self.join(broker.clone());

        Self { broker, strategy, name, balance: Cell::new(balance),
               portfolio_process: RefCell::new(outcomes) }
    }
    */


    pub fn new(broker: Rc<Broker>, strategy: Action, name: String, balance: f64,
               simulations_total: usize, simulation_length: usize) {

        let outcomes = vec![vec![0.0_f64; simulation_length];
                                          simulations_total];

        //self.join(broker.clone());
        //let test_2: Rc::clone(&broker);
        let test_2 = broker.clone();

        let test =
        Self { broker, strategy, name, balance: Cell::new(balance),
               portfolio_process: RefCell::new(outcomes) };

        //&test.join(Rc::clone(&broker));
        &test.join(test_2);

    }


}


#[allow(dead_code)]
pub enum Action {
    // does nothing
    Lurker,

    // hedges a (strike, maturity)
    ConstantCallHedger(f64, usize),
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl Action {

    pub fn strategy(&self) {

        match self {
            Action::Lurker => {
            },

            Action::ConstantCallHedger(strike, maturity) => {
            },
        }
    }
}
