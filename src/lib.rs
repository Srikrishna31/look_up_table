#![no_std]

mod oned_lut;
mod twod_lut;

pub(crate) const EPSILON: f64 = 0.00000001;
pub(crate) const MAX_FUNCTION_POINTS: usize = 1000;
const MAX_STRING_SIZE: usize = 100;

pub use oned_lut::{OneDLookUpTable, OneDLookUpTableRef};
pub use twod_lut::{TwoDLookUpTable, TwoDLookUpTableRef as TwoDLookUpTableCow, TwoDLookUpTableRef};

pub(crate) type String = heapless::String<MAX_STRING_SIZE>;
// Limit the elements to be stored in vector to 1000; For practical purposes this should suffice.
pub(crate) type Vec<T> = heapless::Vec<T, MAX_FUNCTION_POINTS>;
