use std::collections::BTreeMap;
use std::ops::Index;

const EPSILON: f64 = 0.00000001;
pub struct OneDLookUpTable<const N: usize> {
    function_map: BTreeMap<f64, f64>,
    x: [f64; N]
}

impl<const N: usize> OneDLookUpTable<N> {
    pub fn new(x: [f64; N], y: &[f64; N]) -> Result<OneDLookUpTable<N>, String> {
        if !x.windows(2).all(|c| c[1] - c[0] > EPSILON) {
            return Err("X values should be in strictly increasing order".to_string());
        }

        Ok(OneDLookUpTable {
            function_map: BTreeMap::from_iter(*x.iter().zip(y.iter())),
            x
        })
    }
}

impl<const N: usize> Index<f64> for OneDLookUpTable<N> {
    type Output = f64;

    fn index(&self, index: f64) -> &Self::Output {
        if index < self.x[0] {
            return &self.function_map.get(&self.x[0]).unwrap();
        }

        if index > self.x[N] {
            return &self.y[N];
        }

        let lub = match self
            .x
            .binary_search_by(|val| val.partial_cmp(&index).unwrap())
        {
            Ok(ind) => ind,
            Err(ind) => ind,
        };
        let prev = lub - 1;
        let alpha = (index - self.x[prev]) / (self.x[lub] - self.x[prev]);

        let y1 = *self.function_map.get(&self.x[prev]).unwrap();
        let y2= *self.function_map.get(&self.x[lub]).unwrap();

        &(y1 + alpha * (y2 - y1))
    }
}
