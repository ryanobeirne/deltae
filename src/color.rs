//! Manipulate and convert CIE L\*a\*b\* and Lch colors.
//! 
//! ### Lab
//! 
//! * **L**: Lightness     (0...100)
//! * **a**: green-magenta (-128...128)
//! * **b**: blue-yellow   (-128...128)
//! 
//! ### Lch
//! 
//! * **L**: Lightness (0...100)
//! * **c**: Chroma    (0...181.0139)
//! * **h**: Hue       (0...360°)
//!     
//! # Examples
//! 
//! ```
//! extern crate deltae;
//! use deltae::color::{LabValue, LchValue};
//! use std::str::FromStr;
//! 
//! fn main() {
//!     let lab0 = LabValue::from_str("95.08, -0.17, -10.81").unwrap();
//!     let lab1 = LabValue {
//!         l: 95.08,
//!         a: -0.17,
//!         b: -10.81,
//!     };
//! 
//!     assert_eq!(lab0, lab1);
//! 
//!     let lch0 = lab0.to_lch();
//!     let lab2 = lch0.to_lab();
//! 
//!     println!("{}", lch0); // [L:89.73, c:7.2094, h:285.1157]
//! 
//!     assert_eq!(lab0.round_to(4), lab2.round_to(4));
//! }
//! ```

use super::*;
use std::fmt;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct LabValue {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl LabValue {
    pub fn new(l: f64, a: f64, b: f64) -> ValueResult<LabValue> {
        //! New `LabValue` from 3 `f64`s
        LabValue {l, a, b}.validate()
    }

    fn validate(self) -> ValueResult<LabValue> {
        // Check that the Lab values are in the proper range or Error
        if  self.l < 0.0    || self.l > 100.0 ||
            self.a < -128.0 || self.a > 128.0 ||
            self.b < -128.0 || self.b > 128.0
        {
            Err(ValueError::OutOfBounds)
        } else {
            Ok(self)
        }
    }


    pub fn to_lch(&self) -> LchValue {
        //! Convert `LabValue` to `LchValue`
        LchValue {
            l: self.l,
            c: ( self.a.powi(2) + self.b.powi(2) ).sqrt(),
            h: get_h_prime(self.a, self.b),
        }
    }

    pub fn chroma(&self) -> f64 {
        self.to_lch().c
    }

    pub fn hue(&self) -> f64 {
        self.to_lch().h
    }

    pub fn hue_radians(&self) -> f64 {
        self.to_lch().h.to_radians()
    }

    pub fn to_xyz(&self) -> XyzValue {
        lab_to_xyz(self)
    }

    pub fn round_to(&self, places: i32) -> LabValue {
        //! Round `LabValue` to nearest decimal places.
        LabValue {
            l: round_to(self.l, places),
            a: round_to(self.a, places),
            b: round_to(self.b, places),
        }       
    }

    pub fn to_a(&self) -> [f64; 3] {
        //! Returns an array of [L, a, b]
        [self.l, self.a, self.b]
    }

    pub fn to_vec(&self) -> Vec<f64> {
        //! Returns a `Vec<f64>` of [L, a, b]
        vec![self.l, self.a, self.b]
    }
}

impl Default for LabValue {
    fn default() -> LabValue {
        LabValue { l: 0.0, a: 0.0, b: 0.0 }
    }
}

impl FromStr for LabValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<LabValue> {
        //! New `LabValue` from `&str`
        let split = parse_str_to_vecf64(s, 3)?;

        LabValue {
            l: split[0],
            a: split[1],
            b: split[2],
        }.validate()
    }
}

impl fmt::Display for LabValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[L:{}, a:{}, b:{}]", self.l, self.a, self.b)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LchValue {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl LchValue {
    pub fn new(l: f64, c: f64, h: f64) -> ValueResult<LchValue> {
        //! New `LchValue` from 3 `f64`s
        LchValue {l, c, h}.validate()
    }

    fn validate(self) -> ValueResult<LchValue> {
        // Check that the Lab values are in the proper range or Error
        if  self.l < 0.0 || self.l > 100.0 ||
            self.c < 0.0 || self.c > (128_f64.powi(2) + 128_f64.powi(2)).sqrt() ||
            self.h < 0.0 || self.h > 360.0
        {
            Err(ValueError::OutOfBounds)
        } else {
            Ok(self)
        }
    }

    pub fn to_lab(&self) -> LabValue {
        //! Convert `LchValue` to `LabValue`
        LabValue {
            l: self.l,
            a: self.c * self.h.to_radians().cos(),
            b: self.c * self.h.to_radians().sin(),
        }
    }

    pub fn round_to(&self, places: i32) -> LchValue {
        //! Round `LchValue` to nearest decimal places.
        LchValue {
            l: round_to(self.l, places),
            c: round_to(self.c, places),
            h: round_to(self.h, places),
        }       
    }

    pub fn to_a(&self) -> [f64; 3] {
        //! Returns an array of [L, c, h]
        [self.l, self.c, self.h]
    }

    pub fn to_vec(&self) -> Vec<f64> {
        //! Returns a `Vec<f64>` of [L, c, h]
        vec![self.l, self.c, self.h]
    }
}

