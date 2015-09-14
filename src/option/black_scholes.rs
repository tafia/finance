use math::{standard_normal_cdf, standard_normal_pdf};

pub struct Instrument {
    pub price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

/// European option price
/// Can be exerced only at a given date
/// s: spot price
/// k: exercice price
/// r: interest rate (per period)
/// sigma: standard deviation of underlying asset
/// t: time to maturity
pub fn call(s: f64, k: f64, r: f64, sigma: f64, t: f64) -> Instrument {
    let sqrt_t = t.sqrt();
    let sig_sqrt_t = sigma * sqrt_t;
    let d1 = ((s / k).ln() + r * t) / sig_sqrt_t + sig_sqrt_t / 2.0;
    let d2 = d1 - sig_sqrt_t;
    let cdf1 = standard_normal_cdf(d1);
    let cdf2 = standard_normal_cdf(d2);
    let pdf1 = standard_normal_pdf(d1);

    Instrument {
        price: s * cdf1 - k * (-r * t).exp() * cdf2,
        delta: cdf1,
        gamma: pdf1 / (s * sig_sqrt_t),
        theta: -(s * sigma * pdf1) / (2.0 * sqrt_t) - r * k * (-r * t).exp() * cdf2,
        vega: s * sqrt_t * pdf1,
        rho: k * t * (-r * t).exp() * cdf2
    }
}
