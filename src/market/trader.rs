use std::cell::Cell;
use std::cell::RefCell;

#[allow(dead_code)]
pub struct TraderProcess {
    pub name: String,
    pub balance: Cell<f64>,
    pub portfolio_process: RefCell<Vec<Vec<f64>>>,
}


#[allow(dead_code)]
impl TraderProcess {

    pub fn new(name: String, balance: f64, simulations_total: usize,
               simulation_length: usize) -> Self {

        let outcomes = vec![vec![0.0_f64; simulation_length];
                                          simulations_total];

        Self { name, balance: Cell::new(balance),
               portfolio_process: RefCell::new(outcomes) }
    }
}
