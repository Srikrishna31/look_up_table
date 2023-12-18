use crate::error::ConstructionError;
use crate::error::ConstructionError::{ContainingNansOrInfinities, IncreasingXOrderError, MinLengthError};
use crate::EPSILON;

pub(super) type Key = (u64, i16, i8);
use crate::String;

pub(in crate::oned_lut) fn is_object_constructible(xs: &[f64], ys: &[f64]) -> Result<bool, ConstructionError> {
    if xs.len() < 2 || ys.len() < 2 {
        return Err(MinLengthError);
    }

    if xs.iter().any(|v| v.is_nan() || v.is_infinite()) || ys.iter().any(|v| v.is_nan() || v.is_infinite()) {
        return Err(ContainingNansOrInfinities);
    }

    if !xs.windows(2).all(|c| c[1] - c[0] > EPSILON) {
        return Err(IncreasingXOrderError);
    }

    Ok(true)
}

pub(in crate::oned_lut) fn interpolate(x: &f64, xs: &[f64], ys: &[f64]) -> f64 {
    if *x < xs[0] {
        return ys[0];
    }

    if *x > xs[xs.len() - 1] {
        return ys[ys.len() - 1];
    }

    let lub = match xs.binary_search_by(|val| val.partial_cmp(x).unwrap()) {
        // perform interpolation only when the value is not found.
        Ok(ind) => return ys[ind],
        Err(ind) => ind,
    };
    let prev = lub - 1;
    let alpha = (x - xs[prev]) / (xs[lub] - xs[prev]);

    let y1 = &ys[prev];
    let y2 = &ys[lub];

    y1 + alpha * (y2 - y1)
}
