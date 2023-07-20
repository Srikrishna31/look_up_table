//! Two Dimensional Look Up Table
//! 2D LUT is similar to 1D LUT, except that in 2D, we have two independent variables,and the function
//! is a surface as opposed to a curve in 1D. Since we have two independent variables, we have to do
//! atleast 2 interpolations for each lookup that is not directly present as a function break point.
//! It is also known as bilinear interpolation, and is commonly used in Computer Graphics, Image Processing
//! areas.
//! [Bilinear Interpolation](https://en.wikipedia.org/wiki/Bilinear_interpolation)
//!

mod interpolation;

use crate::twod_lut::interpolation::{interpolate_dynamic, is_object_constructible};
use num::Float;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

type Key = ((u64, i16, i8), (u64, i16, i8));

/// This trait is defined as a proxy to get the values from surface arrays, which can be of different
/// types - eg. [[f64; M]; N], of &[[f64;M]], or &[&[f64]], or Cow<&'_ [f64;M]> or Cow<&'_ Cow<&'_ [f64]>> etc.
/// Each of the Lookuptable objects catering to these objects should implement this trait, so that the
/// interpolate function can be uniform.
trait SurfaceValueGetter {
    fn get(&self, x: usize, y: usize) -> f64;
}

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
    pub fn new(xs: [f64; M], ys: [f64; N], surface: SurfaceType<M, N>) -> Result<TwoDLookUpTable<M, N>, String> {
        is_object_constructible(&xs, &ys, &surface).map(|_| TwoDLookUpTable {
            x: xs,
            y: ys,
            surface,
            cache: RefCell::new(HashMap::new()),
        })
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

        let z = interpolate_dynamic(x, y, &self.x, &self.y, self);

        // store the value in cache before returning, to speedup look up process in the future.
        self.cache.borrow_mut().insert(key, z);

        *self.cache.borrow().get(&key).unwrap()
    }
}

impl<const M: usize, const N: usize> SurfaceValueGetter for TwoDLookUpTable<M, N> {
    fn get(&self, x: usize, y: usize) -> f64 {
        self.surface[x][y]
    }
}

impl SurfaceValueGetter for TwoDLookUpTableRef<'_, '_, '_> {
    fn get(&self, x: usize, y: usize) -> f64 {
        self.surface[x][y]
    }
}

impl SurfaceValueGetter for TwoDLookUpTableCow<'_, '_> {
    fn get(&self, x: usize, y: usize) -> f64 {
        self.surface[x][y]
    }
}

#[derive(Debug)]
pub struct TwoDLookUpTableRef<'a, 'b, 'c> {
    xs: &'a [f64],
    ys: &'b [f64],
    surface: &'c [&'c [f64]],
    cache: RefCell<HashMap<Key, f64>>,
}

impl<'a, 'b, 'c> TwoDLookUpTableRef<'a, 'b, 'c> {
    pub fn new(xs: &'a [f64], ys: &'b [f64], surface: &'c [&'c [f64]]) -> Result<Self, String> {
        let mut vec = Vec::with_capacity(surface.len());
        surface.iter().for_each(|v| vec.push(&v[0..v.len()]));
        is_object_constructible(xs.iter(), ys.iter(), vec.into_iter()).map(|_| TwoDLookUpTableRef {
            xs,
            ys,
            surface,
            cache: RefCell::new(HashMap::new()),
        })
    }

    pub fn get(&self, x: &f64, y: &f64) -> f64 {
        // First do the cache lookup
        let key = (x.integer_decode(), y.integer_decode());

        if self.cache.borrow().contains_key(&key) {
            return *self.cache.borrow().get(&key).unwrap();
        }

        let z = interpolate_dynamic(x, y, self.xs, self.ys, self);

        // store the value in cache before returning, to speedup look up process in the future.
        self.cache.borrow_mut().insert(key, z);

        *self.cache.borrow().get(&key).unwrap()
    }
}

#[derive(Debug)]
pub struct TwoDLookUpTableCow<'a, 'b> {
    xs: &'a [f64],
    ys: &'b [f64],
    surface: Vec<&'static [f64]>,
    cache: RefCell<HashMap<Key, f64>>,
    xy_swapped: bool,
}

impl<'a, 'b> TwoDLookUpTableCow<'a, 'b> {
    #[allow(clippy::ptr_arg)]
    pub fn new(
        xs: &'a [f64],
        ys: &'b [f64],
        surface: &'static Cow<'static, [Cow<'static, [f64]>]>,
    ) -> Result<Self, String> {
        let mut vec = Vec::with_capacity(surface.len());
        surface.iter().for_each(|v| vec.push(&v[0..v.len()]));

        // Since we are dealing with dynamic slices, align the xs and ys if the lengths are not aligned
        // according to the surface dimensions. If the lengths are same, then we assume that the xs and
        // ys are passed in the correct order.
        is_object_constructible(xs.iter(), ys.iter(), vec.clone().into_iter()).map(|_| TwoDLookUpTableCow {
            xs,
            ys,
            surface: vec,
            cache: RefCell::new(HashMap::new()),
            xy_swapped: xs.len() != ys.len() && xs.len() == surface.len(),
        })
    }

    pub fn get(&self, x: &f64, y: &f64) -> f64 {
        let (x, y) = if self.xy_swapped { (y, x) } else { (x, y) };
        // First do the cache lookup
        let key = (x.integer_decode(), y.integer_decode());

        if self.cache.borrow().contains_key(&key) {
            return *self.cache.borrow().get(&key).unwrap();
        }

        let z = if self.xy_swapped {
            interpolate_dynamic(x, y, self.ys, self.xs, self)
        } else {
            interpolate_dynamic(x, y, self.xs, self.ys, self)
        };

        // store the value in cache before returning, to speedup look up process in the future.
        self.cache.borrow_mut().insert(key, z);

        *self.cache.borrow().get(&key).unwrap()
    }
}
