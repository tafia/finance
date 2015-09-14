use math::standard_normal_cdf;

/// European option price
/// Can be exerced only at a given date
/// s: spot price
/// k: exercice price
/// r: interest rate (per period)
/// sigma: standard deviation of underlying asset
/// t: time to maturity
pub fn price_call_european(s: &f64, k: &f64, r: &f64, sigma: &f64, t: &f64) -> f64 {
    let sig_t = *sigma * t.sqrt();
    let d1 = ((*s / *k).ln() + *r * *t) / sig_t + sig_t / 2.0;
    let d2 = d1 - sig_t;
    let c = *s *
    *s
}
