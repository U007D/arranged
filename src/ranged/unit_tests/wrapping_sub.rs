use arith_traits::{IMinMax, IWrappingOps};
use assert2::assert;

use crate::{Ranged, RiI8};

#[test]
fn subtracting_non_wrapping_ranged_values_of_same_type_yields_expected_difference() {
    // Given
    type OneToOneHundred = RiI8<1, 100>;
    let sut = Ranged::<OneToOneHundred>::from(43);
    let b = Ranged::<OneToOneHundred>::from(1);
    let expected = Ranged::<OneToOneHundred>::from(42);

    // When
    let result = sut.wrapping_sub(b);

    // Then
    assert!(result == expected);
}

#[test]
fn subtracting_non_wrapping_ranged_values_of_different_types_yields_expected_difference() {
    // Given
    type TwoToOneHundredAndOne = RiI8<2, 101>;
    type ZeroToNine = RiI8<0, 9>;
    let sut = Ranged::<TwoToOneHundredAndOne>::from(43);
    let b = Ranged::<ZeroToNine>::from(1);
    let expected = Ranged::<TwoToOneHundredAndOne>::from(42);

    // When
    let result = sut.wrapping_sub(b);

    // Then
    assert!(result == expected);
}

#[test]
fn subtracting_wrapping_ranged_values_of_same_type_yields_expected_difference() {
    // Given
    type ThirteenToNinetySeven = RiI8<13, 97>;
    let sut = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MIN);
    let b = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MAX);
    let expected = Ranged::<ThirteenToNinetySeven>::from(86);

    // When
    let result = sut.wrapping_sub(b);

    // Then
    assert!(result == expected);
}

#[test]
fn subtracting_wrapping_ranged_values_of_different_types_yields_expected_difference() {
    // Given
    type ThirteenToNinetySeven = RiI8<13, 97>;
    type ThreeToThree = RiI8<3, 3>;
    let sut = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MIN);
    let b = Ranged::<ThreeToThree>::from(3);
    let expected = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MAX - 2);

    // When
    let result = sut.wrapping_sub(b);

    // Then
    assert!(result == expected);
}

#[test]
fn subtracting_wrapping_negative_ranged_values_of_different_types_yields_expected_difference() {
    // Given
    type NegativeNinetySevenToNegativeThirteen = RiI8<-97, -13>;
    type NegativeTenToThree = RiI8<-10, 3>;
    let sut = Ranged::<NegativeNinetySevenToNegativeThirteen>::from(NegativeNinetySevenToNegativeThirteen::MAX);
    let b = Ranged::<NegativeTenToThree>::from(-1);
    let expected = Ranged::<NegativeNinetySevenToNegativeThirteen>::from(NegativeNinetySevenToNegativeThirteen::MIN);

    // When
    let result = sut.wrapping_sub(b);

    // Then
    assert!(result == expected);
}
