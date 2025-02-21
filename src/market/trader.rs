use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::asset::AssetProcess;
use super::brokerage::*;
use super::ptrhash::WeakPtrHash;

use crate::mechanics::enum_impl::Mechanics;



#[allow(dead_code)]
pub struct TraderProcess {
    pub broker: Rc<Broker>,
    pub strategy: Mechanics,
    pub name: String,
    pub balances: Vec<Cell<f64>>,
    pub ownerships: Vec<RefCell<HashMap<WeakPtrHash<AssetProcess>, i64>>>,
    pub portfolio_processes: Vec<Vec<Cell<f64>>>,
}


#[allow(dead_code)]
impl TraderProcess {

    pub fn new(broker: Rc<Broker>, mut strategy: Mechanics, name: String,
               starting_balance: f64) {

        // fetch number of simulation total and their length from broker
        let simulations_total = broker.simulations_total;
        let simulation_length = broker.simulation_length;

        // do due diligence on the to fine-tune strategy based on
        // historical prices
        strategy.due_diligence();

        // create vector of ownerships, one for each unique simulation
        let ownerships = vec![RefCell::new(HashMap::new());
                                        simulations_total];

        // create vector of balances, one for each unique simulation
        let balances = vec![Cell::new(starting_balance);
                                     simulations_total];

        // matrix of portfolio processes, all starting at 'starting_balance'
        let portfolio_outcomes = vec![Cell::new(0.0); simulation_length + 1];
        portfolio_outcomes[0].set(starting_balance);
        let portfolio_processes = vec![portfolio_outcomes; simulations_total];

        // instantiate the object
        let instance = Self { broker: Rc::clone(&broker), strategy, name,
                              balances, ownerships, portfolio_processes };

        // make an 'Rc<_>' of 'instance'
        let rc_instance = Rc::new(instance);

        // call 'join' trait let 'broker' have ownership of instance
        (&rc_instance).join(Rc::clone(&broker));
    }
}
