use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstructionError {
    #[error("Independent Dimension values should be in strictly increasing order")]
    IncreasingDimOrderError,
    #[error("Cannot create a Lookup Table containing NaNs or Infinities")]
    ContainingNansOrInfinities,
    #[error("At least two values should be provided for all dimensions")]
    MinLengthError,
    #[cfg(featuer = "no-std")]
    #[error("Functions with more than {MAX_FUNCTION_POINTS} are not supported")]
    MaxLengthError,
}
