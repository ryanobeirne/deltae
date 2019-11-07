//! Manipulate and convert CIE L\*a\*b\* and Lch colors.
//!
//! # Examples
//!
//! ```
//! use deltae::*;
//! use std::str::FromStr;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let lab0 = LabValue::from_str("95.08, -0.17, -10.81")?;
//!     let lch0 = LchValue {
//!         l: 95.08,
//!         c: 10.811337,
//!         h: 269.09903,
//!     };
//!
//!     assert_eq!(lab0, lch0);
//!
//!     let lch0 = LchValue::from(lab0);
//!     let lab2 = LabValue::from(lch0);
//!
//!     println!("{}", lch0); // [L:89.73, c:7.2094, h:285.1157]
//!
//!     assert_eq!(lab0.round_to(4), lab2.round_to(4));
//!
//!     Ok(())
//! }
//! ```

use super::*;
use super::validate::*;
use std::fmt;
use std::error::Error;
use std::convert::TryFrom;

/// # CIEL\*a\*b\*
///
/// | `Value` | `Color`               | `Range`          |
/// |:-------:|:---------------------:|:----------------:|
/// | `L*`    | `Light <---> Dark`    | `0 <---> 100`    |
/// | `a*`    | `Green <---> Magenta` | `-128 <---> 128` |
/// | `b*`    | `Blue  <---> Yellow`  | `-128 <---> 128` |
///
#[derive(Debug, Clone, Copy)]
pub struct LabValue {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl LabValue {
    /// New `LabValue` from 3 `f32`s
    pub fn new(l: f32, a: f32, b: f32) -> ValueResult<LabValue> {
        LabValue {l, a, b}.validate()
    }

    /// Round `LabValue` to nearest decimal places.
    pub fn round_to(&self, places: i32) -> LabValue {
        LabValue {
            l: round_to(self.l, places),
            a: round_to(self.a, places),
            b: round_to(self.b, places),
        }
    }
}

impl Default for LabValue {
    fn default() -> LabValue {
        LabValue { l: 0.0, a: 0.0, b: 0.0 }
    }
}

impl fmt::Display for LabValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[L:{}, a:{}, b:{}]", self.l, self.a, self.b)
    }
}

/// # Lch: Luminance, Chroma, Hue
///
/// | `Value` | `Color`                    | `Range`            |
/// |:-------:|:--------------------------:|:------------------:|
/// | `L*`    | `Light <---> Dark`         | `0 <---> 100`      |
/// | `c`     | `Chroma (Amount of color)` | `0 <---> 181.0139` |
/// | `h`     | `Hue (Degrees)`            | `0 <---> 360°`     |
///
#[derive(Debug, Clone, Copy)]
pub struct LchValue {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

impl LchValue {
    pub fn new(l: f32, c: f32, h: f32) -> ValueResult<LchValue> {
        LchValue::try_from(&[l, c, h])
    }

    /// Round `LchValue` to nearest decimal places.
    pub fn round_to(&self, places: i32) -> LchValue {
        LchValue {
            l: round_to(self.l, places),
            c: round_to(self.c, places),
            h: round_to(self.h, places),
        }
    }

    /// Returns an array of [L, c, h]
    pub fn to_a(&self) -> [f32; 3] {
        [self.l, self.c, self.h]
    }

    /// Returns a `Vec<f32>` of [L, c, h]
    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.l, self.c, self.h]
    }

    pub fn hue_radians(&self) -> f32 {
        self.h.to_radians()
    }
}

impl Default for LchValue {
    fn default() -> LchValue {
        LchValue { l: 0.0, c: 0.0, h: 0.0 }
    }
}

impl fmt::Display for LchValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[L:{}, c:{}, h:{}]", self.l, self.c, self.h)
    }
}

/// # XYZ
///
/// | `Value` | `Color` | `Range`     |
/// |:-------:|:-------:|:-----------:|
/// | `X`     | `Red`   | `0 <---> 1` |
/// | `Y`     | `Green` | `0 <---> 1` |
/// | `Z`     | `Blue`  | `0 <---> 1` |
///
#[derive(Debug, Clone, Copy)]
pub struct XyzValue {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XyzValue {
    pub fn new(x: f32, y: f32, z:f32) -> ValueResult<XyzValue> {
        XyzValue {x, y, z}.validate()
    }

    pub fn round_to(&self, places: i32) -> XyzValue {
        XyzValue {
            x: round_to(self.x, places),
            y: round_to(self.y, places),
            z: round_to(self.z, places),
        }
    }
}

impl Default for XyzValue {
    fn default() -> XyzValue {
        XyzValue { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl fmt::Display for XyzValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[X:{}, Y:{}, Z:{}]", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
pub enum ValueError {
    OutOfBounds,
    BadFormat,
}

pub type ValueResult<T> = Result<T, Box<dyn Error>>;

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ValueError {
    fn description(&self) -> &str {
        match self {
            ValueError::OutOfBounds => "Value is out of range!",
            ValueError::BadFormat   => "Value is malformed!",
        }
    }
}
