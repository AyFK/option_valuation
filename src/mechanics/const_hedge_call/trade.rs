use std::rc::Rc;

use crate::market::{asset::AssetProcess, trader::TraderProcess, ptrhash::WeakPtrHash};
use crate::maths::bsm::black_scholes_call_delta;


pub fn invoke(trader: &TraderProcess, asset: &Rc<AssetProcess>,
              strike: f64, maturity: usize, sigma: f64) {

    let broker = &trader.broker;
    let t = broker.time_idx.get();

    // get current price and interest rate
    let spot_price = broker.spot_price(asset);
    let risk_free = broker.interest();

    // calculate time to maturity
    let tau = (maturity - t) as f64;

    // calculate required hedge
    let delta = black_scholes_call_delta(spot_price, strike, sigma,
                                         risk_free, tau);
    let hedge = delta.round() as i64;

    // get current hedge
    let ticker_key = WeakPtrHash{weak_reference: Rc::downgrade(&asset)};
    let current_hedge = *trader.ownerships[broker.sim_idx.get()]
                         .borrow_mut().get(&ticker_key).unwrap_or(&0);

    // calulate modification needed for a delta hedged portfolio
    let rebalance = hedge - current_hedge;


    // define hedging conditions
    let purchase: bool = rebalance > 0;
    let sell: bool = rebalance < 0;


    // trade on hedge condition
    if purchase {
        broker.buy_order(trader, asset, 1);
    }

    if sell {
        broker.sell_order(trader, asset, 1);
    }

}
