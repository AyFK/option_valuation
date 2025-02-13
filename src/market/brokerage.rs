use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::asset::*;
use super::trader::*;


#[allow(dead_code)]
pub trait Member {
    fn join(self: Self, broker: Rc<Broker>);
    fn update(&self);
}


#[allow(unused_variables)]
impl Member for AssetProcess {
    fn join(self: Self, broker: Rc<Broker>) {
        broker.all_assets.borrow_mut().push(Rc::new(self));
    }

    fn update(&self) {
        // update price for 'AssetProcess'
        let dy = self.dy();

        let newprice: f64 = dy.exp();
        self.price_processes.borrow_mut();
        self.return_processes.borrow_mut();


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

    fn update(&self) {
        self.balance.set(0.0); // works
        // update portfolio value for 'TraderProcess'
    }
}



#[allow(dead_code)]
pub struct Broker {
    pub lifetime: usize,
    pub time: Cell<usize>,
    pub all_assets: RefCell<Vec<Rc<AssetProcess>>>,
    pub all_traders: RefCell<Vec<Rc<TraderProcess>>>,
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl Broker {

    pub fn open(lifetime: usize) -> Self {
        Self { lifetime, time: Cell::new(1),
               all_assets: RefCell::new(Vec::new()),
               all_traders: RefCell::new(Vec::new()) }

    }

    pub fn buy_order(trader: Rc<TraderProcess>, asset: Rc<AssetProcess>,
                     volume: usize) {

        let spot_price = 1.0;
        let cost = spot_price * volume as f64;
        trader.balance.set(trader.balance.get() - cost);
    }
        //trader->balance -= asset->spot_price * volume;
        //trader->ownership[asset] += volume;

    pub fn update(&self) {

        for asset in self.all_assets.borrow().iter() {
            asset.update();
        }

        for trader in self.all_traders.borrow().iter() {
            trader.update();
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

        match self {
            EuropeanOption::Call(underlying, strike, maturity, writer, owner) => {
                if time == *maturity {
                    // get 'price' from '&asset' instead
                    let expired: f64 = (price - *strike).max(0.0);

                    writer.balance.set(writer.balance.get() - expired);
                    owner.balance.set(owner.balance.get() + expired);
                }
            },
            EuropeanOption::Put(underlying, strike, maturity, writer, owner) => {
                if time == *maturity {
                    // get 'price' from '&asset' instead
                    let expired: f64 = (*strike - price).max(0.0);

                    writer.balance.set(writer.balance.get() - expired);
                    owner.balance.set(owner.balance.get() + expired);
                }
            },
        }
    }
}
