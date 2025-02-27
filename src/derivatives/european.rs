use std::rc::Rc;

use crate::market::{asset::AssetProcess, trader::TraderProcess};

#[allow(dead_code)]
pub enum European {
    Call,
    Put,
}


impl European {

    pub fn pay_off(&self, spot: f64, strike: f64) -> f64 {

        match &self {

            European::Call => {
                return (spot - strike).max(0.0);
            },

            European::Put => {
                return (strike - spot).max(0.0);
            },
        }
    }
}



pub struct EuropeanOption {
    option: European,
    num_underlying: usize,
    underlying: Rc<AssetProcess>,
    strike: f64,
    maturity: usize,

    // writers and owners are `Option`;s due to possibility
    // of 'auto filling' orders for simulation purposes.
    writer: Option<Rc<TraderProcess>>,
    owner: Option<Rc<TraderProcess>>,
}



#[allow(dead_code)]
impl EuropeanOption {


    /// Option 'owner' pays premium and 'writer' receives
    /// it. Instantiate and return the object.
    pub fn new(premium: f64, sim_idx: usize, option: European,
               underlying: Rc<AssetProcess>, num_underlying: usize,
               strike: f64, maturity: usize,
               writer: Option<Rc<TraderProcess>>,
               owner: Option<Rc<TraderProcess>>) -> Self {

        // if 'owner' is defined, it pays the premium
        if let Some(ref trader) = owner {
            trader.balances[sim_idx].set(trader.balances[sim_idx].get()
                                         - premium * num_underlying as f64);
        }

        // if 'writer' is defined, it receives the premium
        if let Some(ref trader) = writer {
            trader.balances[sim_idx].set(trader.balances[sim_idx].get()
                                         + premium * num_underlying as f64);
        }

        return Self { option, num_underlying, underlying, strike,
                      maturity, writer, owner };
    }


    /// Exercise the option and fulfill the obligation.
    /// The 'owner' receives the pay-off (cash-settled)
    /// while the 'writer' pays for it.
    pub fn exercise(&self, sim_idx: usize, time_idx: usize) {

        // check if maturity is reached
        if time_idx == self.maturity {

            // get the current price of the underlying
            let spot_price = self.underlying.price_processes[
                                  sim_idx][time_idx].get();

            // calculate the pay-off
            let pay_off = self.option.pay_off(spot_price, self.strike);

            // if 'owner' is defined, it receives the pay-off
            if let Some(ref trader) = self.owner {
                trader.balances[sim_idx].set(trader.balances[sim_idx].get()
                                             + pay_off * self.num_underlying
                                             as f64);
            }

            // if 'writer' is defined, it covers the pay-off
            if let Some(ref trader) = self.writer {
                trader.balances[sim_idx].set(trader.balances[sim_idx].get()
                                             - pay_off * self.num_underlying
                                             as f64);
            }
        }
    }
}
