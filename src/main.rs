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
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}


#[allow(unused_variables)]
fn main() {

    let simulations_total = 10;
    let simulation_length = 10;

    // broker is responsible for the simulation
    let broker = Rc::new(Broker::new(simulations_total, simulation_length));

    AssetProcess::new(Rc::clone(&broker), Dynamics::BlackScholes,
                      String::from("spx"));

    TraderProcess::new(Rc::clone(&broker), Mechanics::Lurker,
                       String::from("Bob"), 100.0);

    TraderProcess::new(Rc::clone(&broker), Mechanics::Lurker,
                       String::from("Noa"), 100.0);


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


        broker_traders[0].portfolio_processes[0][1].set(1.0);

        println!("this: {} vs that: {}", broker_traders[0].
                 portfolio_processes[0][0].get(), broker_traders[0].
                 portfolio_processes[0][1].get());


        println!("bal: {}", broker_traders[0].balances[0].get());
        broker_traders[0].balances[0].set(broker_traders[0].balances[0].get()
                                      - 200.0);
        println!("bal: {}", broker_traders[0].balances[0].get());
    }


    println!("bal: {}", broker.all_traders.borrow_mut()[0].balances[0].get());
    println!("bal: {}", broker.all_traders.borrow_mut()[1].balances[0].get());

    broker.open();

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

