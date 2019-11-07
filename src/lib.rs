//! Calculate Delta E (color difference) between two colors in CIE Lab space.
//!
//! # Examples
//!
//! ```
//! use deltae::*;
//! use std::str::FromStr;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let lab0 = LabValue::from_str("89.73, 1.88, -6.96")?;
//!     let lab1 = LabValue {
//!         l: 95.08,
//!         a: -0.17,
//!         b: -10.81,
//!     }.validate()?;
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

#[test]
fn derp() {
    //use std::convert::TryFrom;
    let lab = LabValue {l: 95.08, a: -0.17, b: -10.81}.validate().unwrap();
    let lch = LchValue::from(lab);
    let de = lab.delta(lch, DE1976);
    dbg!(lab, lch, LabValue::from(lch), de);
    assert_eq!(lab, lch);
    //let xyz = XyzValue::try_from(&[0.5, 0.5, 0.5]).unwrap();
    //let de  = lch.delta(xyz, DE2000);
    //dbg!(lch, xyz, de);
}

mod convert;
mod eq;
pub mod color;
mod validate;
mod delta;

pub use delta::*;
pub use validate::*;
pub use DEMethod::*;
pub use color::*;

use std::fmt;
use std::io;

#[cfg(test)]
mod tests;

/// ## The measured difference between two colors
///
/// There are many different methods of calculating color difference.
///
#[derive(Debug, PartialEq, Clone)]
pub struct DeltaE {
    pub method: DEMethod,
    pub value: f32,
}

impl DeltaE {
    /// New `DeltaE` from `LabValues` and `DEMethod`.
    pub fn new<A, B>(a: A, b: B, method: DEMethod) -> DeltaE
    where A: Into<LabValue>, B: Into<LabValue> {
        a.delta(b, method)
    }

    /// Round `DeltaE` value and its components to nearest decimal places
    pub fn round_to(mut self, places: i32) -> Self {
        self.value = round_to(self.value, places);
        self
    }
}

impl fmt::Display for DeltaE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl PartialOrd for DeltaE {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

// Round an f32 to a number of decimal places
fn round_to(val: f32, places: i32) -> f32 {
    let mult = 10_f32.powi(places);
    (val * mult).round() / mult
}

/// The multiple DeltaE methods are used for different purposes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DEMethod{
    /// The default DeltaE method
    DE2000,
    /// CIE94 DeltaE implementation, weighted with or without a tolerance for textiles
    DE1994(bool),
    /// The original DeltaE implementation, a basic euclidian distance formula
    DE1976,
    /// An implementation of DeltaE with tolerances for Lightness and Chroma
    DECMC(f32, f32),
}

/// DeltaE 1994 Textiles
pub const DE1994T: DEMethod = DE1994(true);
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
            DE1994(textiles) => match textiles {
                true  => write!(f, "DE1994T"),
                false => write!(f, "DE1994"),
            }
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

