use num::Float;
use std::cell::RefCell;
use std::collections::HashMap;

type Key = ((u64, i16, i8), (u64, i16, i8));
const EPSILON: f64 = 0.00000001;

#[derive(Debug)]
pub struct TwoDLookUpTable<const N: usize> {
    x: [f64; N],
    y: [f64; N],
    surface: [[f64; N]; N],
    cache: RefCell<HashMap<Key, f64>>,
}

impl<const N: usize> TwoDLookUpTable<N> {
    pub fn new(
        x: [f64; N],
        y: [f64; N],
        surface: [[f64; N]; N],
    ) -> Result<TwoDLookUpTable<N>, String> {
        // TODO: explore if this constraint can be expressed in generics to move this check to compile time
        if N < 2 {
            return Err("At least two values should be provided".to_string());
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

    fn check_nans_and_infinities(x: &[f64; N], y: &[f64; N], surface: &[[f64; N]; N]) -> bool {
        x.iter().any(|v| v.is_nan() || v.is_infinite())
            || y.iter().any(|v| v.is_nan() || v.is_infinite())
            || surface
                .iter()
                .any(|row| row.iter().any(|v| v.is_nan() || v.is_infinite()))
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        // if one of the indices is out of range, then perform interpolation only in that direction.
        0.0
    }
}
