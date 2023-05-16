//! Look Up Table
//! Look up tables mimic the evaluation of functions at different points. If a function is complicated
//! or time taking to evaluate at any given point, one can use a Look Up Table, which contains the
//! pre-evaluated sample points of the function.
//! Later when an evaluation is requested, if the sample is directly found, the corresponding value is
//! returned. On the other hand, if the value is not found, then an interpolation (most of the times
//! it is linear to get better performance) of the values is performed to get an estimate of the
//! actual function. In practice this gives a reasonable approximation of the function.
//! Ofcourse when the values are out of bounds, then the last values are returned always.
use num::Float;
use std::cell::RefCell;
use std::collections::HashMap;

type Key = (u64, i16, i8);

const EPSILON: f64 = 0.00000001;

/// Linear interpolation with nearest neighbor extrapolation when index is outside support region.
pub struct OneDLookUpTable<const N: usize> {
    x: [f64; N],
    y: [f64; N],
    cache: RefCell<HashMap<Key, f64>>,
}

impl<const N: usize> OneDLookUpTable<N> {
    pub fn new(x: [f64; N], y: [f64; N]) -> Result<OneDLookUpTable<N>, String> {
        if !x.windows(2).all(|c| c[1] - c[0] > EPSILON) {
            return Err("X values should be in strictly increasing order".to_string());
        }

        Ok(OneDLookUpTable {
            x,
            y,
            cache: RefCell::new(HashMap::new()),
        })
    }

    pub fn get(&self, index: f64) -> f64 {
        // Due to index traits requirements of returning references, we cannot use it to overload.
        if index < self.x[0] {
            return self.y[0];
        }

        if index > self.x[N - 1] {
            return self.y[N - 1];
        }

        let ind = index.integer_decode();
        if self.cache.borrow().contains_key(&ind) {
            return *self.cache.borrow().get(&ind).unwrap();
        }

        let lub = match self
            .x
            .binary_search_by(|val| val.partial_cmp(&index).unwrap())
        {
            // perform interpolation only when the value is not found.
            Ok(ind) => return self.y[ind],
            Err(ind) => ind,
        };
        let prev = lub - 1;
        let alpha = (index - self.x[prev]) / (self.x[lub] - self.x[prev]);

        let y1 = &self.y[prev];
        let y2 = &self.y[lub];
        let y = y1 + alpha * (y2 - y1);

        self.cache.borrow_mut().insert(ind, y);

        *self.cache.borrow().get(&ind).unwrap()
    }
}
