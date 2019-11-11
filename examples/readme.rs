use std::error::Error;
use std::str::FromStr;
use deltae::*;

fn main() -> Result<(), Box<dyn Error>>{
    // Lab from a string
    let lab0 = LabValue::from_str("89.73, 1.88, -6.96")?;
    // Lab directly from values
    let lab1 = LabValue {
        l: 95.08,
        a: -0.17,
        b: -10.81,
    }.validate()?; // Validate that the values are in range

    // Calculate DeltaE between two lab values
    let de0 = DeltaE::new(&lab0, &lab1, DE2000);
    // Use the Delta trait
    let de1 = lab0.delta(lab1, DE2000);
    assert_eq!(de0, de1);

    // Convert to other color types
    let lch0 = LchValue::from(lab0);
    let xyz0 = XyzValue::from(lab1);
    assert_eq!(lch0, lab0);
    assert_eq!(xyz0, lab1);

    // Calculate DeltaE between different color types
    let de2 = lch0.delta(xyz0, DE2000);
    assert_eq!(de2.round_to(4), de0.round_to(4));
    // There is some loss of accuracy in the conversion.
    // Usually rounding to 4 decimal places is more than enough.

    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        lab0, lab1, lch0, xyz0, de0, de1, de2,
    );
    // [L:89.73, a:1.88, b:-6.96]
    // [L:95.08, a:-0.17, b:-10.81]
    // [L:89.73, c:7.2094383, h:285.11572]
    // [X:0.8336992, Y:0.8780792, Z:1.1272838]
    // 5.316941
    // 5.316941
    // 5.316937

    Ok(())
}
