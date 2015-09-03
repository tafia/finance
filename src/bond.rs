use super::FloatExt;
use std::num::{Zero, One};
use cash_flow;

/// Price for a bond with continuous compounding
/// Corresponds to the present value of its future cash flows
/// cash flow = principal + coupon
pub fn price<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    cash_flow::pv(times, amounts, r)
}

/// Price for a bond with continuous compounding
/// Corresponds to the present value of its future cash flows
/// cash flow = principal + coupon
pub fn price_discrete<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    cash_flow::pv_discrete(times, amounts, r)
}

/// Yield to maturity
/// internal rate of return at present time, with bond_price as today cash flow
pub fn yield_to_maturity<F: FloatExt>(times: &[F], amounts: &[F], bond_price: &F) -> Option<F> {

    // find range
    let (zero, one) = (<F as Zero>::zero(), <F as One>::one());
    let mut top = one;
    while price(times, amounts, &top) > *bond_price {
        top.double();
    }

    // this is the same as finding the internal rate of return,
    // if we include in the cash flows the price of the bond as a negative cash flow ``today'
    let mut times = times.to_vec();
    times.insert(0, zero);
    let mut amounts = amounts.to_vec();
    amounts.insert(0, *bond_price);
    cash_flow::irr(&times, &amounts, &F::default_accuracy(), 200, &(zero..top))
}

/// Duration of the bond
/// "weighted average maturity"
/// Bond price is given from the current interest rate
pub fn duration<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    let mut price = <F as Zero>::zero();
    let mut duration = <F as Zero>::zero();
    for (&t, &a) in times.iter().zip(amounts.iter()) {
        let discounted_cf = a * (-(*r * t)).exp();
        price = price + discounted_cf;
        duration = duration + t * discounted_cf;
    }
    duration / price
}

/// Duration where the bond price is defined with its yield to maturity
pub fn duration_macaulay<F: FloatExt>(times: &[F], amounts: &[F], bond_price: &F) -> Option<F> {
    yield_to_maturity(times, amounts, bond_price)
    .map(|y| duration(times, amounts, &y))
}

/// Modified duration = duration / yield
pub fn duration_modified<F: FloatExt>(times: &[F], amounts: &[F], r: &F, bond_price: &F)
    -> Option<F> {
    let duration = duration(times, amounts, r);
    yield_to_maturity(times, amounts, bond_price)
    .map(|y| duration / (<F as One>::one() + y))
}

/// Convexity: curvature of the approximation done when using duration
pub fn convexity<F: FloatExt>(times: &[F], amounts: &[F], y: &F) -> F {
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount * t * t * (-(*y * t)).exp()).sum()
}
