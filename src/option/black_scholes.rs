use FloatExt;
use rand::distributions::normal::Normal;

/// European option price
/// Can be exerced only at a given date
/// s: spot price
/// k: exercice price
/// r: interest rate (per period)
/// sigma: standard deviation of underlying asset
/// t: time to maturity
pub fn price_call_european<F: FloatExt>(s: &F, k: &F, r: &F, sigma: &F, t: &F) -> F {
    let sqrt_t = t.sqrt();
    let d1 = ((*s / *k).ln() + *r * *t) / (*sigma * sqrt_t) + (*sigma * sqrt_t).half();
    let d2 = d1 - *sigma * sqrt_t;
    *s
}
