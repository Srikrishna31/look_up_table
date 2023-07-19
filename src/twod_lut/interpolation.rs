use crate::twod_lut::SurfaceType;
use crate::EPSILON;
use itertools::Itertools;
use num::complex::ComplexFloat;
use std::borrow::{Borrow, Cow};
use std::iter::Iterator;
use std::ops::Sub;

pub(in crate::twod_lut) fn is_object_constructible_gen<I, J, K>(xs: I, ys: J, surface: K) -> Result<bool, String>
where
    I: IntoIterator + Clone,
    J: IntoIterator + Clone,
    K: IntoIterator + Clone,
    I::Item: Borrow<f64> + Sub + Clone,
    <<I as IntoIterator>::Item as Sub>::Output: PartialOrd<f64>,
    J::Item: Borrow<f64> + Sub + Clone,
    <<J as IntoIterator>::Item as Sub>::Output: PartialOrd<f64>,
    K::Item: IntoIterator,
    <<K as IntoIterator>::Item as IntoIterator>::Item: Borrow<f64> + Sub + Clone,
{
    if xs.clone().into_iter().count() < 2 || ys.clone().into_iter().count() < 2 {
        return Err("At least two values should be provided for x and y axes".to_string());
    }

    if itertools::any(xs.clone(), |v| v.borrow().is_nan() || v.borrow().is_infinite())
        || itertools::any(ys.clone(), |v| v.borrow().is_nan() || v.borrow().is_infinite())
        || surface.clone()
            .into_iter()
            .any(|row| itertools::any(row, |v| v.borrow().is_nan() || v.borrow().is_infinite()))
    {
        return Err("Cannot create a Lookup Table containing NaNs or Infinities".to_string());
    }

    let itxs = xs.into_iter().tuple_windows::<(_, _)>();
    let itys = ys.into_iter().tuple_windows::<(_, _)>();
    if !itertools::all(itxs, |(prev, curr)| curr - prev > EPSILON)
        || !itertools::all(itys, |(prev, curr)| curr - prev > EPSILON)
    {
        return Err("X and Y values should be in strictly increasing order".to_string());
    }

    Ok(true)
}

// TODO: Try to unify the two copies of the functions by using an Iterator implementation
// or some other similar construct.

pub(in crate::twod_lut) fn is_object_constructible_dynamic(
    xs: &[f64],
    ys: &[f64],
    surface: &[&[f64]],
) -> Result<bool, String> {
    if xs.len() < 2 || ys.len() < 2 {
        return Err("At least two values should be provided for x and y axes".to_string());
    }

    let check_nan_infinity = |v: &f64| v.is_nan() || v.is_infinite();

    if xs.iter().any(check_nan_infinity)
        || ys.iter().any(check_nan_infinity)
        || surface.iter().any(|row| row.iter().any(check_nan_infinity))
    {
        return Err("Cannot create a Lookup Table containing NaNs or Infinities".to_string());
    }

    if !xs.windows(2).all(|c| c[1] - c[0] > EPSILON) || !ys.windows(2).all(|c| c[1] - c[0] > EPSILON) {
        return Err("X and Y values should be in strictly increasing order".to_string());
    }

    Ok(true)
}

/// Given an index value, try to find the lower and upper bound indices and return them.
/// If the index is out of bounds, return both values as boundary values. If the index directly
/// matches the values present in the array, then return the same value as lower and upper bounds.
#[inline]
fn get_indices(v: &f64, vs: &[f64]) -> (usize, usize) {
    if *v < vs[0] {
        (0, 0)
    } else if *v > vs[vs.len() - 1] {
        (vs.len() - 1, vs.len() - 1)
    } else {
        match vs.binary_search_by(|val| val.partial_cmp(v).unwrap()) {
            Ok(ind) => (ind, ind),
            Err(ind) => (ind - 1, ind),
        }
    }
}

