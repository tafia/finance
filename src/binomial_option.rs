use super::FloatExt;
use std::num::{Zero, One};

fn max<'a, F: FloatExt>(a: &'a F, b: &'a F) -> &'a F {
    if *a > *b {
        a
    } else {
        b
    }
}

/// European option price
/// Can be exerced only at a given date
/// s: spot price
/// k: exercice price
/// r: interest rate (per period)
/// u: up movement
/// d: down movement
/// n: nb periods
pub fn price_call_european<F: FloatExt>(s: &F, k: &F, r: &F, u: &F, d: &F, n: usize) -> F {
    let r_exp = (*r).exp();
    let uu = *u * *u;
    let p_up = (r_exp - *d) / (*u - *d);
    let p_down = <F as One>::one() - p_up;
    let n = n + 1;
    let mut prices = Vec::with_capacity(n);
    prices[0] = *s * (*d).powi(n as i32);
    for i in 1..n {
        prices[i] = prices[i - 1] * uu;
    }
    let zero = <F as Zero>::zero();
    let mut call_values = Vec::<F>::with_capacity(n);
    for (&p, v) in prices.iter().zip(call_values.iter_mut()) {
        *v = *max(&zero, &(p - *k));
    }
    for step in (0..n-1).rev() {
        for i in 0..(step + 1) {
            call_values[i] = (p_up * call_values[i - 1] + p_down * call_values[i])/r_exp;
        }
    }
    call_values[0]
}
