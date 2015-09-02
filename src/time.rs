use num::traits::Float;
use std::num::{Zero, One};
use std::ops::{Add, Sub, Mul};
use std::mem;
use std::fmt::Debug;

pub trait Half {
    fn half(&mut self);
}

impl Half for f64 {
    fn half(&mut self) {
        *self *= 0.5;
    }
}

impl Half for f32 {
    fn half(&mut self) {
        *self *= 0.5;
    }
}

/// Calculates the present value of future cash flows with discrete compounding
pub fn cash_flow_pv_discrete<F>(times: &[F], amounts: &[F], r: &F) -> F
        where F: Float + Add<F> + Zero + One {
    let acc = <F as One>::one() + *r;
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount / acc.powf(t)).sum()
}

/// Calculates the present value of future cash flows with continuous compounded interest
pub fn cash_flow_pv<F>(times: &[F], amounts: &[F], r: &F) -> F
        where F: Float + Add<F> + Zero {
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount * (-(*r * t)).exp()).sum()
}

/// Calculates the internal rate of return
/// Initial bucket is set to F::zero()..F::one()
pub fn cash_flow_irr<F>(times: &[F], amounts: &[F], accuracy: &F, max_iteration: usize) -> Option<F>
        where F: Float + Add<F> + Zero + One + Sub<F> + Mul<F> + Half {

    let (zero, one) = (<F as Zero>::zero(), <F as One>::one());
    let (mut x1, mut x2) = (zero, one);
    let (mut f1, mut f2) = (cash_flow_pv(times, amounts, &x1), cash_flow_pv(times, amounts, &x2));

    if f1 * f2 > zero {
        return None;
    }

    // order x1/x2 f1/f2
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
    let a = cash_flow_irr(&[1.0, 2.0, 3.0], &[-2f64, 1.0, 1.0], &1.0e-5, 50);
    assert_eq!(a, Some(0.9999923706054688));
}
