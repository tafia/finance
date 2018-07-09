mod binomial;
mod black_scholes;

pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

#[derive(Debug, Clone)]
pub struct Call {
    /// Asset price
    pub s: f64,
    /// Asset implied volatility
    pub vol: f64,
    /// Time to maturity
    pub t: f64,
    /// strike
    pub k: f64,
    /// Risk free rate
    pub r: f64,
}
