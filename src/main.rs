use std::rc::Rc;

mod market;
use market::asset::*;
use market::trader::*;
use market::brokerage::*;

fn main() {
    println!("Hello, world!");



    let spx = Rc::new(AssetProcess::new(Dynamics::BlackScholes,
                      String::from("spx"), 10, 10));

    let trader1 = Rc::new(TraderProcess::new(String::from("Al"), 100.0, 10, 10));
    let trader2 = Rc::new(TraderProcess::new(String::from("Bob"), 100.0, 10, 10));


    let o1: EuropeanOption = EuropeanOption::Put(Rc::clone(&spx), 100.0, 10,
                                                 Rc::clone(&trader1),
                                                 Rc::clone(&trader2));

    let t: usize = 10;
    let spot: f64 = 43.0;
    o1.pay_off(spot, t);            // broker : Vec<EuropeanOption> = [...]
                                    // let options = vec![o1];

    println!("{} {}", trader1.balance.get(), trader2.balance.get());

}