impl Default for LchValue {
    fn default() -> LchValue {
        LchValue { l: 0.0, c: 0.0, h: 0.0 }
    }
}

impl FromStr for LchValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<LchValue> {
        //! New `LchValue` from `&str`
        let split = parse_str_to_vecf64(s, 3)?;

        LchValue {
            l: split[0],
            c: split[1],
            h: split[2],
        }.validate()
    }
}

impl fmt::Display for LchValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[L:{}, c:{}, h:{}]", self.l, self.c, self.h)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct XyzValue {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XyzValue {
    pub fn new(x: f64, y: f64, z:f64) -> ValueResult<XyzValue> {
        //! New `XyzValue` from 3 `f64`s
        XyzValue { x, y, z}.validate()
    }

    pub fn to_lab(&self) -> LabValue {
        xyz_to_lab([self.x, self.y, self.z])
    }

    pub fn round_to(&self, places: i32) -> XyzValue {
        XyzValue {
            x: round_to(self.x, places),
            y: round_to(self.y, places),
            z: round_to(self.z, places),
        }
    }

    fn validate(self) -> ValueResult<XyzValue> {
        // Check that the XYZ values are in the proper range or Error
        if self.x < 0.0 || self.x > 1.0 ||
        self.y < 0.0 || self.y > 1.0 ||
        self.z < 0.0 || self.z > 1.0
        {
            Err(ValueError::OutOfBounds)
        } else {
            Ok(self)
    }
}

}

impl Default for XyzValue {
    fn default() -> XyzValue {
        XyzValue { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl FromStr for XyzValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<XyzValue> {
        //! New `XyzValue` from `&str`
        let split = parse_str_to_vecf64(s, 3)?;

        XyzValue {
            x: split[0],
            y: split[1],
            z: split[2],
        }.validate()
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

pub type ValueResult<T> = Result<T, ValueError>;

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

fn parse_str_to_vecf64(s: &str, length: usize) -> ValueResult<Vec<f64>> {
    // Validate and convert strings to `LabValue`.
    // Split string by comma (92.5,33.5,-18.8).
    let collection: Vec<&str> = s.split(",").collect();
    
    // Allow extraneous whitespace ("92.5, 33.5, -18.8")
    let mut v: Vec<&str> = Vec::new();
    for item in collection.iter() {
        if !item.is_empty() {
            v.push(item.trim());
        }
    }
    // Parse the f64's into a Vec
    let split: Vec<f64> = v.iter().filter_map(|s| s.parse().ok()).collect();

    // Check if it's the right number of items
    if v.len() != length || split.len() != length {
        return Err(ValueError::BadFormat);
    }

    Ok(split)
}

const KAPPA: f64 = 24389.0 / 27.0;
const EPSILON: f64 = 216.0 / 24389.0;
const CBRT_EPSILON: f64 = 0.20689655172413796;

fn lab_to_xyz(lab: &LabValue) -> XyzValue {
    let fy = (lab.l + 16.0) / 116.0;
    let fx = (lab.a / 500.0) + fy;
    let fz = fy - (lab.b / 200.0);
    let xr = if fx > CBRT_EPSILON {
        fx.powi(3)
    } else {
        ((fx * 116.0) - 16.0) / KAPPA
    };
    let yr = if lab.l > EPSILON * KAPPA {
        fy.powi(3)
    } else {
        lab.l / KAPPA
    };
    let zr = if fz > CBRT_EPSILON {
        fz.powi(3)
    } else {
        ((fz * 116.0) - 16.0) / KAPPA
    };

    XyzValue {
        x: xr * 0.95047,
        y: yr,
        z: zr * 1.08883,
    }
}

#[inline]
fn xyz_to_lab_map(c: f64) -> f64 {
    if c > EPSILON {
        c.powf(1.0/3.0)
    } else {
        (KAPPA * c + 16.0) / 116.0
    }
}

fn xyz_to_lab(xyz: [f64; 3]) -> LabValue {
    let x = xyz_to_lab_map(xyz[0] / 0.95047);
    let y = xyz_to_lab_map(xyz[1]);
    let z = xyz_to_lab_map(xyz[2] / 1.08883);

    LabValue {
        l: (116.0 * y) - 16.0,
        a: 500.0 * (x - y),
        b: 200.0 * (y - z),
    }
}