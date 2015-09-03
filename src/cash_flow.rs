use std::num::{Zero, One};
use std::mem;
use std::fmt::Debug;
use std::ops::Range;
use super::FloatExt;

/// Calculates the present value of future cash flows with discrete compounding
pub fn pv_discrete<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    let acc = <F as One>::one() + *r;
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount / acc.powf(t)).sum()
}

/// Calculates the present value of future cash flows with continuous compounded interest
pub fn pv<F: FloatExt>(times: &[F], amounts: &[F], r: &F) -> F {
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount * (-(*r * t)).exp()).sum()
}

/// Calculates the internal rate of return
pub fn irr<F: FloatExt>(times: &[F], amounts: &[F],
    accuracy: &F, max_iteration: usize, bucket: &Range<F>) -> Option<F> {

    let (mut x1, mut x2) = (bucket.start, bucket.end);
    let (mut f1, mut f2) = (pv(times, amounts, &x1), pv(times, amounts, &x2));

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
        let f_mid = pv(times, amounts, &x_mid);
        if f_mid.abs() < *accuracy || dx.abs() < *accuracy {
            return Some(x_mid)
        }
        if f_mid < zero {
            rtb = x_mid;
        }
    }
    None
}

pub fn is_unique_irr<F: FloatExt>(times: &[F], amounts: &[F]) -> bool {
    match amounts[1..].iter().zip(amounts[..times.len() - 1].iter())
        .filter(|&(&a1, &a0)| a1.signum() != a0.signum()).count() {
        0 => false,
        1 => true,
        _ => {
            let mut sign_changes = 0;
            let mut sum_a = amounts[0];
            for &a in amounts[1..].iter() {
                let old_sum = sum_a;
                sum_a = sum_a + a;
                if sum_a.signum() != old_sum {
                    sign_changes += 1;
                }
            }
            sign_changes <= 1
        }
    }

}

#[test]
fn test_pv_discrete() {
    let a = pv_discrete(&[0f64, 1.0, 2.0], &[1f64, 1.0, 1.0], &1f64);
    assert!(a - 1.75 < 1e-10);
}

#[test]
fn test_pv() {
    let a = pv(&[0f64, 1.0, 2.0], &[1f64, 1.0, 1.0], &1f64);
    assert!(a - 1.50321472440 < 1e-10);
}

#[test]
fn test_irr() {
    let a = irr(&[1.0, 2.0, 3.0], &[-2f64, 1.0, 1.0], &1.0e-5, 50, &(0.0f64..1.0));
    assert_eq!(a, Some(0.9999923706054688));
}

#[test]
fn test_is_unique_irr() {
    let a = is_unique_irr(&[1.0, 2.0, 3.0], &[-2f64, 1.0, 1.0]);
    assert_eq!(a, true);
}
