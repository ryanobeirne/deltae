use super::round_to;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct LabValue {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl LabValue {
    pub fn new(l: f64, a: f64, b: f64) -> Self {
        //! New `LabValue` from 3 `f64`s
        Self{l, a, b}
    }

    pub fn zero() -> Self {
        //! New `LabValue` with a value of 0,0,0.
        Self { l: 0.0, a: 0.0, b: 0.0 }
    }

    pub fn from(s: &str) -> ValueResult<Self> {
        //! Parse `LabValue` from `&str`
        Ok(string_to_lab(s)?)
    }

    pub fn to_lch(&self) -> LchValue {
        //! Convert Lab to Lch
        let mut h: f64 = (self.b.atan2(self.a)).to_degrees();

        if h < 0.0 {
            h += 360.0;
        };

        LchValue {
            l: self.l,
            c: ( self.a.powi(2) + self.b.powi(2) ).sqrt(),
            h: h
        }
    }

    pub fn round_to(&self, places: i32) -> LabValue {
        //! Round LchValue to nearest decimal places.
        Self {
            l: round_to(self.l, places),
            a: round_to(self.a, places),
            b: round_to(self.b, places),
        }       
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
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        //! New `LchValue` from 3 `f64`s
        Self {l, c, h}
    }

    pub fn zero() -> Self {
        //! New LchValue with a value of 0,0,0
        Self { l: 0.0, c: 0.0, h: 0.0 }
    }

    pub fn from(s: &str) -> ValueResult<Self> {
        //! Parse `LchValue` from `&str`
        Ok(string_to_lch(s)?)
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

    fn cause(&self) -> Option<&Error> {
        Some(self)
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

fn string_to_lab(lab_string: &str) -> ValueResult<LabValue> {
    let split = parse_str_to_vecf64(lab_string, 3)?;

    // Check that the Lab values are in the proper range or Error
    if  split[0] < 0.0    || split[0] > 100.0 ||
        split[1] < -128.0 || split[1] > 128.0 ||
        split[2] < -128.0 || split[2] > 128.0
    {
        return Err(ValueError::OutOfBounds);
    };

    let lab = LabValue {
        l: split[0],
        a: split[1],
        b: split[2],
    };

    Ok(lab)
}

fn string_to_lch(lch_string: &str) -> ValueResult<LchValue> {
    let split = parse_str_to_vecf64(lch_string, 3)?;

    // Check that the Lab values are in the proper range or Error
    if  split[0] < 0.0 || split[0] > 100.0 ||
        split[1] < 0.0 || split[1] > (128_f64.powi(2) + 128_f64.powi(2)).sqrt() ||
        split[2] < 0.0 || split[2] > 360.0
    {
        return Err(ValueError::OutOfBounds);
    };

    let lch = LchValue {
        l: split[0],
        c: split[1],
        h: split[2],
    };

    Ok(lch)
}
