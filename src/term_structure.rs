pub fn yield_from_discount_factor(dfact: f64, t: f64) -> f64 {
    -(dfact.ln() / t)
}

pub fn discount_factor_from_yield(r: f64, t: f64) -> f64 {
    (-r * t).exp()
}

pub fn forward_rate_from_discount_factors(dfact_t1: f64, dfact_t2: f64, t: f64) -> f64 {
    (dfact_t1 / dfact_t2).ln() / t
}

/// f64orward rate from yields t < t1 < t2
/// r1: yield at t, maturity t1
/// r2: yield at t, maturity t2
pub fn forward_rate_from_yields(r1: f64, r2: f64, t: f64, t1: f64, t2: f64) -> f64 {
    (r2 * (t2 - t) - r1 * (t1 - t)) / (t2 - t1)
}

/// Yield from linear interpolation between ordered observable times / yields
pub fn yield_linearly_interpolated(t: f64, times: &[f64], yields: &[f64]) -> f64 {

    if times.len() == 0 {
        return 0.0;
    }

    if times[0] > t {
        return yields[0]
    }

    if times[times.len() - 1] < t {
        return yields[times.len() - 1]
    }

    times.iter().enumerate().find(|&(_, ti)| *ti > t)
    .map(|(i, ti)| {
        let lambda = (*ti - t) / (*ti - times[i - 1]); // garanteed because times[0] <= *t
        (yields[i - 1] - yields[i]) * lambda + yields[i]
    }).expect("`times` slice must be ordered for linear interpolation")
}

/// Base trait to define a term structure
/// Default fn implementations are cyclic
pub trait TermStructure {
    fn yield_(&self, t: f64) -> f64 {
        yield_from_discount_factor(self.discount_factor(t), t)
    }
    fn discount_factor(&self, t: f64) -> f64 {
        discount_factor_from_yield(self.yield_(t), t)
    }
    fn forward_rate(&self, t1: f64, t2: f64) -> f64 {
        let (d1, d2) = (self.discount_factor(t1), self.discount_factor(t2));
        forward_rate_from_discount_factors(d1, d2, 0.0)
    }
}

pub struct TermStructureFlat {
    r: f64
}

impl TermStructure for TermStructureFlat {
    fn yield_(&self, t: f64) -> f64 {
        self.r.max(0.0)
    }
}

pub struct TermStructureInterpolated {
    times: Vec<f64>,
    yields: Vec<f64>
}

impl TermStructureInterpolated {
    pub fn new(times: Vec<f64>, yields: Vec<f64>) -> TermStructureInterpolated {
        TermStructureInterpolated {
            times: times,
            yields: yields
        }
    }
}

impl TermStructure for TermStructureInterpolated {
    fn yield_(&self, t: f64) -> f64 {
        yield_linearly_interpolated(t, &self.times, &self.yields)
    }
}

pub fn bonds_price<T: TermStructure>(times: &[f64], cash_flows: &[f64], d: &T) -> f64 {
    times.iter().zip(cash_flows.iter()).map(|(&t, &cf)| cf * d.discount_factor(t)).sum()
}

pub fn bonds_duration<T: TermStructure>(times: &[f64], cash_flows: &[f64], d: &T) -> f64 {
    let mut sum = 0.0;
    let mut duration = 0.0;
    for (&t, &cf) in times.iter().zip(cash_flows.iter()) {
        let discounted_cf = cf * d.discount_factor(t);
        sum = sum + discounted_cf;
        duration = duration + t * discounted_cf;
    }
    duration / sum
}
