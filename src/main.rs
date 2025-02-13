use std::rc::Rc;
use std::io::{self, Write};

mod market;
#[allow(unused_imports)]
use market::asset::*;
use market::trader::*;
use market::brokerage::*;



#[allow(dead_code)]
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}


#[allow(unused_variables)]
fn main() {


    // broker is responsible for the simulation
    let broker = Rc::new(Broker::open(10));

    AssetProcess::new(Rc::clone(&broker), Dynamics::BlackScholes,
                      String::from("spx"), 10, 10);

    TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                       String::from("Bob"), 100.0, 10, 10);

    TraderProcess::new(Rc::clone(&broker), Action::Lurker,
                       String::from("Noa"), 100.0, 10, 10);


    {
        let broker_traders = broker.all_traders.borrow();
        let n_traders = broker_traders.len();
        println!("\nbroker members:");
        for i in 0..n_traders {
            let name: &str = &(broker_traders[i].name);
            println!("name = {}", name);
        }

        let broker_assets = broker.all_assets.borrow();
        let n_assets = broker_assets.len();
        println!("\nbroker assets:");
        for i in 0..n_assets {
            let ticker: &str = &(broker_assets[i].ticker);
            println!("name = {}", ticker);
        }


        let mut pfprocess = broker_traders[0].portfolio_process.borrow_mut();
        pfprocess[0][1] = 1.0;

        println!("this: {} vs that: {}", pfprocess[0][0], pfprocess[0][1]);


        println!("bal: {}", broker_traders[0].balance.get());
        broker_traders[0].balance.set(broker_traders[0].balance.get()
                                      - 200.0);
        println!("bal: {}", broker_traders[0].balance.get());
    }


    broker.update();
    println!("bal: {}", broker.all_traders.borrow_mut()[0].balance.get());
    println!("bal: {}", broker.all_traders.borrow_mut()[1].balance.get());


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

    let _ = input("\n[Enter]");
}

