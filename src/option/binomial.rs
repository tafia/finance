/// European option price
/// Can be exerced only at a given date
/// s: spot price
/// k: exercice price
/// r: interest rate (per period)
/// u: up movement
/// d: down movement
/// n: nb periods
pub fn price_call_european(s: f64, k: f64, r: f64, u: f64, d: f64, n: usize) -> f64 {
    let r_exp = r.exp();
    let uu = u * u;
    let p_up = (r_exp - d) / (u - d);
    let p_down = 1f64 - p_up;
    let n = n + 1;
    let mut prices = Vec::with_capacity(n);
    prices[0] = s * d.powi(n as i32);
    for i in 1..n {
        prices[i] = prices[i - 1] * uu;
    }
    let mut call_values = Vec::<f64>::with_capacity(n);
    for (&p, v) in prices.iter().zip(call_values.iter_mut()) {
        *v = 0f64.max((p - k));
    }
    for step in (0..n - 1).rev() {
        for i in 0..(step + 1) {
            call_values[i] = (p_up * call_values[i - 1] + p_down * call_values[i]) / r_exp;
        }
    }
    call_values[0]
}
