//! Calculate Delta E (color difference) between two colors in CIE Lab space.
//! 
//! # Examples
//! 
//! ```
//! extern crate deltae;
//! use deltae::{DeltaE, DEMethod::DE2000};
//! use deltae::color::LabValue;
//! use std::str::FromStr;
//!
//! fn main() {
//!     let lab0 = LabValue::from_str("89.73, 1.88, -6.96").unwrap();
//!     let lab1 = LabValue {
//!         l: 95.08,
//!         a: -0.17,
//!         b: -10.81,
//!     };
//!
//!     println!("{}", lab0); // [L:89.73, a:1.88, b:-6.96]
//!
//!     let de0 = DeltaE::new(&lab0, &lab1, DE2000).round_to(4);
//!
//!     println!("{}: {}", de0.method, de0.value); // DE2000: 5.3169
//!
//!     let de1 = DeltaE::from(
//!         "89.73, 1.88, -6.96",
//!         "95.08, -0.17, -10.81",
//!         "DE2000"
//!     ).unwrap();
//!
//!     assert_eq!(de0, de1.round_to(4));
//!
//!     let de3 = lab0.deltae(&lab1, DE2000);
//!     assert_eq!(de0, de3.round_to(4));
//!
//!     let lch0 = lab0.to_lch();
//!     let lab2 = lch0.to_lab();
//!
//!     println!("{}", lch0); // [L:89.73, c:7.2094, h:285.1157]
//!
//!     assert_eq!(lab0.round_to(4), lab2.round_to(4));
//! }
//! ```

use std::fmt;
use std::io;
pub use std::str::FromStr;
pub use DEMethod::*;

pub mod color;
use color::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct DeltaE {
    pub method: DEMethod,
    pub value: f32,
}

impl DeltaE {
    /// New `DeltaE` from `LabValues` and `DEMethod`.
    pub fn new(lab_0: &LabValue, lab_1: &LabValue, method: DEMethod) -> DeltaE {
        let value = match method {
            DEMethod::DE2000  => delta_e_2000(lab_0, lab_1),
            DEMethod::DE1994(textiles)  => delta_e_1994(lab_0, lab_1, textiles),
            DEMethod::DE1976  => delta_e_1976(lab_0, lab_1),
            DEMethod::DECMC(t_l, t_c)  => delta_e_cmc(lab_0, lab_1, t_l, t_c),
        };

        DeltaE { method, value }
    }

    /// Round `DeltaE` value and its components to nearest decimal places
    pub fn round_to(mut self, places: i32) -> Self {
        self.value = round_to(self.value, places);
        self
    }

    /// Parse `DeltaE` from `&str`'s
    pub fn from(color_0: &str, color_1: &str, method: &str) -> ValueResult<DeltaE> {
        let lab_0 = LabValue::from_str(color_0)?;
        let lab_1 = LabValue::from_str(color_1)?;
        let meth = DEMethod::from_str(method)?;

        let de = DeltaE::new(&lab_0, &lab_1, meth);

        Ok(de)
    }
}

impl fmt::Display for DeltaE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

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

impl Eq for DEMethod {}

/// DeltaE 1994 Textiles
pub const DE1994T: DEMethod = DE1994(true);
/// DeltaE CMC (1:1)
pub const DECMC1: DEMethod = DECMC(1.0, 1.0);
/// DeltaE CMC (2:1)
pub const DECMC2: DEMethod = DECMC(2.0, 1.0);

impl Default for DEMethod {
    fn default() -> DEMethod {
        DEMethod::DE2000
    }
}

impl FromStr for DEMethod {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<DEMethod, Self::Err> {
        match s.to_lowercase().trim() {
            "de2000"  | "de00"  | "2000"  | "00"  => Ok(DEMethod::DE2000),
            "de1976"  | "de76"  | "1976"  | "76"  => Ok(DEMethod::DE1976),
            "de1994"  | "de94"  | "1994"  | "94" |
            "de1994g" | "de94g" | "1994g" | "94g" => Ok(DEMethod::DE1994(false)),
            "de1994t" | "de94t" | "1994t" | "94t" => Ok(DEMethod::DE1994(true)),
            "decmc"   | "decmc1"| "cmc1"  | "cmc" => Ok(DEMethod::DECMC(1.0, 1.0)),
            "decmc2"  | "cmc2"                    => Ok(DEMethod::DECMC(2.0, 1.0)),
            _ => Err(io::Error::from(io::ErrorKind::InvalidInput)),
        }
    }
}

