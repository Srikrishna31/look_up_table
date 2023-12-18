use thiserror::Error;

#[derive(Error, Debug)]
enum ConstructionError {
    #[error("X values should be in strictly increasing order")]
    IncreasingXOrderError,
    #[error("Cannot create a Lookup Table containing NaNs or Infinities")]
    ContainingNansOrInifinities,
    #[error("Atleast two values should be provided for all dimensions")]
    MinLengthError,
    #[cfg(featuer="no-std")]
    #[error("Functions with more than {MAX_FUNCTION_POINTS} are not supported")]
    MaxLengthError,
}