use math::{standard_normal_cdf, standard_normal_pdf};
use ::{MAX_ITERATIONS, HIGH_VALUE, ACCURACY};

pub struct Greeks {
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
pub fn call_price_and_greeks(s: f64, k: f64, r: f64, sigma: f64, t: f64, with_greeks: bool) -> (f64, Option<Greeks>) {
    let sqrt_t = t.sqrt();
    let sig_sqrt_t = sigma * sqrt_t;
    let d1 = ((s / k).ln() + r * t) / sig_sqrt_t + sig_sqrt_t / 2.0;
    let d2 = d1 - sig_sqrt_t;
    let cdf1 = standard_normal_cdf(d1);
    let cdf2 = standard_normal_cdf(d2);

    (s * cdf1 - k * (-r * t).exp() * cdf2,
        if with_greeks {
            let pdf1 = standard_normal_pdf(d1);
            Some(Greeks {
                    delta: cdf1,
                    gamma: pdf1 / (s * sig_sqrt_t),
                    theta: -(s * sigma * pdf1) / (2.0 * sqrt_t) - r * k * (-r * t).exp() * cdf2,
                    vega: s * sqrt_t * pdf1,
                    rho: k * t * (-r * t).exp() * cdf2
                })
        } else {
            None
        })
}

pub fn implied_volatility_call_bissection(s: f64, k: f64, r: f64, t: f64, option_price: f64) ->
Result<f64, &'static str>
    {
    let mut sigma_low = 1.0e-4; // check for arbitrage violation
    let (mut price, _) = call_price_and_greeks(s, k, r, sigma_low, t, false);

    if price > option_price {
        return Err("Arbitrage violation");
    }

    let mut sigma_high = 0.3f64;
    price = call_price_and_greeks(s, k, r, sigma_high, t, false).0;
    while price < option_price {
        sigma_high *= 2.0;
        price = call_price_and_greeks(s, k, r, sigma_high, t, false).0;
        if sigma_high > HIGH_VALUE {
            return Err("Something wrong");
        }
    }
    for _ in 0..MAX_ITERATIONS {
        let sigma = (sigma_low + sigma_high) * 0.5;
        price = call_price_and_greeks(s, k, r, sigma, t, false).0;
        let diff = price - option_price;
        if diff.abs() < ACCURACY {
            return Ok(sigma);
        }
        if diff < 0.0 {
            sigma_low = sigma;
        } else {
            sigma_high = sigma;
        }
    }
    Err("Cannot find implied volatility")
}
