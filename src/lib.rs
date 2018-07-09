extern crate num;
extern crate rand;

pub mod bond;
pub mod cash_flow;
pub mod future;
pub mod math;
pub mod option;
pub mod term_structure;

pub const ACCURACY: f64 = 1.0e-5;
pub const MAX_ITERATIONS: usize = 100;
pub const HIGH_VALUE: f64 = 1e10;
pub const ERROR: f64 = -1e40;
