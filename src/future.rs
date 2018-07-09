/// Price of future
/// s: current price of asset
/// r: risk-free interest rate
/// t: current time
/// mat: maturity
pub fn price(s: f64, r: f64, t: f64, mat: f64) -> f64 {
    (r * (mat - t)).exp() * s
}
