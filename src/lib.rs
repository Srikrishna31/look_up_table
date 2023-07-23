#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod oned_lut;
mod twod_lut;

pub(crate) const EPSILON: f64 = 0.00000001;

#[cfg(feature = "no-std")]
pub(crate) const MAX_FUNCTION_POINTS: usize = 1000;

#[cfg(feature = "no-std")]
const MAX_STRING_SIZE: usize = 100;

pub use oned_lut::{OneDLookUpTable, OneDLookUpTableRef};
pub use twod_lut::{TwoDLookUpTable, TwoDLookUpTableRef};

//#[cfg(not(feature="std"))]
#[cfg(feature = "no-std")]
pub(crate) type String = heapless::String<MAX_STRING_SIZE>;
//#[cfg(not(feature="std"))]
#[cfg(feature = "no-std")]
// Limit the elements to be stored in vector to 1000; For practical purposes this should suffice.
pub(crate) type Vec<T> = heapless::Vec<T, MAX_FUNCTION_POINTS>;

#[cfg(feature = "std")]
pub(crate) type String = std::string::String;

#[cfg(feature = "std")]
pub(crate) type Vec<T> = std::vec::Vec<T>;
