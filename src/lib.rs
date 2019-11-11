#![warn(missing_docs)]
//! Calculate Delta E (color difference) between two colors in CIE Lab space.
//!
//! # Examples
//!
//! ```
//! use deltae::*;
//! use std::str::FromStr;
//! use std::error::Error;
//! use std::convert::TryFrom;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let lab0 = LabValue::from_str("89.73, 1.88, -6.96")?;
//!     let lab1 = LabValue {
//!         l: 95.08,
//!         a: -0.17,
//!         b: -10.81,
//!     }.validate()?;
//!     let lab2 = LabValue::try_from(&[89.73, 1.88, -6.96])?;
//!     assert_eq!(lab0, lab2);
//!
//!     println!("{}", &lab0); // [L:89.73, a:1.88, b:-6.96]
//!
//!     let de0 = DeltaE::new(&lab0, &lab1, DE2000);
//!     let de1 = lab0.delta(&lab1, DE2000);
//!     assert_eq!(de0, de1); // DE2000: 5.3169
//!
//!     Ok(())
//! }
//! ```

mod color;
mod convert;
mod delta;
mod eq;
mod round;
mod validate;

#[cfg(test)]
mod tests;

pub use DEMethod::*;
pub use color::*;
pub use delta::*;
pub use round::*;
pub use validate::*;

use std::fmt;
use std::io;

pub(crate) type ValueResult<T> = Result<T, color::ValueError>;

/// ## The measured difference between two colors
///
/// There are many different methods of calculating color difference.
/// Different methods have a specific purpose, mainly in determining the level
/// of tolerance for describing die difference between two colors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DeltaE {
    /// The mathematical method used for calculating color difference
    pub method: DEMethod,
    /// The calculated value
    pub value: f32,
}

impl DeltaE {
    /// New `DeltaE` from colors and `DEMethod`.
    pub fn new<A, B>(a: A, b: B, method: DEMethod) -> DeltaE
    where A: Delta, B: Delta {
        a.delta(b, method)
    }
}

impl fmt::Display for DeltaE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl PartialEq<f32> for DeltaE {
    fn eq(&self, f: &f32) -> bool {
        self.value == *f
    }
}

/// One should be careful when ordering DeltaE. A `DE2000:1.0` value is not
/// necessarily the same amount of color difference as a amount of color
/// difference `DE1976:1.0` value.
impl PartialOrd for DeltaE {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// The most common DeltaE methods
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DEMethod{
    /// The default DeltaE method
    DE2000,
    /// An implementation of DeltaE with tolerances for Lightness and Chroma
    DECMC(f32, f32),
    /// CIE94 DeltaE implementation, weighted with a tolerance for graphics
    DE1994G,
    /// CIE94 DeltaE implementation, weighted with a tolerance for textiles
    DE1994T,
    /// The original DeltaE implementation, a basic euclidian distance formula
    DE1976,
}

/// DeltaE CMC (1:1)
pub const DECMC1: DEMethod = DECMC(1.0, 1.0);
/// DeltaE CMC (2:1)
pub const DECMC2: DEMethod = DECMC(2.0, 1.0);

impl Eq for DEMethod {}

impl Default for DEMethod {
    fn default() -> DEMethod {
        DEMethod::DE2000
    }
}

impl fmt::Display for DEMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DECMC(tl, tc) => {
                if (tl, tc) == (&1.0, &1.0) {
                    write!(f, "DECMC1")
                } else if (tl, tc) == (&2.0, &1.0) {
                    write!(f, "DECMC2")
                } else {
                    write!(f, "DECMC({:0.2}:{:0.2})", tl, tc)
                }
            }
            _ => write!(f, "{:?}", self)
        }
    }
}

