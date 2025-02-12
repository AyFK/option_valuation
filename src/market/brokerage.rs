use std::rc::Rc;

use super::asset::*;
use super::trader::*;


#[allow(dead_code)]
pub struct Broker {
    pub lifetime: usize,
    pub time: usize,
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl Broker {
    pub fn new(players: Vec<TraderProcess>, assets: Vec<AssetProcess>,
               lifetime: usize) -> Self {

        Self { lifetime, time: 1 }
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
