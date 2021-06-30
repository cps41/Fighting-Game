use std::vec::Vec;

// Force Vectors
// [force x, -force y]
// force y is negated because y increases as you 
pub const gravity: Vec<f64> = vec![0.0, 9.81];
pub const jump: Vec<f64> = vec![0.0, 12.0]; // arbitrary, subject to change
pub const punch: Vec<f64> = vec![-7.0, 0.0]; // arbitrary, subject to change
pub const kick: Vec<f64> = vec![-5.5, 0.0]; // arbitrary, subject to change
pub const low_kick: Vec<f64> = vec![-5.0, 0.0]; // arbitrary, subject to change