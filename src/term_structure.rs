use super::FloatExt;
use std::num::Zero;

pub fn yield_from_discount_factor<F: FloatExt>(dfact: &F, t: &F) -> F {
    -(dfact.ln() / t)
}

pub fn discount_factor_from_yield<F: FloatExt>(r: &F, t: &F) -> F {
    (-r * t).exp()
}

pub fn forward_rate_from_discount_factors<F: FloatExt>(
    dfact_t: &F, dfact_T: &F, t: &F) -> F {
    (dfact_t / dfact_T).ln() / t
}

/// Forward rate from yields t < t1 < t2
/// r1: yield at t, maturity t1
/// r2: yield at t, maturity t2
pub fn forward_rate_from_yields<F: FloatExt>(
    r1: &F, r2: &F, t: &F, t1: &F, t2: &F) -> F {
    (r2 * (t2 - t) - r1 * (t1 - t)) / (t2 - t1)
}

/// Yield from linear interpolation between ordered observable times / yields
pub fn yield_linearly_interpolated<F: FloatExt>(t: &F, times: &[F], yields: &[F]) {

    if times.len() == 0 {
        return <F as Zero>::zero();
    }
    
    if times[0] > *t {
        return yields[0]
    }
    
    if times[times.len() - 1] < *t {
        return yields[times.len() - 1]
    }
    
    times.iter().enumerate().find(|(_, &ti)| *ti > t)
    .map(|(i, ti)| {
        let lambda = (ti - t) / (ti - times[i - 1]); // garanteed because times[0] <= *t
        (yields[i - 1] - yields[i]) * lambda + yields[i]
    }).expect("`times` slice must be ordered for linear interpolation")
}

