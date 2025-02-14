use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::asset::*;
use super::trader::*;


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

        let dy = self.dy();

        let new_price: f64 = dy.exp();
        self.price_processes[sim_idx][time_idx].get();
        self.return_processes[sim_idx][time_idx].get();


        /*
        new_price = this->spot_price * std::pow(M_E, dX);
        this->spot_price = new_price;
        this->price_process[t] = new_price;
        this->return_process[t-1] = 100 * dX;
        */

    }
}


impl Member for TraderProcess {
    fn join(self: Self, broker: Rc<Broker>) {
        broker.all_traders.borrow_mut().push(Rc::new(self));
    }

    fn update(&self, sim_idx: usize, time_idx: usize) {
        // update portfolio value for 'TraderProcess'

        let mut equity: f64 = 0.0;
        let position = self.ownership[sim_idx].borrow();

        for (asset, volume) in position.iter() {
            let spot_price = asset.price_processes[sim_idx][time_idx].get();
            let spot_volume = *volume as f64;
            equity += spot_price * spot_volume;
        }

        equity += self.balances[sim_idx].get();
        self.portfolio_processes[sim_idx][time_idx].set(equity);
    }
}



#[allow(dead_code)]
pub struct Broker {
    pub lifetime: usize,
    pub time_idx: Cell<usize>,
    pub sim_idx: Cell<usize>,
    pub all_assets: RefCell<Vec<Rc<AssetProcess>>>,
    pub all_traders: RefCell<Vec<Rc<TraderProcess>>>,
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl Broker {

    pub fn open(lifetime: usize) -> Self {

        Self { lifetime, time_idx: Cell::new(0),
               sim_idx: Cell::new(0),
               all_assets: RefCell::new(Vec::new()),
               all_traders: RefCell::new(Vec::new()) }

    }

    pub fn buy_order(&self, trader: Rc<TraderProcess>,
                     asset: Rc<AssetProcess>, volume: usize) {

        let sim_idx = self.sim_idx.get();

        let spot_price = 1.0;
        let bankroll = spot_price * volume as f64;
        trader.balances[sim_idx].set(trader.balances[sim_idx].get() - bankroll);
    }
        //trader->balance -= asset->spot_price * volume;
        //trader->ownership[asset] += volume;

    pub fn update(&self) {
        let sim_idx = self.sim_idx.get();
        let time_idx = self.time_idx.get();

        for asset in self.all_assets.borrow().iter() {
            asset.update(sim_idx, time_idx);
        }

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
                    let expired: f64 = (price - *strike).max(0.0);

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
                    let expired: f64 = (*strike - price).max(0.0);

                    writer.balances[sim_idx].set(writer.balances[sim_idx].
                                             get() - expired);

                    owner.balances[sim_idx].set(owner.balances[sim_idx].
                                            get() + expired);
                }
            },
        }
    }
}
