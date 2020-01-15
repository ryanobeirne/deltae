//! Standard Illuminants for Chromatic Adaptation.
//! See also: [BruceLindbloom.com](http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html)

use crate::XyzValue;

/// Tungsten-filament (incandescent)
pub const A:   XyzValue = XyzValue { x: 1.09850, y: 1.00000, z: 0.35585 };
/// Daylight simulation at noon (4874°K)
pub const B:   XyzValue = XyzValue { x: 0.99072, y: 1.00000, z: 0.85223 };
/// Daylight simulation average (6774°K)
pub const C:   XyzValue = XyzValue { x: 0.98074, y: 1.00000, z: 1.18232 };
/// Natural daylight at horizon (5003°K)
pub const D50: XyzValue = XyzValue { x: 0.96422, y: 1.00000, z: 0.82521 };
/// Natural daylight at mid-morning (5503°K)
pub const D55: XyzValue = XyzValue { x: 0.95682, y: 1.00000, z: 0.92149 };
/// Natural daylight at noon (6504°K)
pub const D65: XyzValue = XyzValue { x: 0.95047, y: 1.00000, z: 1.08883 };
/// Natural daylight in north sky (7504°K)
pub const D75: XyzValue = XyzValue { x: 0.94972, y: 1.00000, z: 1.22638 };
/// Equal energy radiator (constant spectral distribution)
pub const E:   XyzValue = XyzValue { x: 1.00000, y: 1.00000, z: 1.00000 };
/// Fluorescent (standard)
pub const F2:  XyzValue = XyzValue { x: 0.99186, y: 1.00000, z: 0.67393 };
/// Fluroescent (Broadband)
pub const F7:  XyzValue = XyzValue { x: 0.95041, y: 1.00000, z: 1.08747 };
/// Fluorescent (Narrowband)
pub const F11: XyzValue = XyzValue { x: 1.00962, y: 1.00000, z: 0.64350 };

/// Common Standard Illuminant Types.
/// This list is not exhaustive.
#[derive(Debug, Copy, Clone)]
pub enum Illuminant {
    /// Tungsten-filament (incandescent)
    A,
    /// Daylight simulation at noon (4874°K)
    B,
    /// Daylight simulation average (6774°K)
    C,
    /// Natural daylight at horizon (5003°K)
    D50,
    /// Natural daylight at mid-morning (5503°K)
    D55,
    /// Natural daylight at noon (6504°K)
    D65,
    /// Natural daylight in north sky (7504°K)
    D75,
    /// Equal energy radiator (constant spectral distribution)
    E,
    /// Fluorescent (standard)
    F2,
    /// Fluroescent (Broadband)
    F7,
    /// Fluorescent (Narrowband)
    F11,
}

impl Illuminant {
    /// Get the `XyzValue` of the `Illuminant` type
    pub fn xyz(&self) -> XyzValue {
        XyzValue::from(*self)
    }
}

impl From<Illuminant> for XyzValue {
    fn from(illum: Illuminant) -> Self {
        match illum {
           Illuminant::A   => A,
           Illuminant::B   => B,
           Illuminant::C   => C,
           Illuminant::D50 => D50,
           Illuminant::D55 => D55,
           Illuminant::D65 => D65,
           Illuminant::D75 => D75,
           Illuminant::E   => E,
           Illuminant::F2  => F2,
           Illuminant::F7  => F7,
           Illuminant::F11 => F11,
        }      
    }
}
