use std::rc::Rc;

use crate::market::{asset::AssetProcess, trader::TraderProcess};
use crate::datastructs::ptrhash::WeakPtrHash;
use crate::maths::bsm::black_scholes_call_delta;

use crate::derivatives::european::European;


pub fn invoke(trader: &Rc<TraderProcess>, asset: &Rc<AssetProcess>,
              strike: f64, maturity: usize, sigma: f64) {

    let broker = &trader.broker;
    let t = broker.time_idx.get();

    // get current price and interest rate
    let spot_price = broker.spot_price(asset);
    let risk_free = broker.interest();

    // calculate time to maturity
    let tau = (maturity as isize) - (t as isize);


    // when the option is alive, hedge it
    if tau > 0 {

        // calculate required hedge
        let delta = black_scholes_call_delta(spot_price, strike, sigma,
                                             risk_free, tau as f64);

        // round and convert to integer (no fractional shares)
        let hedge = (delta * 100.0).round() as i64;

        // get portfolio hedge
        let ticker_key = WeakPtrHash{weak_reference: Rc::downgrade(&asset)};
        let current_hedge = *trader.ownerships[broker.sim_idx.get()]
                             .borrow_mut().get(&ticker_key).unwrap_or(&0);

        // calulate rebalance needed for a delta neutral position
        let rebalance = hedge - current_hedge;

        // define hedging conditions
        let purchase: bool = rebalance > 0;
        let sell: bool = rebalance < 0;

        // trade on hedge condition
        if purchase {
            broker.buy_order(trader, asset, rebalance);
        }

        if sell {
            broker.sell_order(trader, asset, -rebalance);
        }
    }



    // if we are at t0, write an option
    if t == 0 {
        broker.write_eu_option_on_autofill(European::Call, &asset,
                                        strike, maturity, trader);
    }


    // at maturity sell your hedge to cover pay-off
    if t == maturity {

        // get current hedge
        let ticker_key = WeakPtrHash{weak_reference: Rc::downgrade(&asset)};
        let hedge = *trader.ownerships[broker.sim_idx.get()]
                     .borrow_mut().get(&ticker_key).unwrap_or(&0);

        // a call hedge can only be positive, get rid of it
        if hedge > 0 {
            broker.sell_order(trader, asset, hedge);
        }
    }
}
