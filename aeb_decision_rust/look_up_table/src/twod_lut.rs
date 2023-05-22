use num::Float;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::EPSILON;

type Key = ((u64, i16, i8), (u64, i16, i8));


/// Type alias for a surface - a 2D array, where M is the height and N is the width.
pub type SurfaceType<const M: usize, const N: usize> = [[f64;N]; M];

#[derive(Debug)]
pub struct TwoDLookUpTable<const M: usize, const N: usize> {
    x: [f64; M],
    y: [f64; N],
    surface: SurfaceType<M, N>,
    cache: RefCell<HashMap<Key, f64>>,
}

impl<const M: usize, const N: usize> TwoDLookUpTable<M, N> {
    pub fn new(
        x: [f64; M],
        y: [f64; N],
        surface: SurfaceType<M, N>,
    ) -> Result<TwoDLookUpTable<M, N>, String> {
        // TODO: explore if this constraint can be expressed in generics to move this check to compile time
        if N < 2 || M < 2{
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

    fn check_nans_and_infinities(x: &[f64; M], y: &[f64; N], surface: &SurfaceType<M, N>) -> bool {
        x.iter().any(|v| v.is_nan() || v.is_infinite())
            || y.iter().any(|v| v.is_nan() || v.is_infinite())
            || surface
                .iter()
                .any(|row| row.iter().any(|v| v.is_nan() || v.is_infinite()))
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        // First do the cache lookup
        let x_bits = x.integer_decode();
        let y_bits = y.integer_decode();

        if self.cache.borrow().contains_key(&(x_bits, y_bits)) {
            return *self.cache.borrow().get(&(x_bits, y_bits)).unwrap();
        }

        // if one of the indices is out of range, then perform interpolation only in that direction.
        match (x > )
        0.0
    }

    fn unidirectional_interpolation(y: &f64, ys: &[f64; N]) -> f64 {
        if *y < ys[0] {
            return ys[0];
        }

        if *y > ys[N - 1] {
            return ys[N - 1];
        }

        let lub = match ys.binary_search_by(|ydash| ydash.partial_cmp(y).unwrap()) {
            Ok(ind) => return *ys[ind],
            Err(ind) => ind,
        };
        let prev = lub - 1;

        let alpha =
    }
}
