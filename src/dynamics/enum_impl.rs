
use super::{fetch_db, black_scholes};


#[allow(dead_code)]
pub enum Dynamics {
    /// (ticker, mu, sigma)
    BlackScholes(String, Option<f64>, Option<f64>),

    /// (ticker, u, d)
    Binomial,
}


#[allow(dead_code)]
impl Dynamics {


    pub fn inference(&mut self) -> f64 {

        match self {

            Dynamics::BlackScholes(ticker, ref mut mu, ref mut sigma) => {

                // get historical data
                let fetch_db::CloseData {price, log_return} =
                fetch_db::ts_close(ticker, None);

                // x0 is returned later
                let x0 = *price.last().unwrap();

                // get parameter values based on historical data
                let values = black_scholes::inference::invoke(&log_return);
                let params = [mu, sigma];

                assert_eq!(values.len(), params.len(),
                           "ERROR: 'black_Scholes::inference::invoke()' \
                            returned wrong number of parameter values for \
                            this 'Dynamic' variant.");

                // iterate over parameters
                for i in 0..params.len() {
                    // if 'parameter' is defined, leave it be
                    if let Some(_) = params[i] {
                    }

                    // if 'None', replace with Some(_)
                    else {
                        *params[i] = Some(values[i]);
                    }
                }
                return x0;
            }

            // add match for other variants
            _ => {
                return 0.0;
            }
        }
    }


    pub fn dy(&self) -> f64 {

        match &self {
            Dynamics::BlackScholes(_, mu, sigma) => {
                return black_scholes::dy::invoke(mu.unwrap(),
                                                 sigma.unwrap());
            }

            // add match for other variants
            _ => {
                return 0.0;
            }
        }
    }
}
