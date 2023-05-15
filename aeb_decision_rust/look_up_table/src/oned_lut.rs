use std::ops::Index;

pub struct OneDLookUpTable<const N: usize> {
    x: [f64; N],
    y: [f64; N],
}

impl <const N: usize> OneDLookUpTable<N> {
    pub fn new(x: [f64; N], y: [f64; N]) -> Result<OneDLookUpTable<N>, String> {
        if !x.chunks(2).all(|i, j| j - i > std::f64::consts::E) {
            return Err("X values should be in strictly increasing order".to_string());
        }

        Ok(OneDLookUpTable{x,y})
    }
}

impl <const N: usize> Index<f64> for OneDLookUpTable<N> {
    type Output = f64;

    fn index(&self, index: f64) -> &Self::Output {
        if index < x[0]
    }
}

