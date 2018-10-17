//! Manipulate and convert CIE L\*a\*b\* and Lch colors.
//! 
//! Lab:
//! * L: Lightness     (0...100)
//! * a: green-magenta (-128...128)
//! * b: blue-yellow   (-128...128)
//! 
//! Lch:
//! * L: Lightness (0...100)
//! * c: Chroma    (0...181.0139)
//! * h: Hue       (0...360°)
//!     
//! # Examples
//! 
//! ```
//! extern crate deltae;
//! use deltae::color::{LabValue, LchValue};
//! 
//! fn main() {
//!     let lab0 = LabValue::from("95.08, -0.17, -10.81").unwrap();
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

#[derive(Debug, PartialEq, Clone)]
pub struct LabValue {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl LabValue {
    pub fn new(l: f64, a: f64, b: f64) -> ValueResult<Self> {
        //! New `LabValue` from 3 `f64`s
        let lab = Self{l, a, b};
        validate_lab(lab)
    }

    pub fn zero() -> Self {
        //! New `LabValue` with a value of 0,0,0.
        Self { l: 0.0, a: 0.0, b: 0.0 }
    }

    pub fn from(lab_string: &str) -> ValueResult<Self> {
        //! New `LabValue` from `&str`
        let split = parse_str_to_vecf64(lab_string, 3)?;

        let lab = Self {
            l: split[0],
            a: split[1],
            b: split[2],
        };

        validate_lab(lab)
    }

    pub fn to_lch(&self) -> LchValue {
        //! Convert `LabValue` to `LchValue`
        LchValue {
            l: self.l,
            c: ( self.a.powi(2) + self.b.powi(2) ).sqrt(),
            h: get_h_prime(self.a, self.b),
        }
    }

    pub fn round_to(&self, places: i32) -> LabValue {
        //! Round `LabValue` to nearest decimal places.
        Self {
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
    pub fn new(l: f64, c: f64, h: f64) -> ValueResult<Self> {
        //! New `LchValue` from 3 `f64`s
        let lch = Self {l, c, h};
        validate_lch(lch)
    }

    pub fn zero() -> Self {
        //! New `LchValue` with a value of 0,0,0
        Self { l: 0.0, c: 0.0, h: 0.0 }
    }

    pub fn from(lch_string: &str) -> ValueResult<Self> {
        //! New `LchValue` from `&str`
        let split = parse_str_to_vecf64(lch_string, 3)?;

        let lch = Self {
            l: split[0],
            c: split[1],
            h: split[2],
        };

        validate_lch(lch)
    }

    pub fn to_lab(&self) -> LabValue {
        //! Convert `LchValue` to `LabValue`
        LabValue {
            l: self.l,
            a: self.c * self.h.to_radians().cos(),
            b: self.c * self.h.to_radians().sin(),
        }
    }

    pub fn round_to(&self, places: i32) -> Self {
        //! Round `LchValue` to nearest decimal places.
        Self {
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

impl fmt::Display for LchValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[L:{}, c:{}, h:{}]", self.l, self.c, self.h)
    }
}

#[derive(Debug)]
pub enum ValueError {
    OutOfBounds,
    BadFormat,
}

type ValueResult<T> = Result<T, ValueError>;

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

fn parse_str_to_vecf64(s: &str, length: usize) -> Result<Vec<f64>, ValueError> {
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

fn validate_lab(lab: LabValue) -> ValueResult<LabValue> {
    // Check that the Lab values are in the proper range or Error
    if  lab.l < 0.0    || lab.l > 100.0 ||
        lab.a < -128.0 || lab.a > 128.0 ||
        lab.b < -128.0 || lab.b > 128.0
    {
        Err(ValueError::OutOfBounds)
    } else {
        Ok(lab)
    }
}

fn validate_lch(lch: LchValue) -> ValueResult<LchValue> {
    // Check that the Lab values are in the proper range or Error
    if  lch.l < 0.0 || lch.l > 100.0 ||
        lch.c < 0.0 || lch.c > (128_f64.powi(2) + 128_f64.powi(2)).sqrt() ||
        lch.h < 0.0 || lch.h > 360.0
    {
        Err(ValueError::OutOfBounds)
    } else {
        Ok(lch)
    }
}
