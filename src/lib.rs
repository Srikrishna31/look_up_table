#![no_std]

#[cfg(not(feature = "no-std"))]
extern crate std;

mod error;
mod oned_lut;
mod twod_lut;

pub(crate) const EPSILON: f64 = 0.00000001;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="no-std")] {
        pub(crate) const MAX_FUNCTION_POINTS: usize = 1000;

        // Limit the elements to be stored in vector to 1000; For practical purposes this should suffice.
        pub(crate) type Vec<T> = heapless::Vec<T, MAX_FUNCTION_POINTS>;
    } else {
        pub(crate) type Vec<T> = std::vec::Vec<T>;
    }
}

// Re-exports for public api
pub use error::ConstructionError;
pub use oned_lut::{OneDLookUpTable, OneDLookUpTableRef};
pub use twod_lut::{TwoDLookUpTable, TwoDLookUpTableRef};
