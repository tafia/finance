const A1: f64 = 0.254829592;
const A2: f64 = -0.284496736;
const A3: f64 = 1.421413741;
const A4: f64 = -1.453152027;
const A5: f64 = 1.061405429;
const P: f64 = 0.3275911;

/// Normal  cumulative density function
pub fn standard_normal_cdf(mut x: f64) -> f64 {
    let sign = x.signum();
    x = x.abs() / 2f64.sqrt();

    let t = 1.0 / (1.0 + P * x);
    let y = 1.0 - (((((A5 * t + A4) * t) + A3) * t + A2) * t + A1) * t * (-x*x).exp();

    0.5*(1.0 + sign * y)
}
