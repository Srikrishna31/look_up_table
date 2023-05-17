use num::Float;
use std::cell::RefCell;
use std::collections::HashMap;

type Key = ((u64, i16, i8), (u64, i16, i8));

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
        Ok(TwoDLookUpTable {
            x,
            y,
            surface,
            cache: RefCell::new(HashMap::new()),
        })
    }

    pub fn get(x: f64, y: f64) -> f64 {
        0.0
    }
}
