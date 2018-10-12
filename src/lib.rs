pub mod color;

#[cfg(test)]
pub mod tests;

use color::*;

#[derive(Debug, PartialEq)]
pub struct DeltaE {
    pub method: DEMethod,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub enum DEMethod{
    DE2000,
    DE1994,
    DE1976,
    // DECMC1,
    // DECMC2,
}

impl DEMethod {
    pub fn from(string: &str) -> DEMethod {
        match string.to_lowercase().as_ref() {
            "de1976" | "de76" | "1976" | "76" => DEMethod::DE1976,
            "de2000" | "de00" | "2000" | "00" => DEMethod::DE2000,
            "de1994" | "de94" | "1994" | "94" => DEMethod::DE1994,
            // "decmc1" | "cmc1" | "cmc"         => DEMethod::DECMC1,
            // "decmc2" | "cmc2"                 => DEMethod::DECMC1,
            _ => {
                eprintln!("Invalid Method: '{}'. Using DE2000", string);
                DEMethod::DE2000 
            }
        }
    }
}

impl DeltaE {
    pub fn new(lab_0: &LabValue, lab_1: &LabValue, method: DEMethod) -> DeltaE {
        let value = match method {
            DEMethod::DE1976 => delta_e_1976(lab_0, lab_1),
            DEMethod::DE1994 => delta_e_1994(lab_0, lab_1),
            DEMethod::DE2000 => delta_e_2000(lab_0, lab_1),
        };

        DeltaE { method, value }
    }

    pub fn round_to(self, places: i32) -> DeltaE {
        DeltaE {
            method: self.method,
            value: round_to(self.value, places),
        }
    }
}

fn round_to(val: f64, places: i32) -> f64 {
    let mult = 10_f64.powi(places);
    (val * mult).round() / mult
}

fn delta_e_1976(c0: &LabValue, c1: &LabValue) -> f64 {
    //! DeltaE 1976. Basic euclidian distance formula.
    ( (c0.l - c1.l).powi(2) + (c0.a - c1.a).powi(2) + (c0.b - c1.b).powi(2) ).sqrt()
}

fn delta_e_1994(c0: &LabValue, c1: &LabValue) -> f64 {
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

fn delta_e_2000(c0: &LabValue, c1:&LabValue) -> f64 {
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