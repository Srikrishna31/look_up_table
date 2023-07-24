#![no_std]

#[cfg(not(feature = "no-std"))]
extern crate std;

mod oned_lut;
mod twod_lut;

pub(crate) const EPSILON: f64 = 0.00000001;

use cfg_if::cfg_if;
pub use oned_lut::{OneDLookUpTable, OneDLookUpTableRef};
pub use twod_lut::{TwoDLookUpTable, TwoDLookUpTableRef};

cfg_if! {
    if #[cfg(feature="no-std")] {
        const MAX_STRING_SIZE: usize = 100;
        pub(crate) const MAX_FUNCTION_POINTS: usize = 1000;

        pub(crate) type String = heapless::String<MAX_STRING_SIZE>;
        // Limit the elements to be stored in vector to 1000; For practical purposes this should suffice.
        pub(crate) type Vec<T> = heapless::Vec<T, MAX_FUNCTION_POINTS>;
    } else {
        pub(crate) type String = std::string::String;
        pub(crate) type Vec<T> = std::vec::Vec<T>;
    }
}
