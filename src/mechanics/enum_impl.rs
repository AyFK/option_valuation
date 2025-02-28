use std::rc::Rc;

use crate::market::{trader::TraderProcess, asset::AssetProcess};
use super::{lurker, long_const_hedge_call, short_const_hedge_call};

// 'fetch_db' has to move, both 'Mechanics' and 'Dynamics' use it...
use crate::database::fetch_db;


/// Trading pattern (`Mechanics`) for `TraderProcess`.
#[allow(dead_code)]
pub enum Mechanics {
    /// Purchases 1x asset at the start and holds until end
    /// of simulation.
    Lurker(Rc<AssetProcess>),

    /// Hedges a long (asset, strike, maturity, implied_volatility)
    /// call option at every possible time unit.
    LongCallConstHedger(Rc<AssetProcess>, f64, usize, Option<f64>),

    /// Hedges a short (asset, strike, maturity, implied_volatility)
    /// call option at every possible time unit.
    ShortCallConstHedger(Rc<AssetProcess>, f64, usize, Option<f64>),
}



impl Mechanics {

    /// Execute trading strategy (`Mechanics`) on `&TraderProcess`.
    #[allow(dead_code)]
    pub fn trade(&self, trader: &Rc<TraderProcess>) {

        match &self {

            Mechanics::Lurker(asset) => {
                // call external function
                lurker::strategy::invoke(trader, asset);
            },

            Mechanics::LongCallConstHedger(asset, strike, maturity,
                                           implied_volatility) => {

                if let Some(sigma) = implied_volatility {
                    // call external function
                    long_const_hedge_call::strategy::invoke(trader, asset,
                                         *strike, *maturity, *sigma);

                }

                else {
                    println!("ERROR: 'due_diligence()' not called on \
                              'Mechanics' and no default value for \
                              implied volatility as 'Some(f64)' provided.");
                }
            },

            Mechanics::ShortCallConstHedger(asset, strike, maturity,
                                            implied_volatility) => {

                if let Some(sigma) = implied_volatility {
                    // call external function
                    short_const_hedge_call::strategy::invoke(trader, asset,
                                               *strike, *maturity, *sigma);

                }

                else {
                    println!("ERROR: 'due_diligence()' not called on \
                              'Mechanics' and no default value for \
                              implied volatility as 'Some(f64)' provided.");
                }
            },

        }
    }


    /// Does due diligence on `Mechanics` replacing all `None`
    /// with `Some(f64)`.
    #[allow(dead_code)]
    pub fn due_diligence(&mut self) {

        match self {

            // does no due diligence
            Mechanics::Lurker(_) => {
            },

            // calculates historical volatility
            Mechanics::LongCallConstHedger(asset, _, _, ref mut implied_sigma) => {

                // if 'sigma' is defined, leave it be
                if let Some(_) = implied_sigma {
                }

                // if 'None', replace with Some(_)
                else {
                    // get historical data used for due diligence
                    let fetch_db::CloseData {price: _, log_return} =
                    fetch_db::ts_close(&asset.ticker, None);

                    *implied_sigma = Some(long_const_hedge_call::
                    due_diligence::invoke(&log_return));
                }
            },

            // calculates historical volatility
            Mechanics::ShortCallConstHedger(asset, _, _, ref mut implied_sigma) => {

                // if 'sigma' is defined, leave it be
                if let Some(_) = implied_sigma {
                }

                // if 'None', replace with Some(_)
                else {
                    // get historical data used for due diligence
                    let fetch_db::CloseData {price: _, log_return} =
                    fetch_db::ts_close(&asset.ticker, None);

                    *implied_sigma = Some(short_const_hedge_call::
                    due_diligence::invoke(&log_return));
                }
            },
        }
    }
}