impl fmt::Display for DEMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DE1994(true) => write!(f, "DE1994T"),
            DECMC(tl, tc) => write!(f, "DECMC({:0.2}:{:0.2})", tl, tc),
            _ => write!(f, "{:?}", self)
        }
    }
}

/// DeltaE 1976. Basic euclidian distance formula.
fn delta_e_1976(lab_0: &LabValue, lab_1: &LabValue) -> f32 {
    ( (lab_0.l - lab_1.l).powi(2) + (lab_0.a - lab_1.a).powi(2) + (lab_0.b - lab_1.b).powi(2) ).sqrt()
}

/// DeltaE 1994. Weighted for textiles (`true`) or graphics (`false`)
fn delta_e_1994(lab_0: &LabValue, lab_1: &LabValue, textiles: bool) -> f32 {
    let delta_l = lab_0.l - lab_1.l;
    let chroma_0 = (lab_0.a.powi(2) + lab_0.b.powi(2)).sqrt();
    let chroma_1 = (lab_1.a.powi(2) + lab_1.b.powi(2)).sqrt();
    let delta_chroma = chroma_0 - chroma_1;
    let delta_a = lab_0.a - lab_1.a;
    let delta_b = lab_0.b - lab_1.b;
    let delta_hue = (delta_a.powi(2) + delta_b.powi(2) - delta_chroma.powi(2)).sqrt();

    let (kl, k1, k2) = match textiles {
        true  => (2.0, 0.048, 0.014),
        false => (1.0, 0.045, 0.015),
    };

    let s_l = 1.0;
    let s_c = 1.0 + k1 * chroma_0;
    let s_h = 1.0 + k2 * chroma_0;

    (   (delta_l / kl * s_l).powi(2)
      + (delta_chroma / s_c).powi(2)
      + (delta_hue / s_h).powi(2)
    ).sqrt()
}

fn get_h_prime(a: f32, b: f32) -> f32 {
    let h_prime = b.atan2(a).to_degrees();
    if h_prime < 0.0 {
        h_prime + 360.0
    } else {
        h_prime
    }           
}

