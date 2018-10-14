use super::round_to;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct LabValue {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LchValue {
    pub l: f64,
    pub c: f64,
    pub h: f64,
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

impl LabValue {
    pub fn zero() -> LabValue {
        //! New LabValue with a value of 0,0,0.
        LabValue { l: 0.0, a: 0.0, b: 0.0 }
    }

    pub fn from(s: &str) -> ValueResult<LabValue> {
        //! Parse LabValue from &str
        Ok(string_to_lab(s)?)
    }

    pub fn to_lch(&self) -> LchValue {
        //! Convert Lab to Lch.
        let mut h: f64 = self.b.atan2(self.a).to_degrees();

        if h < 0.0 {
            h += 360.0;
        };

        LchValue {
            l: self.l,
            c: ( self.a.powi(2) + self.b.powi(2) ).sqrt(),
            h: h
        }
    }

    pub fn round_to(&self, places: i32) -> LchValue {
        //! Round LchValue to nearest decimal places.
        LchValue {
            l: round_to(self.l, places),
            c: round_to(self.a, places),
            h: round_to(self.b, places),
        }       
    }
}

impl LchValue {
    pub fn zero() -> LchValue {
        //! New LchValue with a value of 0,0,0
        LchValue { l: 0.0, c: 0.0, h: 0.0 }
    }

    pub fn from(s: &str) -> ValueResult<LchValue> {
        //! Parse LchValue from &str
        Ok(string_to_lch(s)?)
    }

    pub fn to_lab(&self) -> LabValue {
        //! Convert LchValue to LabValue
        LabValue {
            l: self.l,
            a: self.c * self.h.to_radians().cos(),
            b: self.c * self.h.to_radians().sin(),
        }
    }

    pub fn round_to(&self, places: i32) -> LchValue {
        //! Round LchValue to nearest decimal places.
        LchValue {
            l: round_to(self.l, places),
            c: round_to(self.c, places),
            h: round_to(self.h, places),
        }       
    }
}

fn string_to_lab(lab_string: &str) -> ValueResult<LabValue> {
    //! Validate and convert strings to LabValue.
    //! Split string by comma (92.5,33.5,-18.8).
    let s = lab_string.split(",");
    let st: Vec<&str> = s.clone().collect();
    let split: Vec<f64> = s.filter_map(|s| s.parse().ok()).collect();

    // Validate that there are only 3 values
    if st.len() != 3 || split.len() != 3 {
        return Err(ValueError::BadFormat);
    };

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
    //! Validate and convert strings to LchValue.
    //! Split string by comma (92.5,153.2,240.3).
    let s = lch_string.split(",");
    let st: Vec<&str> = s.clone().collect();
    let split: Vec<f64> = s.filter_map(|s| s.parse().ok()).collect();

    // Validate that there are only 3 values
    if st.len() != 3 || split.len() != 3 {
        return Err(ValueError::BadFormat);
    };

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
