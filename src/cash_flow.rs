use std::mem;
use std::ops::Range;
use super::ACCURACY;

/// Calculates the present value of future cash flows with discrete compounding
pub fn pv_discrete(times: &[f64], amounts: &[f64], r: f64) -> f64 {
    let acc = 1.0 + r;
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount / acc.powf(t)).sum()
}

/// Calculates the present value of future cash flows with continuous compounded interest
pub fn pv(times: &[f64], amounts: &[f64], r: f64) -> f64 {
    amounts.iter().zip(times.iter())
    .map(|(&amount, &t)| amount * (-(r * t)).exp()).sum()
}

/// Calculates the internal rate of return
pub fn irr(times: &[f64], amounts: &[f64], max_iteration: usize, bucket: &Range<f64>) -> Option<f64> {

    let (mut x1, mut x2) = (bucket.start, bucket.end);
    let (mut f1, mut f2) = (pv(times, amounts, x1), pv(times, amounts, x2));

    if f1 * f2 > 0.0 {
        return None;
    }

    // sort x1/x2 f1/f2
    if f1 > 0.0 {
        mem::swap(&mut x1, &mut x2);
        mem::swap(&mut f1, &mut f2)
    }

    let (mut rtb, mut dx) = (x1, x2 - x1);
    for _ in 0..max_iteration {
        dx / 2.0;
        let x_mid = rtb + dx;
        let f_mid = pv(times, amounts, x_mid);
        if f_mid.abs() < ACCURACY || dx.abs() < ACCURACY {
            return Some(x_mid)
        }
        if f_mid < 0.0 {
            rtb = x_mid;
        }
    }
    None
}

pub fn is_unique_irr(times: &[f64], amounts: &[f64]) -> bool {
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
    let a = pv_discrete(&[0f64, 1.0, 2.0], &[1f64, 1.0, 1.0], 1f64);
    assert!(a - 1.75 < 1e-10);
}

#[test]
fn test_pv() {
    let a = pv(&[0f64, 1.0, 2.0], &[1f64, 1.0, 1.0], 1f64);
    assert!(a - 1.50321472440 < 1e-10);
}

#[test]
fn test_irr() {
    let a = irr(&[1.0, 2.0, 3.0], &[-2f64, 1.0, 1.0], 50, &(0.0f64..1.0));
    assert_eq!(a, Some(0.9999923706054688));
}

#[test]
fn test_is_unique_irr() {
    let a = is_unique_irr(&[1.0, 2.0, 3.0], &[-2f64, 1.0, 1.0]);
    assert_eq!(a, true);
}
