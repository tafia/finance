#![feature(iter_arith)]
#![feature(zero_one)]

extern crate num;

mod time;

use num::traits::Float;
use std::num::{Zero, One};
use std::ops::{Add, Sub, Mul};

/// General purpose float trait for finance
pub trait FloatExt : Float + Zero + One + Add<Self> + Sub<Self> + Mul<Self> {
    /// Set Self = Self / 2
    fn half(&mut self);
}

impl FloatExt for f64 {
    fn half(&mut self) {
        *self *= 0.5;
    }
}