/// DeltaE 2000. This is a ridiculously complicated formula.
fn delta_e_2000(lab_0: &LabValue, lab_1: &LabValue) -> f32 {
    let chroma_0 = (lab_0.a.powi(2) + lab_0.b.powi(2)).sqrt();
    let chroma_1 = (lab_1.a.powi(2) + lab_1.b.powi(2)).sqrt();

    let c_bar = (chroma_0 + chroma_1) / 2.0;

    let g = 0.5 * (1.0 - ( c_bar.powi(7) / (c_bar.powi(7) + 25_f32.powi(7)) ).sqrt());

    let a_prime_0 = lab_0.a * (1.0 + g);
    let a_prime_1 = lab_1.a * (1.0 + g);

    let c_prime_0 = (a_prime_0.powi(2) + lab_0.b.powi(2)).sqrt();
    let c_prime_1 = (a_prime_1.powi(2) + lab_1.b.powi(2)).sqrt();

    let l_bar_prime = (lab_0.l + lab_1.l)/2.0;
    let c_bar_prime = (c_prime_0 + c_prime_1) / 2.0;

    let h_prime_0 = get_h_prime(a_prime_0, lab_0.b);
    let h_prime_1 = get_h_prime(a_prime_1, lab_1.b);

    let h_bar_prime = if (h_prime_0 - h_prime_1).abs() > 180.0 {
        if (h_prime_0 - h_prime_1) < 360.0 {
            (h_prime_0 + h_prime_1 + 360.0) / 2.0
        } else {
            (h_prime_0 + h_prime_1 - 360.0) / 2.0
        }
    } else {
        (h_prime_0 + h_prime_1) / 2.0
    };

    let t = 1.0 - 0.17 * ((      h_bar_prime - 30.0).to_radians()).cos()
                + 0.24 * ((2.0 * h_bar_prime       ).to_radians()).cos()
                + 0.32 * ((3.0 * h_bar_prime +  6.0).to_radians()).cos()
                - 0.20 * ((4.0 * h_bar_prime - 63.0).to_radians()).cos();
    
    let mut delta_h = h_prime_1 - h_prime_0;
    if delta_h > 180.0 && h_prime_1 <= h_prime_0 {
        delta_h += 360.0;
    } else if delta_h > 180.0 {
        delta_h -= 360.0;
    };

    let delta_l_prime = lab_1.l - lab_0.l;
    let delta_c_prime = c_prime_1 - c_prime_0;
    let delta_h_prime = 2.0 * (c_prime_0 * c_prime_1).sqrt() * (delta_h.to_radians() / 2.0).sin();

    let s_l = 1.0 + (
              (0.015 * (l_bar_prime - 50.0).powi(2))
            / (20.00 + (l_bar_prime - 50.0).powi(2)).sqrt()
        );
    let s_c = 1.0 + 0.045 * c_bar_prime;
    let s_h = 1.0 + 0.015 * c_bar_prime * t;
    
    let delta_theta = 30.0 * (-((h_bar_prime - 275.0)/25.0).powi(2)).exp();
    let r_c =  2.0 * (c_bar_prime.powi(7)/(c_bar_prime.powi(7) + 25_f32.powi(7))).sqrt();
    let r_t = -(r_c * (2.0 * delta_theta.to_radians()).sin());

    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;

    let de2000 = (
        (delta_l_prime/(k_l*s_l)).powi(2)
      + (delta_c_prime/(k_c*s_c)).powi(2)
      + (delta_h_prime/(k_h*s_h)).powi(2)
      + (r_t * (delta_c_prime/(k_c*s_c)) * (delta_h_prime/(k_h*s_h)))
    ).sqrt();

    de2000
}

/// Custom weighted DeltaE formula
fn delta_e_cmc(lab0: &LabValue, lab1 :&LabValue, tolerance_l: f32, tolerance_c: f32) -> f32 {
    let chroma_0 = (lab0.a.powi(2) + lab0.b.powi(2)).sqrt();
    let chroma_1 = (lab1.a.powi(2) + lab1.b.powi(2)).sqrt();
    let delta_c = chroma_0 - chroma_1;

    let delta_l = lab0.l - lab1.l;
    let delta_a = lab0.a - lab1.a;
    let delta_b = lab0.b - lab1.b;

    let delta_h = (delta_a.powi(2) + delta_b.powi(2) - delta_c.powi(2)).sqrt();

    let s_l = if lab0.l < 16.0 {
        0.511
    } else {
        (0.040975 * lab0.l) / (1.0 + (0.01765 * lab0.l))
    };

    let s_c = ((0.0638 * chroma_0) / (1.0 + (0.0131 * chroma_0))) + 0.638;

    let h = lab0.b.atan2(lab0.a).to_degrees();

    let h_1 = if h >= 0.0 {
        h
    } else {
        h + 360.0
    };

    let f = (chroma_0.powi(4) / (chroma_0.powi(4) + 1900.0)).sqrt();

    let t = if 164.0 <= h_1 && h_1 <= 345.0 {
        0.56 + (0.2 * (h_1 + 168.0).to_radians().cos()).abs()
    } else {
        0.36 + (0.4 * (h_1 + 35.0).to_radians().cos()).abs()
    };

    let s_h = s_c * (f * t + 1.0 - f);

    let decmc = (
        (delta_l / (tolerance_l * s_l)).powi(2) +
        (delta_c / (tolerance_c * s_c)).powi(2) +
        (delta_h / s_h).powi(2)
    ).sqrt();

    decmc
}
