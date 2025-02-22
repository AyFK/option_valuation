
use super::{fetch_db, black_scholes};


#[allow(dead_code)]
pub enum Dynamics {
    /// An obeject to represent the Black Scholes market
    /// assumption of constant 'μ' (trend) and 'σ' (volatility).
    BlackScholes(Option<f64>, Option<f64>),

    /// An obeject to represent the binomial market
    /// assumption of constant 'u' (ups) and 'd' (downs).
    Binomial(Option<f64>, Option<f64>),
}


#[allow(dead_code)]
impl Dynamics {

    /// Infer parameter values on 'Dynamics' type and return
    /// starting value 'x0'.
    pub fn inference(&mut self, ticker: &str) -> f64 {

        // get historical data
        let fetch_db::CloseData {price, log_return} =
                                fetch_db::ts_close(ticker, None);

        // need to return starting value
        let x0 = *price.last().unwrap();

        // match different inference methods for different 'Dynamics'
        // variants
        match self {

            Dynamics::BlackScholes(ref mut mu, ref mut sigma) => {

                // get parameter values based on historical data
                let results = black_scholes::inference::invoke(&log_return);
                let params = [mu, sigma];

                assert_eq!(results.len(), params.len(),
                           "ERROR: 'black_Scholes::inference::invoke()' \
                            returned an incorrect number of parameter \
                            values for this 'Dynamics' variant.");

                // assign inference results to corresponding parameter
                // values
                for i in 0..params.len() {
                    // if 'params[i]' is pre-defined, leave it be
                    if let Some(_) = params[i] {
                    }

                    // if 'None', replace with 'results[i]'
                    else {
                        *params[i] = Some(results[i]);
                    }
                }
            }


            // add match for other variants
            _ => {
            }
        }

        return x0;
    }


    /// Get incremental log-return results such that:
    /// S(t+1) = S(t) e^{dy}
    pub fn dy(&self) -> f64 {

        match &self {
            Dynamics::BlackScholes(mu, sigma) => {
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
