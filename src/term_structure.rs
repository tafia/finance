use super::FloatExt;
use std::num::Zero;

pub fn yield_from_discount_factor<F: FloatExt>(dfact: &F, t: &F) -> F {
    -((*dfact).ln() / *t)
}

pub fn discount_factor_from_yield<F: FloatExt>(r: &F, t: &F) -> F {
    (-*r * *t).exp()
}

pub fn forward_rate_from_discount_factors<F: FloatExt>(
    dfact_t1: &F, dfact_t2: &F, t: &F) -> F {
    (*dfact_t1 / *dfact_t2).ln() / *t
}

/// Forward rate from yields t < t1 < t2
/// r1: yield at t, maturity t1
/// r2: yield at t, maturity t2
pub fn forward_rate_from_yields<F: FloatExt>(
    r1: &F, r2: &F, t: &F, t1: &F, t2: &F) -> F {
    (*r2 * (*t2 - *t) - *r1 * (*t1 - *t)) / (*t2 - *t1)
}

/// Yield from linear interpolation between ordered observable times / yields
pub fn yield_linearly_interpolated<F: FloatExt>(t: &F, times: &[F], yields: &[F]) -> F {

    if times.len() == 0 {
        return (<F as Zero>::zero());
    }

    if times[0] > *t {
        return yields[0]
    }

    if times[times.len() - 1] < *t {
        return yields[times.len() - 1]
    }

    times.iter().enumerate().find(|&(_, ti)| *ti > *t)
    .map(|(i, ti)| {
        let lambda = (*ti - *t) / (*ti - times[i - 1]); // garanteed because times[0] <= *t
        (yields[i - 1] - yields[i]) * lambda + yields[i]
    }).expect("`times` slice must be ordered for linear interpolation")
}

/// Base trait to define a term structure
/// Default fn implementations are cyclic
trait TermStructure {
    type FloatType: FloatExt;
    fn yield_(&self, t: &Self::FloatType) -> Self::FloatType {
        yield_from_discount_factor(&self.discount_factor(t), t)
    }
    fn discount_factor(&self, t: &Self::FloatType) -> Self::FloatType {
        discount_factor_from_yield(&self.yield_(t), t)
    }
    fn forward_rate(&self, t1: &Self::FloatType, t2: &Self::FloatType) -> Self::FloatType {
        let (d1, d2) = (self.discount_factor(t1), self.discount_factor(t2));
        forward_rate_from_discount_factors(&d1, &d2, &<Self::FloatType as Zero>::zero())
    }
}

struct TermStructureFlat<F> {
    r: F
}

impl<F> TermStructureFlat<F> {
    fn new(r: F) -> TermStructureFlat<F> {
        TermStructureFlat {
            r: r
        }
    }
}

impl<F: FloatExt> TermStructure for TermStructureFlat<F> {
    type FloatType = F;
    fn yield_(&self, t: &F) -> F {
        let zero = <F as Zero>::zero();
        if self.r >= zero {
            return self.r;
        }
        zero
    }
}
