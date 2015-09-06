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
pub trait TermStructure {
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

pub struct TermStructureFlat<F> {
    r: F
}

impl<F> TermStructureFlat<F> {
    pub fn new(r: F) -> TermStructureFlat<F> {
        TermStructureFlat {
            r: r
        }
    }
}

impl<F: FloatExt> TermStructure for TermStructureFlat<F> {
    type FloatType = F;
    fn yield_(&self, t: &F) -> F {
        let zero = <F as Zero>::zero();
        if *t >= zero {
            return self.r;
        }
        zero
    }
}

#[derive(PartialEq, Eq)]
pub struct TermStructureInterpolated<F> {
    times: Vec<F>,
    yields: Vec<F>
}

impl<F> TermStructureInterpolated<F> {
    pub fn new(times: Vec<F>, yields: Vec<F>) -> TermStructureInterpolated<F> {
        TermStructureInterpolated {
            times: times,
            yields: yields
        }
    }
}

impl<F: FloatExt> TermStructure for TermStructureInterpolated<F> {
    type FloatType = F;
    fn yield_(&self, t: &F) -> F {
        yield_linearly_interpolated(t, &self.times, &self.yields)
    }
}

pub fn bonds_price<F, T>(times: &[F], cash_flows: &[F], d: &T) -> F
        where F: FloatExt, T: TermStructure<FloatType=F> {
    times.iter().zip(cash_flows.iter()).map(|(t, &cf)| cf * d.discount_factor(t)).sum()
}

pub fn bonds_duration<F, T>(times: &[F], cash_flows: &[F], d: &T) -> F
        where F: FloatExt, T: TermStructure<FloatType=F> {
    let mut sum = <F as Zero>::zero();
    let mut duration = <F as Zero>::zero();
    for (t, &cf) in times.iter().zip(cash_flows.iter()) {
        let discounted_cf = cf * d.discount_factor(t);
        sum = sum + discounted_cf;
        duration = duration + *t * discounted_cf;
    }
    duration / sum
}
