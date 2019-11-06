use super::*;
use std::convert::TryFrom;
use std::error::Error;

// To Lab /////////////////////////////////////////////////////////////////////
impl From<LchValue> for LabValue {
    fn from(lch: LchValue) -> LabValue {
        LabValue {
            l: lch.l,
            a: lch.c * lch.h.to_radians().cos(),
            b: lch.c * lch.h.to_radians().sin(),
        }
    }
}

impl From<&LchValue> for LabValue {
    fn from(lch: &LchValue) -> LabValue {
        LabValue::from(*lch)
    }
}

impl From<&LabValue> for LabValue {
    fn from(lab: &LabValue) -> LabValue {
        *lab
    }
}

impl From<XyzValue> for LabValue {
    fn from(xyz: XyzValue) -> LabValue {
        color::xyz_to_lab([xyz.x, xyz.y, xyz.z])
    }
}

impl From<&XyzValue> for LabValue {
    fn from(xyz: &XyzValue) -> LabValue {
        LabValue::from(*xyz)
    }
}

impl TryFrom<&[f32; 3]> for LabValue {
    type Error = Box<dyn Error>;
    fn try_from(slice: &[f32; 3]) -> ValueResult<LabValue> {
        LabValue {
            l: slice[0],
            a: slice[1],
            b: slice[2]
        }.validate()
    }
}

impl TryFrom<(f32, f32, f32)> for LabValue {
    type Error = Box<dyn Error>;
    fn try_from(tuple: (f32, f32, f32)) -> ValueResult<LabValue> {
        LabValue {
            l: tuple.0,
            a: tuple.1,
            b: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f32, f32, f32)> for LabValue {
    type Error = Box<dyn Error>;
    fn try_from(tuple: &(f32, f32, f32)) -> ValueResult<LabValue> {
        LabValue {
            l: tuple.0,
            a: tuple.1,
            b: tuple.2,
        }.validate()
    }
}

// To Lch /////////////////////////////////////////////////////////////////////
impl From<LabValue> for LchValue {
    fn from(lab: LabValue) -> LchValue {
        LchValue {
            l: lab.l,
            c: ( lab.a.powi(2) + lab.b.powi(2) ).sqrt(),
            h: get_h_prime(lab.a, lab.b),
        }
    }
}

impl From<&LabValue> for LchValue {
    fn from(lab: &LabValue) -> LchValue {
        LchValue::from(*lab)
    }
}

impl From<XyzValue> for LchValue {
    fn from(xyz: XyzValue) -> LchValue {
        LchValue::from(LabValue::from(xyz))
    }
}

impl From<&XyzValue> for LchValue {
    fn from(xyz: &XyzValue) -> LchValue {
        LchValue::from(*xyz)
    }
}

impl TryFrom<&[f32; 3]> for LchValue {
    type Error = Box<dyn Error>;
    fn try_from(slice: &[f32; 3]) -> ValueResult<LchValue> {
        LchValue {
            l: slice[0],
            c: slice[1],
            h: slice[2]
        }.validate()
    }
}

impl TryFrom<(f32, f32, f32)> for LchValue {
    type Error = Box<dyn Error>;
    fn try_from(tuple: (f32, f32, f32)) -> ValueResult<LchValue> {
        LchValue {
            l: tuple.0,
            c: tuple.1,
            h: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f32, f32, f32)> for LchValue {
    type Error = Box<dyn Error>;
    fn try_from(tuple: &(f32, f32, f32)) -> ValueResult<LchValue> {
        LchValue {
            l: tuple.0,
            c: tuple.1,
            h: tuple.2,
        }.validate()
    }
}

// To Xyz /////////////////////////////////////////////////////////////////////
impl From<LabValue> for XyzValue {
    fn from(lab: LabValue) -> XyzValue {
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
}

impl From<&LabValue> for XyzValue {
    fn from(lab: &LabValue) -> XyzValue {
        XyzValue::from(*lab)
    }
}

impl From<LchValue> for XyzValue {
    fn from(lch: LchValue) -> XyzValue {
        XyzValue::from(LabValue::from(lch))
    }
}

impl From<&LchValue> for XyzValue {
    fn from(lch: &LchValue) -> XyzValue {
        XyzValue::from(*lch)
    }
}

impl TryFrom<&[f32; 3]> for XyzValue {
    type Error = Box<dyn Error>;
    fn try_from(slice: &[f32; 3]) -> ValueResult<XyzValue> {
        XyzValue {
            x: slice[0],
            y: slice[1],
            z: slice[2]
        }.validate()
    }
}

impl TryFrom<(f32, f32, f32)> for XyzValue {
    type Error = Box<dyn Error>;
    fn try_from(tuple: (f32, f32, f32)) -> ValueResult<XyzValue> {
        XyzValue {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }.validate()
    }
}

impl TryFrom<&(f32, f32, f32)> for XyzValue {
    type Error = Box<dyn Error>;
    fn try_from(tuple: &(f32, f32, f32)) -> ValueResult<XyzValue> {
        XyzValue {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }.validate()
    }
}
