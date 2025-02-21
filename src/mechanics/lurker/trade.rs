use std::rc::Rc;

use crate::market::{asset::AssetProcess, trader::TraderProcess};


pub fn invoke(trader: &TraderProcess, asset: &Rc<AssetProcess>) {

    let broker = &trader.broker;
    let t = broker.time_idx.get();

    let purchase = t == 0;
    let sell = t == broker.simulation_length;

    /*
    trader: &Rc<TraderProcess>, asset: &Rc<AssetProcess>,
             broker: &Rc<Broker>) {
    */

    if purchase {
        broker.buy_order(trader, asset, 1);
    }

    if sell {
        broker.sell_order(trader, asset, 1);
    }
}
