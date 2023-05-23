//! Two Dimensional Look Up Table
use crate::EPSILON;
use num::Float;
use std::cell::RefCell;
use std::collections::HashMap;

type Key = ((u64, i16, i8), (u64, i16, i8));

/// Type alias for a surface - a 2D array, where M is the height and N is the width.
pub type SurfaceType<const M: usize, const N: usize> = [[f64; N]; M];

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

    fn check_nans_and_infinities(x: &[f64; M], y: &[f64; N], surface: &SurfaceType<M, N>) -> bool {
        x.iter().any(|v| v.is_nan() || v.is_infinite())
            || y.iter().any(|v| v.is_nan() || v.is_infinite())
            || surface
                .iter()
                .any(|row| row.iter().any(|v| v.is_nan() || v.is_infinite()))
    }

    pub fn get(&self, x: &f64, y: &f64) -> f64 {
        // First do the cache lookup
        let key = (x.integer_decode(), y.integer_decode());

        if self.cache.borrow().contains_key(&key) {
            return *self.cache.borrow().get(&key).unwrap();
        }

        // if one of the indices is out of range, then perform interpolation only in that direction.
        let (x1_ind, x2_ind) = {
            if *x < self.x[0] {
                (0, 0)
            } else if *x > self.x[M - 1] {
                (M - 1, M - 1)
            } else {
                match self.x.binary_search_by(|val| val.partial_cmp(x).unwrap()) {
                    Ok(ind) => (ind - 1, ind),
                    Err(ind) => (ind - 1, ind),
                }
            }
        };

        let (y1_ind, y2_ind) = {
            if *y < self.y[0] {
                (0, 0)
            } else if *y > self.y[N - 1] {
                (N - 1, N - 1)
            } else {
                match self.y.binary_search_by(|val| val.partial_cmp(y).unwrap()) {
                    Ok(ind) => (ind - 1, ind),
                    Err(ind) => (ind - 1, ind),
                }
            }
        };
        let (x1, x2, y1, y2) = (
            self.x[x1_ind],
            self.x[x2_ind],
            self.y[y1_ind],
            self.y[y2_ind],
        );
        let fq11 = self.surface[x1_ind][y1_ind];
        let fq12 = self.surface[x1_ind][y2_ind];
        let fq21 = self.surface[x2_ind][y1_ind];
        let fq22 = self.surface[x2_ind][y2_ind];

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

        self.cache.borrow_mut().insert(key, z);

        *self.cache.borrow().get(&key).unwrap()
    }
}