pub(in crate::twod_lut) fn interpolate<const M: usize, const N: usize>(
    x: &f64,
    y: &f64,
    xs: &[f64; M],
    ys: &[f64; N],
    surface: &SurfaceType<M, N>,
) -> f64 {
    // Retrieve the lower and upper bound indices for x and y axes.
    let (x1_ind, x2_ind) = get_indices(x, xs);
    let (y1_ind, y2_ind) = get_indices(y, ys);
    let (x1, x2, y1, y2) = (xs[x1_ind], xs[x2_ind], ys[y1_ind], ys[y2_ind]);

    // These represent the four corners of the quad, within which the interpolation is to be done.
    let fq11 = surface[y1_ind][x1_ind];
    let fq12 = surface[y1_ind][x2_ind];
    let fq21 = surface[y2_ind][x1_ind];
    let fq22 = surface[y2_ind][x2_ind];

    // if both the indices are out of range, then return the corner point
    // if one of the indices is out of range or maps to an exact breakpoint,
    // then perform interpolation only in other direction.
    // else perform interpolation on both the axes.
    if fq11 == fq22 {
        fq11
    } else if fq11 == fq21 {
        let alpha = (x - x1) / (x2 - x1);

        fq11 + alpha * (fq12 - fq11)
    } else if fq11 == fq12 {
        let alpha = (y - y1) / (y2 - y1);

        fq11 + alpha * (fq21 - fq11)
    } else {
        let alpha_x = (x - x1) / (x2 - x1);
        let alpha_y = (y - y1) / (y2 - y1);

        let fxy1 = fq11 + alpha_x * (fq21 - fq11);
        let fxy2 = fq12 + alpha_x * (fq22 - fq12);

        fxy1 + (fxy2 - fxy1) * alpha_y
    }
}

pub(in crate::twod_lut) fn interpolate_dynamic(x: &f64, y: &f64, xs: &[f64], ys: &[f64], surface: &[&[f64]]) -> f64 {
    // Retrieve the lower and upper bound indices for x and y axes.
    let (x1_ind, x2_ind) = get_indices(x, xs);
    let (y1_ind, y2_ind) = get_indices(y, ys);
    let (x1, x2, y1, y2) = (xs[x1_ind], xs[x2_ind], ys[y1_ind], ys[y2_ind]);

    // These represent the four corners of the quad, within which the interpolation is to be done.
    let fq11 = surface[y1_ind][x1_ind];
    let fq12 = surface[y1_ind][x2_ind];
    let fq21 = surface[y2_ind][x1_ind];
    let fq22 = surface[y2_ind][x2_ind];

    // if both the indices are out of range, then return the corner point
    // if one of the indices is out of range or maps to an exact breakpoint,
    // then perform interpolation only in other direction.
    // else perform interpolation on both the axes.
    if fq11 == fq22 {
        fq11
    } else if fq11 == fq21 {
        let alpha = (x - x1) / (x2 - x1);

        fq11 + alpha * (fq12 - fq11)
    } else if fq11 == fq12 {
        let alpha = (y - y1) / (y2 - y1);

        fq11 + alpha * (fq21 - fq11)
    } else {
        let alpha_x = (x - x1) / (x2 - x1);
        let alpha_y = (y - y1) / (y2 - y1);

        let fxy1 = fq11 + alpha_x * (fq21 - fq11);
        let fxy2 = fq12 + alpha_x * (fq22 - fq12);

        fxy1 + (fxy2 - fxy1) * alpha_y
    }
}

pub(in crate::twod_lut) fn interpolate_dynamic_cow(
    x: &f64,
    y: &f64,
    xs: &[f64],
    ys: &[f64],
    surface: &[Cow<'static, [f64]>],
) -> f64 {
    // Retrieve the lower and upper bound indices for x and y axes.
    let (x1_ind, x2_ind) = get_indices(x, xs);
    let (y1_ind, y2_ind) = get_indices(y, ys);
    let (x1, x2, y1, y2) = (xs[x1_ind], xs[x2_ind], ys[y1_ind], ys[y2_ind]);

    // These represent the four corners of the quad, within which the interpolation is to be done.
    let fq11 = surface[y1_ind][x1_ind];
    let fq12 = surface[y1_ind][x2_ind];
    let fq21 = surface[y2_ind][x1_ind];
    let fq22 = surface[y2_ind][x2_ind];

    // if both the indices are out of range, then return the corner point
    // if one of the indices is out of range or maps to an exact breakpoint,
    // then perform interpolation only in other direction.
    // else perform interpolation on both the axes.
    if fq11 == fq22 {
        fq11
    } else if fq11 == fq21 {
        let alpha = (x - x1) / (x2 - x1);

        fq11 + alpha * (fq12 - fq11)
    } else if fq11 == fq12 {
        let alpha = (y - y1) / (y2 - y1);

        fq11 + alpha * (fq21 - fq11)
    } else {
        let alpha_x = (x - x1) / (x2 - x1);
        let alpha_y = (y - y1) / (y2 - y1);

        let fxy1 = fq11 + alpha_x * (fq21 - fq11);
        let fxy2 = fq12 + alpha_x * (fq22 - fq12);

        fxy1 + (fxy2 - fxy1) * alpha_y
    }
}
