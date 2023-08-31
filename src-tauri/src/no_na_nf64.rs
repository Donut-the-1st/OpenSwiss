//Modified from Shepmaster, https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats

use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NaNError;

impl fmt::Display for NaNError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tried to conform NaN to NoNaN")
    }
}

#[derive(PartialEq, PartialOrd)]
pub struct NoNaNf64(f64);

impl NoNaNf64 {
    pub(crate) fn new(val: f64) -> Result<NoNaNf64, NaNError> {
        if val.is_nan() {
            Err(NaNError)
        } else {
            Ok(NoNaNf64(val))
        }
    }
}

impl Eq for NoNaNf64 {}

impl Ord for NoNaNf64 {
    fn cmp(&self, other: &NoNaNf64) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
