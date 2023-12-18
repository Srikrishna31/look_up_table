//! Look Up Table
//! Look up tables mimic the evaluation of functions at different points. If a function is complicated
//! or time taking to evaluate at any given point, one can use a Look Up Table, which contains the
//! pre-evaluated sample points of the function.
//! Later when an evaluation is requested, if the sample is directly found, the corresponding value is
//! returned. On the other hand, if the value is not found, then an interpolation (most of the times
//! it is linear to get better performance) of the values is performed to get an estimate of the
//! actual function. In practice this gives a reasonable approximation of the function.
//! Ofcourse when the values are out of bounds, then the last values are returned always.

mod interpolation;

use super::oned_lut::interpolation::{interpolate, is_object_constructible, Key};
use crate::error::ConstructionError;
use crate::String;
use core::cell::RefCell;

#[cfg(not(feature = "no-std"))]
use std::collections::HashMap;

#[cfg(feature = "no-std")]
use hashbrown::HashMap;

use num::Float;

/// Linear interpolation with nearest neighbor extrapolation when index is outside support region,
/// and with Caching support to enable fast lookups on same values.
/// This structure is an owning structure in that, it owns the array values passed into it. It is
/// useful for defining LUTs at compile time.
#[derive(Debug)]
pub struct OneDLookUpTable<const N: usize> {
    x: [f64; N],
    y: [f64; N],
    cache: RefCell<HashMap<Key, f64>>,
}

impl<const N: usize> OneDLookUpTable<N> {
    /// Constructs a `OneDLookUpTable` object, given the input arrays `x` and `y` modelling the sample
    /// points of a uni-variate function.
    /// If the `x` values are not sorted in ascending order:
    /// ```
    ///  use look_up_table::{OneDLookUpTable, ConstructionError};
    ///  let lut = OneDLookUpTable::new([3.0, 1.0, 2.0], [1.0;3]);
    ///  assert!(matches!(lut.err().unwrap(), ConstructionError::IncreasingXOrderError))
    /// ```
    ///
    /// If the `x` or `y` values contain NANs or Infinities
    /// ```
    ///  use look_up_table::{OneDLookUpTable, ConstructionError};
    ///  let lut = OneDLookUpTable::new([f64::NAN, 1.0, 2.0], [f64::NEG_INFINITY;3]);
    ///  assert!(matches!(lut.err().unwrap(), ConstructionError::ContainingNansOrInfinities))
    /// ```
    ///
    /// If the `x` or `y` values are arrays of 1 value:
    /// ```
    ///  use look_up_table::{OneDLookUpTable, ConstructionError};
    ///  let lut = OneDLookUpTable::new([1.0], [f64::NEG_INFINITY]);
    ///  assert!(matches!(lut.err().unwrap(), ConstructionError::MinLengthError))
    /// ```
    pub fn new(x: [f64; N], y: [f64; N]) -> Result<OneDLookUpTable<N>, ConstructionError> {
        is_object_constructible(&x, &y).map(|_| OneDLookUpTable {
            x,
            y,
            cache: RefCell::new(HashMap::new()),
        })
    }

    /// Returns an interpolated value for the given `index` or x value. If the `index`
    /// value is present in the array, it directly returns the corresponding y value without any
    /// interpolation. If the `index` value lies outside the range, then it clamps the values to the
    /// boundary values.
    pub fn get(&self, index: &f64) -> f64 {
        // Due to index traits requirements of returning references, we cannot use it to overload.

        // There could be a possibility that the values which are very close in real number line to
        // have different bit patterns, so this code would do a full interpolation for nearly identical
        // value lookups.
        let ind = index.integer_decode();
        if self.cache.borrow().contains_key(&ind) {
            return *self.cache.borrow().get(&ind).unwrap();
        }

        let y = interpolate(index, &self.x, &self.y);

        self.cache.borrow_mut().insert(ind, y);

        *self.cache.borrow().get(&ind).unwrap()
    }
}

/// This struct allows reference arrays/slices to be used as lookup functions, which can be defined
/// at runtime or borrow the slices from other enclosing objects.
#[derive(Debug)]
pub struct OneDLookUpTableRef<'a, 'b> {
    xs: &'a [f64],
    ys: &'b [f64],
    cache: RefCell<HashMap<Key, f64>>,
}

impl<'a, 'b> OneDLookUpTableRef<'a, 'b> {
    pub fn new(xs: &'a [f64], ys: &'b [f64]) -> Result<OneDLookUpTableRef<'a, 'b>, ConstructionError> {
        is_object_constructible(xs, ys).map(|_| OneDLookUpTableRef {
            xs,
            ys,
            cache: RefCell::new(HashMap::new()),
        })
    }

    pub fn get(&self, index: &f64) -> f64 {
        // There could be a possibility that the values which are very close in real number line to
        // have different bit patterns, so this code would do a full interpolation for nearly identical
        // value lookups.
        let ind = index.integer_decode();
        if self.cache.borrow().contains_key(&ind) {
            return *self.cache.borrow().get(&ind).unwrap();
        }

        let y = interpolate(index, self.xs, self.ys);

        self.cache.borrow_mut().insert(ind, y);

        *self.cache.borrow().get(&ind).unwrap()
    }
}
