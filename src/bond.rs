use super::ACCURACY;
use cash_flow;

/// Price for a bond with continuous compounding
/// Corresponds to the present value of its future cash flows
/// cash flow = principal + coupon
pub fn price(times: &[f64], amounts: &[f64], r: f64) -> f64 {
    cash_flow::pv(times, amounts, r)
}

/// Price for a bond with continuous compounding
/// Corresponds to the present value of its future cash flows
/// cash flow = principal + coupon
pub fn price_discrete(times: &[f64], amounts: &[f64], r: f64) -> f64 {
    cash_flow::pv_discrete(times, amounts, r)
}

/// Yield to maturity
/// internal rate of return at present time, with bond_price as today cash flow
pub fn yield_to_maturity(times: &[f64], amounts: &[f64], bond_price: f64) -> Option<f64> {

    // find range
    let (zero, one) = (0f64, 1f64);
    let mut top = one;
    while price(times, amounts, top) > bond_price {
        top * 2.0;
    }

    // this is the same as finding the internal rate of return,
    // if we include in the cash flows the price of the bond as a negative cash flow ``today'
    let mut times = times.to_vec();
    times.insert(0, zero);
    let mut amounts = amounts.to_vec();
    amounts.insert(0, bond_price);
    cash_flow::irr(&times, &amounts, &(zero..top))
}

/// Duration of the bond
/// "weighted average maturity"
/// Bond price is given from the current interest rate
pub fn duration(times: &[f64], amounts: &[f64], r: f64) -> f64 {
    let mut price = 0f64;
    let mut duration = 0f64;
    for (&t, &a) in times.iter().zip(amounts.iter()) {
        let discounted_cf = a * (-(r * t)).exp();
        price = price + discounted_cf;
        duration = duration + t * discounted_cf;
    }
    duration / price
}

/// Duration where the bond price is defined with its yield to maturity
pub fn duration_macaulay(times: &[f64], amounts: &[f64], bond_price: f64) -> Option<f64> {
    yield_to_maturity(times, amounts, bond_price)
    .map(|y| duration(times, amounts, y))
}

/// Modified duration = duration / yield
pub fn duration_modified(times: &[f64], amounts: &[f64], r: f64, bond_price: f64)
    -> Option<f64> {
    let duration = duration(times, amounts, r);
    yield_to_maturity(times, amounts, bond_price)
    .map(|y| duration / (1f64 + y))
}

/// Convexity: curvature of the approximation done when using duration
pub fn convexity(times: &[f64], amounts: &[f64], y: f64) -> f64 {
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount * t * t * (-(y * t)).exp()).sum()
}
