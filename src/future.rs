use super::FloatExt;

/// Price of future
/// s: current price of asset
/// r: risk-free interest rate
/// t: current time
/// mat: maturity
pub fn price<F: FloatExt>(s: &F, r: &F, t: &F, mat: &F) -> F {
    (*r * (*mat - *t)).exp() * *s
}
