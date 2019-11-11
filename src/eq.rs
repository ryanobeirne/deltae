/// We can determine that two colors are equivalent if the Delta is less than
/// a certain value. Typically, two colors with a DE2000 value of less than 1.0
/// are considered indistinguishable.
///
use super::*;
use super::delta::Delta;

impl<T: Delta + Copy> PartialEq<T> for LabValue
where LabValue: From<T> {
    fn eq(&self, other: &T) -> bool {
        self.delta(*other, DE2000).value < 1.0
    }
}

impl<T: Delta + Copy> PartialEq<T> for LchValue
where LchValue: From<T> {
    fn eq(&self, other: &T) -> bool {
        self.delta(*other, DE2000).value < 1.0
    }
}

impl<T: Delta + Copy> PartialEq<T> for XyzValue
where XyzValue: From<T> {
    fn eq(&self, other: &T) -> bool {
        self.delta(*other, DE2000).value < 1.0
    }
}
