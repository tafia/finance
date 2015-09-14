#![feature(iter_arith)]
#![feature(zero_one)]

extern crate num;
extern crate rand;

mod bond;
mod cash_flow;
mod future;
mod math;
mod option;
mod term_structure;

use num::traits::{Float, Signed};
use std::num::{Zero, One};
use std::ops::{Add, Sub, Mul};

pub const ACCURACY: f64 = 1.0e-5;
