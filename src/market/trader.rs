use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::asset::AssetProcess;
use super::brokerage::*;


#[allow(dead_code)]
pub struct TraderProcess {
    pub broker: Rc<Broker>,
    pub strategy: Action,
    pub name: String,
    pub balances: Vec<Cell<f64>>,
    pub ownership: Vec<RefCell<HashMap<*const AssetProcess, i64>>>
    pub portfolio_processes: Vec<Vec<Cell<f64>>>,
}


#[allow(dead_code)]
impl TraderProcess {


    pub fn new(broker: Rc<Broker>, strategy: Action, name: String,
               starting_balance: f64, simulations_total: usize,
               simulation_length: usize) {

        let ownership = vec![RefCell::new(HashMap::new());
                                       simulations_total];

        let balances = vec![Cell::new(starting_balance);
                                     simulations_total];

        let portfolio_outcomes = vec![Cell::new(0.0); simulation_length];
        let portfolio_processes = vec![portfolio_outcomes; simulations_total];

        let instance = Self { broker: Rc::clone(&broker), strategy, name,
                              balances, ownership, portfolio_processes };

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
