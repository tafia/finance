#![feature(iter_arith)]
#![feature(zero_one)]

extern crate num;

mod bond;
mod cash_flow;
mod future;
mod term_structure;

use num::traits::{Float, Signed};
use std::num::{Zero, One};
use std::ops::{Add, Sub, Mul};

/// General purpose float trait for finance
pub trait FloatExt : Float + Zero + One + Add<Self> + Sub<Self> + Mul<Self> + Signed {
    /// Set Self = Self / 2
    fn half(&mut self);
    fn double(&mut self);
    fn default_accuracy() -> Self;
}

impl FloatExt for f64 {
    fn half(&mut self) {
        *self *= 0.5;
    }
    fn double(&mut self) {
        *self *= 2.0;
    }
    fn default_accuracy() -> f64 {
        1.0e-5
    }
}
