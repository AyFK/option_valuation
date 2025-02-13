use std::rc::Rc;

mod market;
#[allow(unused_imports)]
use market::asset::*;
use market::trader::*;
use market::brokerage::*;

fn main() {


    let broker = Rc::new(Broker::open(10));


    /*
    Rc::new(AssetProcess::new(Rc::clone(&broker), Dynamics::BlackScholes,
                              String::from("spx"), 10, 10))
                              .join(Rc::clone(&broker));
    */


    /*
    Rc::new(TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                               String::from("Al"),100.0, 10, 10))
                               .join(Rc::clone(&broker));


    Rc::new(TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                               String::from("Bob"),100.0, 10, 10))
                               .join(Rc::clone(&broker));
    */

    AssetProcess::new(Rc::clone(&broker), Dynamics::BlackScholes,
                      String::from("spx"), 10, 10);

    TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                       String::from("Bob"), 100.0, 10, 10);

    TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                       String::from("Noa"), 100.0, 10, 10);


    let broker_traders = broker.all_traders.borrow_mut();
    let n_traders = broker_traders.len();
    println!("\nbroker members:");
    for i in 0..n_traders {
        let name: &str = &(broker_traders[i].name);
        println!("name = {}", name);

    }

    let broker_assets = broker.all_assets.borrow_mut();
    let n_assets = broker_assets.len();
    println!("\nbroker assets:");
    for i in 0..n_assets {
        let ticker: &str = &(broker_assets[i].ticker);
        println!("name = {}", ticker);

    }



    /*
    let o1: EuropeanOption = EuropeanOption::Put(Rc::clone(&spx), 100.0, 10,
                                                 Rc::clone(&trader1),
                                                 Rc::clone(&trader2));

    let t: usize = 10;
    let spot: f64 = 43.0;
    o1.pay_off(spot, t);            // broker : Vec<EuropeanOption> = [...]
                                    // let options = vec![o1];

    println!("{} {}", trader1.balance.get(), trader2.balance.get());
    */


    // ideally:
    // broker.runExchange()
}

