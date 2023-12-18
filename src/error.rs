use thiserror_no_std::Error;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="no-std")] {
        use crate::MAX_FUNCTION_POINTS;
    }
}

#[derive(Error, Debug)]
pub enum ConstructionError {
    #[error("Independent Dimension values should be in strictly increasing order")]
    IncreasingDimOrderError,
    #[error("Cannot create a Lookup Table containing NaNs or Infinities")]
    ContainingNansOrInfinities,
    #[error("At least two values should be provided for all dimensions")]
    MinLengthError,
    #[cfg(feature = "no-std")]
    #[error("Functions with more than {MAX_FUNCTION_POINTS} are not supported")]
    MaxLengthError,
}
