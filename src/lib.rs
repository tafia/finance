#![feature(iter_arith)]

extern crate num;
extern crate rand;

mod bond;
mod cash_flow;
mod future;
mod math;
mod option;
mod term_structure;

pub const ACCURACY: f64 = 1.0e-5;
