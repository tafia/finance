use std::num::{Zero, One};
use std::mem;
use std::fmt::Debug;
use std::ops::Range;
use super::FloatExt;

/// Calculates the present value of future cash flows with discrete compounding
pub fn cash_flow_pv_discrete<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    let acc = <F as One>::one() + *r;
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount / acc.powf(t)).sum()
}

/// Calculates the present value of future cash flows with continuous compounded interest
pub fn cash_flow_pv<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount * (-(*r * t)).exp()).sum()
}

/// Calculates the internal rate of return
/// Initial bucket is set to F::zero()..F::one()
pub fn cash_flow_irr<F: FloatExt>(times: &[F], amounts: &[F],
    accuracy: &F, max_iteration: usize, bucket: &Range<F>) -> Option<F> {

    let (mut x1, mut x2) = (bucket.start, bucket.end);
    let (mut f1, mut f2) = (cash_flow_pv(times, amounts, &x1), cash_flow_pv(times, amounts, &x2));

    let zero = <F as Zero>::zero();
    if f1 * f2 > zero {
        return None;
    }

    // sort x1/x2 f1/f2
    if f1 > zero {
        mem::swap(&mut x1, &mut x2);
        mem::swap(&mut f1, &mut f2)
    }

    let (mut rtb, mut dx) = (x1, x2 - x1);
    for _ in 0..max_iteration {
        dx.half();
        let x_mid = rtb + dx;
        let f_mid = cash_flow_pv(times, amounts, &x_mid);
        if f_mid.abs() < *accuracy || dx.abs() < *accuracy {
            return Some(x_mid)
        }
        if f_mid < zero {
            rtb = x_mid;
        }
    }
    None
}

#[test]
fn test_cash_flow_pv_discrete() {
    let a = cash_flow_pv_discrete(&[0f64, 1.0, 2.0], &[1f64, 1.0, 1.0], &1f64);
    assert!(a - 1.75 < 1e-10);
}

#[test]
fn test_cash_flow_pv() {
    let a = cash_flow_pv(&[0f64, 1.0, 2.0], &[1f64, 1.0, 1.0], &1f64);
    assert!(a - 1.50321472440 < 1e-10);
}

#[test]
fn test_cash_flow_irr() {
    let a = cash_flow_irr(&[1.0, 2.0, 3.0], &[-2f64, 1.0, 1.0], &1.0e-5, 50, &(0.0f64..1.0));
    assert_eq!(a, Some(0.9999923706054688));
}
