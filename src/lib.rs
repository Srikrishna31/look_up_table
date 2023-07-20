mod oned_lut;
mod twod_lut;

pub(crate) const EPSILON: f64 = 0.00000001;

pub use oned_lut::{OneDLookUpTable, OneDLookUpTableRef};
pub use twod_lut::{TwoDLookUpTable, TwoDLookUpTableRef as TwoDLookUpTableCow, TwoDLookUpTableRef};
