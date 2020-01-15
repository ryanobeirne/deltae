use crate::*;
use illuminant::Illuminant;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Matrix3x3 {
    inner: [f32; 9],
}

impl Matrix3x3 {
    pub fn new(xyz_0: XyzValue, xyz_1: XyzValue, xyz_2: XyzValue) -> Self {
        Matrix3x3 {
            inner: [
                xyz_0.x, xyz_0.y, xyz_0.z,
                xyz_1.x, xyz_1.y, xyz_1.z,
                xyz_2.x, xyz_2.y, xyz_2.z,
            ]
        }
    }
}

impl<'a> FromIterator<&'a f32> for Matrix3x3 {
    fn from_iter<I: IntoIterator<Item=&'a f32>>(iter: I) -> Self {
        let vec = iter.into_iter().take(9).collect::<Vec<_>>();

        let mut inner = [1_f32; 9];
        for i in 0..9 {
            if let Some(val) = vec.get(i) {
                inner[i] = **val;
            }
        }
            
        Matrix3x3 { inner }
    }
}

#[test]
fn matrix_from_iter() {
    let arr = [1.0_f32, 2.0, 3.0, 4.0, 5.0];
    let matrix = arr.into_iter().collect::<Matrix3x3>();
    let exp = Matrix3x3 {
        inner: [
            1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 1.0, 1.0, 1.0
        ]
    };
    assert_eq!(matrix, exp);
}

pub fn xyz_to_rgb(xyz: XyzValue, illum: Illuminant) -> RgbValue {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub struct RgbNominalValue {
    r: f32,
    g: f32,
    b: f32,
}

#[test]
fn nominalize_rgb() {
    let rgb = RgbValue::new(64, 128, 255);
    let nom = rgb.nominalize();
    let exp = RgbNominalValue {
        r: 0.2509804,
        g: 0.5019608,
        b: 1.0,
    };
    assert_eq!(nom, exp);
}

pub trait Nominalize {
    type NominalType;
    fn nominalize(&self) -> Self::NominalType;
}

impl Nominalize for RgbValue {
    type NominalType = RgbNominalValue;
    fn nominalize(&self) -> Self::NominalType {
        RgbNominalValue {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
        }
    }
}
