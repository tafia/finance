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
pub fn call_price(s: f64, k: f64, r: f64, sigma: f64, t: f64, greeks: &mut Option<Greeks>) -> f64 {
    let sqrt_t = t.sqrt();
    let sig_sqrt_t = sigma * sqrt_t;
    let d1 = ((s / k).ln() + r * t) / sig_sqrt_t + sig_sqrt_t / 2.0;
    let d2 = d1 - sig_sqrt_t;
    let cdf1 = standard_normal_cdf(d1);
    let cdf2 = standard_normal_cdf(d2);

    if let Some(ref mut greeks) = *greeks {
        let pdf1 = standard_normal_pdf(d1);
        *greeks = Greeks {
            delta: cdf1,
            gamma: pdf1 / (s * sig_sqrt_t),
            theta: -(s * sigma * pdf1) / (2.0 * sqrt_t) - r * k * (-r * t).exp() * cdf2,
            vega: s * sqrt_t * pdf1,
            rho: k * t * (-r * t).exp() * cdf2
        };
    }
    s * cdf1 - k * (-r * t).exp() * cdf2
}

pub fn implied_volatility_call_bissection(s: f64, k: f64, r: f64, t: f64, option_price: f64) ->
    Result<f64, &'static str>
{
    let mut sigma_low = 1.0e-5; // check for arbitrage violation
    let mut no_greek = None;
    let mut price = call_price(s, k, r, sigma_low, t, &mut no_greek);

    if price > option_price {
        return Ok(0.0);
    }

    let mut sigma_high = 0.3f64;
    price = call_price(s, k, r, sigma_high, t, &mut no_greek);
    while price < option_price {
        sigma_high *= 2.0;
        price = call_price(s, k, r, sigma_high, t, &mut no_greek);
        if sigma_high > HIGH_VALUE {
            return Err("Something wrong");
        }
    }
    for _ in 0..MAX_ITERATIONS {
        let sigma = (sigma_low + sigma_high) * 0.5;
        price = call_price(s, k, r, sigma, t, &mut no_greek);
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

pub fn implied_volatility_call_newton(s: f64, k: f64, r: f64, t: f64, option_price: f64) ->
    Result<f64, &'static str>
{
    let mut sigma_low = 1.0e-5; // check for arbitrage violation
    let mut no_greek = None;
    let mut price = call_price(s, k, r, sigma_low, t, &mut no_greek);

    if price > option_price {
        return Ok(0.0);
    }

    let sqrt_t = t.sqrt();
    let mut sigma = (option_price / s) / (0.398 * sqrt_t);
    for _ in 0..MAX_ITERATIONS {
        price = call_price(s, k, r, sigma, t, &mut no_greek);
        let diff = option_price - price;
        if diff.abs() < ACCURACY {
            return Ok(sigma);
        }
        let d1 = ((s / k).ln() + sqrt_t) / (sigma * sqrt_t) + 0.5 * sigma * sqrt_t;
        let vega = s * sqrt_t * standard_normal_pdf(d1);
        sigma += diff / vega;
    }
    Err("Cannot find implied volatility")
}
