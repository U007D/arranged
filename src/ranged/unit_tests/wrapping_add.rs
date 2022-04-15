use arith_traits::{IMinMax, IWrappingOps};
use assert2::assert;

use crate::{Ranged, RiI8};

#[test]
fn adding_non_wrapping_ranged_values_of_same_type_yields_expected_sum() {
    // Given
    type OneToOneHundred = RiI8<1, 100>;
    let sut = Ranged::<OneToOneHundred>::from(41);
    let b = Ranged::<OneToOneHundred>::from(1);
    let expected = Ranged::<OneToOneHundred>::from(42);

    // When
    let result = sut.wrapping_add(b);

    // Then
    assert!(result == expected);
}

#[test]
fn adding_non_wrapping_ranged_values_of_different_types_yields_expected_sum() {
    // Given
    type TwoToOneHundredAndOne = RiI8<2, 101>;
    type ZeroToNine = RiI8<0, 9>;
    let sut = Ranged::<TwoToOneHundredAndOne>::from(41);
    let b = Ranged::<ZeroToNine>::from(1);
    let expected = Ranged::<TwoToOneHundredAndOne>::from(42);

    // When
    let result = sut.wrapping_add(b);

    // Then
    assert!(result == expected);
}

#[test]
fn adding_wrapping_ranged_values_of_same_type_yields_expected_sum() {
    // Given
    type ThirteenToNinetySeven = RiI8<13, 97>;
    let sut = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MAX);
    let b = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MIN);
    let expected = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MIN + 12);

    // When
    let result = sut.wrapping_add(b);

    // Then
    assert!(result == expected);
}

#[test]
fn adding_wrapping_ranged_values_of_different_types_yields_expected_sum() {
    // Given
    type ThirteenToNinetySeven = RiI8<13, 97>;
    type ThreeToThree = RiI8<3, 3>;
    let sut = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MAX);
    let b = Ranged::<ThreeToThree>::from(3);
    let expected = Ranged::<ThirteenToNinetySeven>::from(ThirteenToNinetySeven::MIN + 2);

    // When
    let result = sut.wrapping_add(b);

    // Then
    assert!(result == expected);
}
