use super::*;
use color::{LabValue, LchValue, XyzValue};
use std::convert::TryFrom;
use std::str::FromStr;

#[test]
fn lab_to_lch() {
    let lab = LabValue {
        l: 30.0,
        a: 40.0,
        b: 50.0,
    };

    let lch  = LchValue::from(lab);
    let lab2 = LabValue::from(lch);
    assert_eq!(lab.round_to(4), lab2.round_to(4));
}

#[test]
fn lch_to_lab() {
    let lch = LchValue {
        l: 30.0,
        c: 40.0,
        h: 50.0,
    };

    let lab  = LabValue::from(lch);
    let lch2 = LchValue::from(lab);
    assert_eq!(lch.round_to(4), lch2.round_to(4));
}

#[test]
fn lab_to_xyz() {
    let lab = LabValue {
        l: 30.0,
        a: 40.0,
        b: 50.0,
    };

    let xyz  = XyzValue::from(lab);
    let lab2 = LabValue::from(xyz);
    assert_eq!(lab.round_to(4), lab2.round_to(4));
}

#[test]
fn lab_string() {
    let good = &[
        "100,128,-128",
        "100,-128,128",
        "100, -128, 128",
        "0,0,0",
        "0,1,-1",
        "50,-1,-1",
        "99.9999,127.9999,-127.9999",
    ];

    for i in good {
        assert!(LabValue::from_str(i).is_ok());
    }

    let bad = &[
        "100,128,-129",
        "101,129,129",
        "101, 129, 129",
        "derp",
        "1,2,three,4",
        "",
        "1,2,3,4",
        "1,2",
        "1",
        "1,2,3,derp"
    ];

    for i in bad {
        assert!(LabValue::from_str(i).is_err());
    }
}

#[test]
fn lch_string() {
    let good = &[
        "100,181.0193,360",
        "100, 181.0193, 360",
        "100,129,129",
        "0,0,0",
        "99.9999,181.0193,359.9999",
    ];

    for i in good {
        assert!(LchValue::from_str(i).is_ok());
    }

    let bad = &[
        "100,128,-129",
        "100,181.0194,360",
        "100, 181.0194, 360",
        "0,-0.01,-0.01",
        "derp",
        "1,2,three,4",
        "",
        "1,2,3,4",
        "1,2",
        "1",
        "1,2,3,derp"
    ];

    for i in bad {
        assert!(LchValue::from_str(i).is_err());
    }
}

#[test]
fn xyz_string() {
    let good = &[
        "0, 0, 0",
        "1, 1, 1",
        "0.5, 0.5, 0.5"
    ];

    for i in good {
        assert!(XyzValue::from_str(i).is_ok());
    }

    let bad = &[
        "-0.01, 0, 0",
        "0, 1.01, 0",
        "0, 0, 1.01",
        "derp",
        "0, 0, 0, derp",
        "0, 0, derp"
    ];

    for i in bad {
        assert!(XyzValue::from_str(i).is_err());
    }
}

fn compare_de(method: DEMethod, expected: f32, reference: &[f32; 3], sample: &[f32; 3]) -> ValueResult<()> {
    let lab0 = LabValue::try_from(reference)?;
    let lab1 = LabValue::try_from(sample)?;

    let de = lab0.delta(lab1, method).round_to(4).value;

    assert_eq!(expected, de);

    Ok(())
}

#[test]
fn decmc1() {
    assert!(compare_de(DEMethod::DECMC(1.0, 1.0), 17.4901, &[20.0, 30.0, 40.0], &[30.0, 40.0, 50.0]).is_ok());
}

#[test]
fn decmc2() {
    assert!(compare_de(DEMethod::DECMC(2.0, 1.0), 10.0731, &[20.0, 30.0, 40.0], &[30.0, 40.0, 50.0]).is_ok());
}

#[test]
fn de1976_test_set() {
    let set = &[
        (0.0000,   &[0.0000,  0.0000,    0.0000  ], &[0.0000,    0.0000,    0.0000  ]),
        (5.0000,   &[0.0000,  0.0000,    0.0000  ], &[0.0000,    3.0000,    4.0000  ]),
        (5.0000,   &[0.0000,  0.0000,    0.0000  ], &[0.0000,   -3.0000,   -4.0000  ]),
        (50.0000,  &[0.0000,  0.0000,    0.0000  ], &[0.0000,   -30.0000,  -40.0000 ]),
        (181.0193, &[0.0000,  0.0000,    0.0000  ], &[0.0000,    128.0000,  128.0000]),
        (362.0387, &[0.0000, -128.0000, -128.0000], &[0.0000,    128.0000,  128.0000]),
        (375.5955, &[0.0000, -128.0000, -128.0000], &[100.0000,  128.0000,  128.0000])
    ];

    for (expected, reference, sample) in set.iter() {
        assert!(compare_de(DEMethod::DE1976, *expected, reference, sample).is_ok());
    }
}

