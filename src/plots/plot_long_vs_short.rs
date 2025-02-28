use std::rc::Rc;

use super::{performance_histogram, performance_plot};

use crate::market::{asset::AssetProcess, trader::TraderProcess};

pub fn invoke(asset: &Rc<AssetProcess>, long: &Rc<TraderProcess>,
              short: &Rc<TraderProcess>) {

    performance_histogram::figure(&long.performance);
    performance_histogram::figure(&short.performance);


    let rnd_idx = 999;

    performance_plot::figure(&long.portfolio_processes[rnd_idx],
                             &asset.price_processes[rnd_idx],
                             &asset.return_processes[rnd_idx]);

    performance_plot::figure(&short.portfolio_processes[rnd_idx],
                             &asset.price_processes[rnd_idx],
                             &asset.return_processes[rnd_idx]);
}
