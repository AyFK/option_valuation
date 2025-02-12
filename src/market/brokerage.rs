use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use super::asset::*;
use super::trader::*;


/*
#[allow(dead_code)]
pub trait Member {
    fn join(self: Rc<Self>, broker: Rc<Broker>);
}


impl Member for AssetProcess {
    fn join(self: Rc<Self>, broker: Rc<Broker>) {
        println!("passed! As");
        broker.all_assets.borrow_mut().push(self);
    }
}


impl Member for TraderProcess {
    fn join(self: Rc<Self>, broker: Rc<Broker>) {
        println!("passed! Tr");
        broker.all_traders.borrow_mut().push(self);
    }
}
*/

#[allow(dead_code)]
pub trait Member {
    fn join(self: Self, broker: Rc<Broker>);
}


impl Member for AssetProcess {
    fn join(self: Self, broker: Rc<Broker>) {
        println!("passed! As");
        broker.all_assets.borrow_mut().push(Rc::new(self));
    }
}


impl Member for TraderProcess {
    fn join(self: Self, broker: Rc<Broker>) {
        println!("passed! Tr");
        broker.all_traders.borrow_mut().push(Rc::new(self));
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
    /*
    pub fn new(players: Vec<TraderProcess>, assets: Vec<AssetProcess>,
               lifetime: usize) -> Self {

        Self { lifetime, time: 1 }
    }
    */

    pub fn open(lifetime: usize) -> Self {
        Self { lifetime, time: Cell::new(1), all_assets: RefCell::new(Vec::new()), all_traders: RefCell::new(Vec::new()) }

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