// Tests taken from Table 1: "CIEDE2000 total color difference test data" of
// "The CIEDE2000 Color-Difference Formula: Implementation Notes,
// Supplementary Test Data, and Mathematical Observations" by Gaurav Sharma,
// Wencheng Wu and Edul N. Dalal.
//
// http://www.ece.rochester.edu/~gsharma/papers/CIEDE2000CRNAFeb05.pdf

#[test]
fn de2000_test_set() {
    let set = &[
        (0.0000,   &[0.0000,   0.0000,   0.0000 ], &[0.0000,   0.0000,   0.0000 ]),
        (0.0000,   &[99.5000,  0.0050,  -0.0100 ], &[99.5000,  0.0050,  -0.0100 ]),
        (100.0000, &[100.0000, 0.0050,  -0.0100 ], &[0.0000,   0.0000,   0.0000 ]),
        (2.0425,   &[50.0000,  2.6772,  -79.7751], &[50.0000,  0.0000,  -82.7485]),
        (2.8615,   &[50.0000,  3.1571,  -77.2803], &[50.0000,  0.0000,  -82.7485]),
        (3.4412,   &[50.0000,  2.8361,  -74.0200], &[50.0000,  0.0000,  -82.7485]),
        (1.0000,   &[50.0000, -1.3802,  -84.2814], &[50.0000,  0.0000,  -82.7485]),
        (1.0000,   &[50.0000, -1.1848,  -84.8006], &[50.0000,  0.0000,  -82.7485]),
        (1.0000,   &[50.0000, -0.9009,  -85.5211], &[50.0000,  0.0000,  -82.7485]),
        (2.3669,   &[50.0000,  0.0000,   0.0000 ], &[50.0000, -1.0000,   2.0000 ]),
        (2.3669,   &[50.0000, -1.0000,   2.0000 ], &[50.0000,  0.0000,   0.0000 ]),
        (7.1792,   &[50.0000,  2.4900,  -0.0010 ], &[50.0000, -2.4900,   0.0009 ]),
        (7.1792,   &[50.0000,  2.4900,  -0.0010 ], &[50.0000, -2.4900,   0.0010 ]),
        (7.2195,   &[50.0000,  2.4900,  -0.0010 ], &[50.0000, -2.4900,   0.0011 ]),
        (7.2195,   &[50.0000,  2.4900,  -0.0010 ], &[50.0000, -2.4900,   0.0012 ]),
        (4.8045,   &[50.0000, -0.0010,   2.4900 ], &[50.0000,  0.0009,  -2.4900 ]),
        (4.7461,   &[50.0000, -0.0010,   2.4900 ], &[50.0000,  0.0011,  -2.4900 ]),
        (4.3065,   &[50.0000,  2.5000,   0.0000 ], &[50.0000,  0.0000,  -2.5000 ]),
        (27.1492,  &[50.0000,  2.5000,   0.0000 ], &[73.0000,  25.0000, -18.0000]),
        (22.8977,  &[50.0000,  2.5000,   0.0000 ], &[61.0000, -5.0000,   29.0000]),
        (31.9030,  &[50.0000,  2.5000,   0.0000 ], &[56.0000, -27.0000, -3.0000 ]),
        (19.4535,  &[50.0000,  2.5000,   0.0000 ], &[58.0000,  24.0000,  15.0000]),
        (1.0000,   &[50.0000,  2.5000,   0.0000 ], &[50.0000,  3.1736,   0.5854 ]),
        (1.0000,   &[50.0000,  2.5000,   0.0000 ], &[50.0000,  3.2972,   0.0000 ]),
        (1.0000,   &[50.0000,  2.5000,   0.0000 ], &[50.0000,  1.8634,   0.5757 ]),
        (1.0000,   &[50.0000,  2.5000,   0.0000 ], &[50.0000,  3.2592,   0.3350 ]),
        (1.2644,   &[60.2574, -34.0099,  36.2677], &[60.4626, -34.1751,  39.4387]),
        (1.2630,   &[63.0109, -31.0961, -5.8663 ], &[62.8187, -29.7946, -4.0864 ]),
        (1.8731,   &[61.2901,  3.7196,  -5.3901 ], &[61.4292,  2.2480,  -4.9620 ]),
        (1.8645,   &[35.0830, -44.1164,  3.7933 ], &[35.0232, -40.0716,  1.5901 ]),
        (2.0373,   &[22.7233,  20.0904, -46.6940], &[23.0331, 14.9730,  -42.5619]),
        (1.4146,   &[36.4612,  47.8580,  18.3852], &[36.2715,  50.5065,  21.2231]),
        (1.4441,   &[90.8027, -2.0831,   1.4410 ], &[91.1528, -1.6435,   0.0447 ]),
        (1.5381,   &[90.9257, -0.5406,  -0.9208 ], &[88.6381, -0.8985,   -0.7239]),
        (0.6377,   &[6.7747,  -0.2908,  -2.4247 ], &[5.8714,  -0.0985,   -2.2286]),
        (0.9082,   &[2.0776,   0.0795,  -1.1350 ], &[0.9033,  -0.0636,   -0.5514])
    ];

    for (expected, reference, sample) in set.iter() {
        assert!(compare_de(DEMethod::DE2000, *expected, reference, sample).is_ok())
    }
}
