/// We can determine that two colors are equivalent if the Delta is less than
/// a certain value. Typically, two colors with a DE2000 value of less than 1.0
/// are considered indistinguishable.
///
use crate::*;
use delta::Delta;

/// A trait for determining if a color is within a DeltaE tolerance
pub trait DeTolerance
where
    Self: Sized + Delta,
{
    fn in_tolerance<D: Delta>(self, other: D, tolerance: &Tolerance) -> bool {
        self.delta(other, &tolerance.0.method) <= tolerance.0
    }
}

impl<T> DeTolerance for T where T: Sized + Delta {}

/// A wrapper around DeltaE for defining a tolerance for the DeltaEq trait
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Tolerance(DeltaE);

impl Tolerance {
    /// Construct a new Tolerance from a value and a DeMethod
    pub fn new(method: DEMethod, value: f32) -> Self {
        Tolerance(
            DeltaE { method, value }
        )
    }
}

impl Default for Tolerance {
    fn default() -> Self {
        Tolerance(DeltaE { method: DE2000, value: 1.0 })
    }
}

impl From<&DeltaE> for Tolerance {
    fn from(de: &DeltaE) -> Self {
        Tolerance(de.clone())
    }
}

impl From<DeltaE> for Tolerance {
    fn from(de: DeltaE) -> Self {
        Tolerance::from(&de)
    }
}

impl PartialEq<DeltaE> for Tolerance {
    fn eq(&self, other: &DeltaE) -> bool {
        &self.0 == other
    }
}

impl PartialEq<Tolerance> for DeltaE {
    fn eq(&self, other: &Tolerance) -> bool {
        self == &other.0
    }
}
