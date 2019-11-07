use super::*;
use super::delta::Delta;

impl<T: Delta + Copy> PartialEq<T> for LabValue
where LabValue: From<T> {
    fn eq(&self, other: &T) -> bool {
        self.delta(*other, DE1976).value <= 0.0001
    }
}

impl<T: Delta + Copy> PartialEq<T> for LchValue
where LchValue: From<T> {
    fn eq(&self, other: &T) -> bool {
        self.delta(*other, DE1976).value <= 0.0001
    }
}

impl<T: Delta + Copy> PartialEq<T> for XyzValue
where XyzValue: From<T> {
    fn eq(&self, other: &T) -> bool {
        self.delta(*other, DE1976).value <= 0.0001
    }
}
