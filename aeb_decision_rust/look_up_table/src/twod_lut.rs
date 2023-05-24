//! Two Dimensional Look Up Table
//! 2D LUT is similar to 1D LUT, except that in 2D, we have two independent variables,and the function
//! is a surface as opposed to a curve in 1D. Since we have two independent variables, we have to do
//! atleast 2 interpolations for each lookup that is not directly present as a function break point.
//! It is also known as bilinear interpolation, and is commonly used in Computer Graphics, Image Processing
//! areas.
//! [Bilinear Interpolation](https://en.wikipedia.org/wiki/Bilinear_interpolation)

use crate::EPSILON;
use num::Float;
use std::cell::RefCell;
use std::collections::HashMap;

type Key = ((u64, i16, i8), (u64, i16, i8));

/// Type alias for a surface - a 2D array, where M is the height(rows) and N is the width(columns).
pub type SurfaceType<const M: usize, const N: usize> = [[f64; N]; M];

/// Two Dimensional Linear interpolation with nearest neighbor extrapolation when indices are outside
/// support region, and with caching support to enable fast lookups on frequently used values.
/// M is the number of rows and signifies height
/// N is the number of columns and signifies width
#[derive(Debug)]
pub struct TwoDLookUpTable<const M: usize, const N: usize> {
    x: [f64; M],                       // Breakpoints/sample points on x-axis
    y: [f64; N],                       // Breakpoints/sample points on y-axis
    surface: SurfaceType<M, N>,        // Corresponding function values for x and y indices.
    cache: RefCell<HashMap<Key, f64>>, // A cache to support fast lookup for frequently used values.
}

impl<const M: usize, const N: usize> TwoDLookUpTable<M, N> {
    /// Constructs a `OneDLookUpTable` object, given the input arrays `x` and `y` modelling the sample
    /// points of a uni-variate function.
    /// If the `x` or `y` values are not sorted in ascending order:
    /// ```
    ///  use look_up_table::TwoDLookUpTable;
    ///  let lut = TwoDLookUpTable::new([3.0, 1.0, 2.0], [1.0;2], [[1.0;2]; 3]);
    ///  assert_eq!(lut.err().unwrap(), "X and Y values should be in strictly increasing order")
    /// ```
    ///
    /// If the `x` or `y` or `surface` values contain NANs or Infinities
    /// ```
    ///  use look_up_table::TwoDLookUpTable;
    ///  let lut = TwoDLookUpTable::new([f64::NAN, 1.0, 2.0], [f64::NEG_INFINITY;3], [[1.0;3];3]);
    ///  assert_eq!(lut.err().unwrap(), "Cannot create a Lookup Table containing NaNs or Infinities")
    /// ```
    ///
    /// If the `x` or `y` or `surface` values are arrays of 1 value:
    /// ```
    ///  use look_up_table::TwoDLookUpTable;
    ///  let lut = TwoDLookUpTable::new([1.0], [f64::NEG_INFINITY], [[1.0]; 1]);
    ///  assert_eq!(lut.err().unwrap(), "At least two values should be provided for x and y axes")
    /// ```
    pub fn new(
        x: [f64; M],
        y: [f64; N],
        surface: SurfaceType<M, N>,
    ) -> Result<TwoDLookUpTable<M, N>, String> {
        // TODO: explore if this constraint can be expressed in generics to move this check to compile time
        if N < 2 || M < 2 {
            return Err("At least two values should be provided for x and y axes".to_string());
        }

        if Self::check_nans_and_infinities(&x, &y, &surface) {
            return Err("Cannot create a Lookup Table containing NaNs or Infinities".to_string());
        }

        if !x.windows(2).all(|c| c[1] - c[0] > EPSILON)
            || !y.windows(2).all(|c| c[1] - c[0] > EPSILON)
        {
            return Err("X and Y values should be in strictly increasing order".to_string());
        }

        Ok(TwoDLookUpTable {
            x,
            y,
            surface,
            cache: RefCell::new(HashMap::new()),
        })
    }

    #[inline]
    fn check_nans_and_infinities(x: &[f64; M], y: &[f64; N], surface: &SurfaceType<M, N>) -> bool {
        x.iter().any(|v| v.is_nan() || v.is_infinite())
            || y.iter().any(|v| v.is_nan() || v.is_infinite())
            || surface
                .iter()
                .any(|row| row.iter().any(|v| v.is_nan() || v.is_infinite()))
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

    /// Returns an interpolated value for the given `x` and `y` indices. If the values are directly
    /// present in the array, it directly returns the corresponding `surface` value without any
    /// interpolation. If on the other hand, only one index value is either outside the support
    /// region, or directly present in the array, then does the interpolation in the other direction.
    /// If both indices are not present in the arrays, but are within the bounds, then does the
    /// linear interpolation in each direction to arrive at the final value.
    pub fn get(&self, x: &f64, y: &f64) -> f64 {
        // First do the cache lookup
        let key = (x.integer_decode(), y.integer_decode());

        if self.cache.borrow().contains_key(&key) {
            return *self.cache.borrow().get(&key).unwrap();
        }

        // Retrieve the lower and upper bound indices for x and y axes.
        let (x1_ind, x2_ind) = Self::get_indices(x, &self.x);
        let (y1_ind, y2_ind) = Self::get_indices(y, &self.y);
        let (x1, x2, y1, y2) = (
            self.x[x1_ind],
            self.x[x2_ind],
            self.y[y1_ind],
            self.y[y2_ind],
        );

        // These represent the four corners of the quad, within which the interpolation is to be done.
        let fq11 = self.surface[y1_ind][x1_ind];
        let fq12 = self.surface[y1_ind][x2_ind];
        let fq21 = self.surface[y2_ind][x1_ind];
        let fq22 = self.surface[y2_ind][x2_ind];

        // if both the indices are out of range, then return the corner point
        // if one of the indices is out of range or maps to an exact breakpoint,
        // then perform interpolation only in other direction.
        // else perform interpolation on both the axes.
        let z = if fq11 == fq22 {
            fq11
        } else if fq11 == fq21 {
            let alpha = (x - x1) / (x2 - x1);

            fq11 + alpha * fq12
        } else if fq11 == fq12 {
            let alpha = (y - y1) / (y2 - y1);

            fq11 + alpha * fq21
        } else {
            let alpha_x = (x - x1) / (x2 - x1);
            let alpha_y = (y - y1) / (y2 - y1);

            let fxy1 = fq11 + alpha_x * fq21;
            let fxy2 = fq12 + alpha_x * fq22;

            fxy1 + alpha_y * fxy2
        };

        // store the value in cache before returning, to speedup look up process in the future.
        self.cache.borrow_mut().insert(key, z);

        *self.cache.borrow().get(&key).unwrap()
    }
}
