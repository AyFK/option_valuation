use std::rc::Rc;

use crate::market;
use crate::market::{asset::AssetProcess, trader::TraderProcess, ptrhash::WeakPtrHash};
use crate::maths::bsm::black_scholes_call_delta;

// move this to its own file
use crate::market::brokerage::European;

pub fn invoke(trader: &Rc<TraderProcess>, asset: &Rc<AssetProcess>,
              strike: f64, maturity: usize, sigma: f64) {

    let broker = &trader.broker;
    let t = broker.time_idx.get();

    // get current price and interest rate
    let spot_price = broker.spot_price(asset);
    let risk_free = broker.interest();

    // calculate time to maturity
    let tau = (maturity as isize) - (t as isize);


    // if we have not reached maturity, hedge the derivitive
    if tau > 0 {
        // calculate required hedge
        let delta = black_scholes_call_delta(spot_price, strike, sigma,
                                             risk_free, tau as f64);

        let hedge = (delta * 100.0).round() as i64;

        // get current hedge
        let ticker_key = WeakPtrHash{weak_reference: Rc::downgrade(&asset)};
        let current_hedge = *trader.ownerships[broker.sim_idx.get()]
                             .borrow_mut().get(&ticker_key).unwrap_or(&0);

        // calulate modification needed for a delta hedged portfolio
        let rebalance = hedge - current_hedge;

        // define hedging conditions
        let purchase: bool = rebalance > 0 && tau > 0;
        let sell: bool = rebalance < 0 && tau > 0;

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

        if hedge > 0 {
            broker.sell_order(trader, asset, hedge);
        }
    }
}
