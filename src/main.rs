use std::rc::Rc;

mod market;
use market::asset::*;
use market::trader::*;
use market::brokerage::*;

fn main() {
    println!("Hello, world!");


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

    TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                       String::from("Bob"),100.0, 10, 10);




    // TODO: add these into 'new' method instead, cant find a way to
    /*
    println!("rc: {}", Rc::strong_count(&broker));
    spx.join(Rc::clone(&broker));
    println!("rc: {}", Rc::strong_count(&broker));
    trader1.join(Rc::clone(&broker));
    trader2.join(Rc::clone(&broker));
    println!("rc: {}", Rc::strong_count(&broker));
    */


    // w.t.f.
    //trader1.clone().join(broker.clone());

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

