#[derive(Debug, PartialEq)]
pub struct LabValue {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, PartialEq)]
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

pub type ValueResult<T> = Result<T, ValueError>;

impl LabValue {
    pub fn zero() -> LabValue {
        //! New LabValue with a value of 0,0,0.
        LabValue { l: 0.0, a: 0.0, b: 0.0 }
    }

    pub fn from(s: &str) -> ValueResult<LabValue> {
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
            h: h % 360.0,
        }
    }

    pub fn round_to(&self, places: i32) -> LchValue {
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
       Ok(string_to_lch(s)?)
    }

    pub fn to_lab(&self) -> LabValue {
        LabValue {
            l: self.l,
            a: self.c * self.h.to_radians().cos(),
            b: self.c * self.h.to_radians().sin(),
        }
    }

    pub fn round_to(&self, places: i32) -> LchValue {
        LchValue {
            l: round_to(self.l, places),
            c: round_to(self.c, places),
            h: round_to(self.h, places),
        }       
    }
}

pub fn round_to(val: f64, places: i32) -> f64 {
    let mult = 10_f64.powi(places);
    (val * mult).round() / mult
}

pub fn string_to_lab(lab_string: &str) -> ValueResult<LabValue> {
    //! Validate and convert strings to LabValue.
    //! Split string by comma (92.5,33.5,-18.8).
    let s = lab_string.split(",");
    let st: Vec<&str> = s.clone().collect();

    //Validate that there are only 3 values
    if st.len() != 3 {
        return Err(ValueError::BadFormat);
    };

    let split = s.filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<f64>>();

    //Validate that there are only 3 floats
    if split.len() != 3 {
        // eprintln!("Bad Lab values format: '{}'\n\tUse 'L,a,b'", lab_string);
        return Err(ValueError::BadFormat);
    };

    //Check that the Lab values are in the proper range or Error
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

pub fn string_to_lch(lch_string: &str) -> ValueResult<LchValue> {
    //! Validate and convert strings to LchValue.
    //! Split string by comma (92.5,33.5,240.3).
    let s = lch_string.split(",");
    let st: Vec<&str> = s.clone().collect();

    // Validate that there are only 3 values
    if st.len() != 3 {
        return Err(ValueError::BadFormat);
    };

    let split = s.filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<f64>>();

    // Validate that there are only 3 floats
    if split.len() != 3 {
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

pub fn delta_e_1976(c0: &LabValue, c1: &LabValue) -> f64 {
    //! DeltaE 1976. Basic euclidian distance formula.
    ( (c0.l - c1.l).powi(2) + (c0.a - c1.a).powi(2) + (c0.b - c1.b).powi(2) ).sqrt()
}

pub fn delta_e_1994(c0: &LabValue, c1: &LabValue) -> f64 {
    //! DeltaE 1994. Only does Graphic Arts mode at the moment.
    //! TODO: Add Textiles mode
    let delta_l = c0.l - c1.l;
    let chroma_0 = (c0.a.powi(2) + c0.b.powi(2)).sqrt();
    let chroma_1 = (c1.a.powi(2) + c1.b.powi(2)).sqrt();
    let delta_chroma = chroma_0 - chroma_1;
    let delta_a = c0.a - c1.a;
    let delta_b = c0.b - c1.b;
    let delta_hue = (delta_a.powi(2) + delta_b.powi(2) - delta_chroma.powi(2)).sqrt();
    let s_l = 1.0;
    let s_c = 1.0 + 0.045 * chroma_0;
    let s_h = 1.0 + 0.015 * chroma_0;

    (   (delta_l / s_l).powi(2)
      + (delta_chroma / s_c).powi(2)
      + (delta_hue / s_h).powi(2)
    ).sqrt()
}

pub fn delta_e_2000(c0: &LabValue, c1:&LabValue) -> f64 {
    //! DeltaE 2000. This is a ridiculously complicated formula.
    let l_bar_prime = (c0.l + c1.l)/2.0;
    let c_0 = (c0.a.powi(2) + c0.b.powi(2)).sqrt();
    let c_1 = (c1.a.powi(2) + c1.b.powi(2)).sqrt();
    let c_bar = (c_0 + c_1) / 2.0;
    let g = 0.5 * (1.0 - ( c_bar.powi(7) / (c_bar.powi(7) + 25_f64.powi(7)) ).sqrt());
    let a_prime_0 = c0.a * (1.0 + g);
    let a_prime_1 = c1.a * (1.0 + g);
    let c_prime_0 = (a_prime_0.powi(2) + c0.b.powi(2)).sqrt();
    let c_prime_1 = (a_prime_1.powi(2) + c1.b.powi(2)).sqrt();
    let c_bar_prime = (c_prime_0 + c_prime_1) / 2.0;
    
    //Hue calculations have to account for degrees: 360 == 0
    let h_prime_0 = c0.to_lch().h;
    let h_prime_1 = c1.to_lch().h;
    let mut h_bar_prime = h_prime_0 - h_prime_1;
    if h_bar_prime > 180.0 {
        h_bar_prime = (h_prime_0 + h_prime_1 + 360.0) / 2.0;
    } else {
        h_bar_prime = (h_prime_0 + h_prime_1) / 2.0;
    };

    let t = 1.0 - 0.17*(    h_bar_prime - 30.0).to_radians().cos()
                + 0.24*(2.0*h_bar_prime       ).to_radians().cos()
                + 0.32*(3.0*h_bar_prime +  6.0).to_radians().cos()
                - 0.20*(4.0*h_bar_prime - 63.0).to_radians().cos();
    
    let mut delta_h = h_prime_1 - h_prime_0;
    if delta_h > 180.0 && h_prime_1 <= h_prime_0 {
        delta_h += 360.0;
    } else if delta_h > 180.0 {
        delta_h -= 360.0;
    };

    let delta_l_prime = c1.l - c0.l;
    let delta_c_prime = c_prime_1 - c_prime_0;
    let delta_h_prime = 2.0 * (c_prime_0 * c_prime_1).sqrt() * (delta_h.to_radians() / 2.0).sin();

    let s_l = 1.0 + (
              (0.015 * (l_bar_prime - 50.0).powi(2))
            / (20.00 + (l_bar_prime - 50.0).powi(2)).sqrt()
        );
    let s_c = 1.0 + 0.045 * c_bar_prime;
    let s_h = 1.0 + 0.015 * c_bar_prime * t;
    
    let delta_theta = 30.0 * (-((h_bar_prime - 275.0)/25.0).powi(2)).exp();
    let r_c =  2.0 * (c_bar_prime.powi(7)/(c_bar_prime.powi(7) + 25_f64.powi(7))).sqrt();
    let r_t = -(r_c * (2.0 * delta_theta.to_radians()).sin());

    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;

    //Return the Delta E 2000
    (
        (delta_l_prime/(k_l*s_l)).powi(2)
      + (delta_c_prime/(k_c*s_c)).powi(2)
      + (delta_h_prime/(k_h*s_h)).powi(2)
      + (r_t * (delta_c_prime/(k_c*s_c)) * (delta_h_prime/(k_h*s_h)))
    ).sqrt()
}

//fn delta_e_CMC1(c1: &LabValue, c2:&LabValue) -> f64 {
    //math
//}

//fn delta_e_CMC2(c1: &LabValue, c2:&LabValue) -> f64 {
    //math
//}

#[cfg(test)]
pub mod tests;
