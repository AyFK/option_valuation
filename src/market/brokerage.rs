use core::f64;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::asset::*;
use super::trader::*;
use super::ptrhash::WeakPtrHash;


#[allow(dead_code)]
pub trait Member {
    fn join(self: Self, broker: Rc<Broker>);
    fn update(&self, sim_idx: usize, time_idx: usize);
}


#[allow(unused_variables)]
impl Member for AssetProcess {
    fn join(self: Self, broker: Rc<Broker>) {
        broker.all_assets.borrow_mut().push(Rc::new(self));
    }

    fn update(&self, sim_idx: usize, time_idx: usize) {
        // update price for 'AssetProcess'

        let price = self.price_processes[sim_idx][time_idx].get();
        let dy = self.dy();

        let new_price = price * dy.exp();
        self.price_processes[sim_idx][time_idx+1].set(new_price);
        self.return_processes[sim_idx][time_idx].set(dy * 100.0);
    }
}


impl Member for TraderProcess {
    fn join(self: Self, broker: Rc<Broker>) {
        broker.all_traders.borrow_mut().push(Rc::new(self));
    }

    fn update(&self, sim_idx: usize, time_idx: usize) {
        // update portfolio value for 'TraderProcess'

        let mut equity: f64 = 0.0;
        let position = self.ownerships[sim_idx].borrow();


        for (asset_ptr, volume) in position.iter() {
            if let Some(strong_reference) = asset_ptr.weak_reference.upgrade() {
                let spot_price = strong_reference.
                                 price_processes[sim_idx][time_idx].get();
                let spot_volume = *volume as f64;
                equity += spot_price * spot_volume;
            }
            else {
                println!("Error: Referenced object has been dropped.");
            }
        }

        equity += self.balances[sim_idx].get();
        self.portfolio_processes[sim_idx][time_idx].set(equity);
    }
}



#[allow(dead_code)]
pub struct Broker {
    pub simulations_total: usize,
    pub sim_idx: Cell<usize>,

    pub simulation_length: usize,
    pub time_idx: Cell<usize>,

    pub all_assets: RefCell<Vec<Rc<AssetProcess>>>,
    pub all_traders: RefCell<Vec<Rc<TraderProcess>>>,
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl Broker {

    pub fn new(simulations_total: usize, simulation_length: usize) -> Self {

        Self { simulations_total, time_idx: Cell::new(0),
               simulation_length, sim_idx: Cell::new(0),
               all_assets: RefCell::new(Vec::new()),
               all_traders: RefCell::new(Vec::new()) }

    }

    pub fn open(&self) {


        for i in 0..self.simulations_total {
            self.sim_idx.set(i);

            for j in 0..self.simulation_length {
                self.time_idx.set(j);

                // ... action

                self.update();
            }
        }
    }


    fn transfer_funds(&self, trader: &Rc<TraderProcess>,
                      asset: &Rc<AssetProcess>, volume: i64) {

        let sim_idx = self.sim_idx.get();
        let time_idx = self.time_idx.get();

        let spot_price = trader.portfolio_processes[sim_idx][time_idx].get();
        let spot_volume = volume as f64;
        let bankroll = spot_price * spot_volume;

        let new_bal = trader.balances[sim_idx].get() - bankroll;
        trader.balances[sim_idx].set(new_bal);
    }


    fn transfer_ownership(&self, trader: &Rc<TraderProcess>,
                          asset: &Rc<AssetProcess>, volume: i64) {

        let sim_idx = self.sim_idx.get();
        let ticker_key = WeakPtrHash{weak_reference: Rc::downgrade(&asset)};

        // borrow RefCell mutably to access the HashMap
        let mut position = trader.ownerships[sim_idx].borrow_mut();

        // retrieve the current value or default to 0 if position do not exist
        let curr_volume = *position.get(&ticker_key).unwrap_or(&0);

        // update the value associated with key
        position.insert(ticker_key, curr_volume + volume);
    }


    pub fn buy_order(&self, trader: &Rc<TraderProcess>,
                     asset: &Rc<AssetProcess>, volume: i64) {
        self.transfer_funds(trader, asset, volume);
        self.transfer_ownership(trader, asset, volume);
    }


    pub fn sell_order(&self, trader: &Rc<TraderProcess>,
                      asset: &Rc<AssetProcess>, volume: i64) {
        self.transfer_funds(trader, asset, -volume);
        self.transfer_ownership(trader, asset, -volume);
    }


    pub fn update(&self) {
        let sim_idx = self.sim_idx.get();
        let time_idx = self.time_idx.get();

        println!("{} {}", sim_idx, time_idx);

        // update asset trajectory first
        for asset in self.all_assets.borrow().iter() {
            asset.update(sim_idx, time_idx);
        }

        // based on asset trajectory, update traders portfolios second
        for trader in self.all_traders.borrow().iter() {
            trader.update(sim_idx, time_idx);
        }
    }
}




#[allow(dead_code)]
pub enum EuropeanOption {
    // (&asset, strike, maturity, &writer, &owner)
    Call(Rc<AssetProcess>, f64, usize, Rc<TraderProcess>,
         Rc<TraderProcess>),

    // (&asset, strike, maturity, &writer, &owner)
    Put(Rc<AssetProcess>, f64, usize, Rc<TraderProcess>,
        Rc<TraderProcess>),
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl EuropeanOption {

    pub fn pay_off(&self, price: f64, time: usize) {

        let sim_idx = 0;

        match self {
            EuropeanOption::Call(underlying, strike, maturity,
                                 writer, owner) => {
                if time == *maturity {
                    // get 'price' from '&asset' instead
                    let expired = (price - *strike).max(0.0);

                    writer.balances[sim_idx].set(writer.balances[sim_idx].
                                             get() - expired);

                    owner.balances[sim_idx].set(owner.balances[sim_idx].
                                            get() + expired);
                }
            },
            EuropeanOption::Put(underlying, strike, maturity,
                                writer, owner) => {
                if time == *maturity {
                    // get 'price' from '&asset' instead
                    let expired = (*strike - price).max(0.0);

                    writer.balances[sim_idx].set(writer.balances[sim_idx].
                                             get() - expired);

                    owner.balances[sim_idx].set(owner.balances[sim_idx].
                                            get() + expired);
                }
            },
        }
    }
}
