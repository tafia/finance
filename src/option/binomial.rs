use option::Call;

impl Call {
    /// European option price
    /// Can be exerced only at a given date
    /// s: spot price
    /// k: exercice price
    /// r: interest rate (per period)
    /// u: up movement
    /// d: down movement
    /// n: nb periods
    pub fn bin_price<F: Fn(f64) -> usize>(&self, u: f64, d: f64, time_to_period: F) -> f64 {
        let n = time_to_period(self.t);
        let r_exp = self.r.exp();
        let uu = u * u;
        let p_up = (r_exp - d) / (u - d);
        let p_down = 1f64 - p_up;
        let n = n + 1;
        let mut prices = Vec::with_capacity(n);
        let mut call_values = Vec::with_capacity(n);
        prices[0] = self.s * d.powi(n as i32);
        call_values[0] = 0f64.max(prices[0] - self.k);
        for i in 1..n {
            prices[i] = prices[i - 1] * uu;
            call_values[i] = 0f64.max(prices[i] - self.k);
        }
        for step in (0..n - 1).rev() {
            for i in 0..(step + 1) {
                call_values[i] = (p_up * call_values[i - 1] + p_down * call_values[i]) / r_exp;
            }
        }
        call_values[0]
    }
}
