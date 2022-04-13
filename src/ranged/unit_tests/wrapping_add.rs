use arith_traits::IWrappingOps;
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
fn adding_non_wrapping_ranged_values_of_different_type_yields_expected_sum() {
    // Given
    type OneToOneHundred = RiI8<1, 100>;
    type ZeroToNine = RiI8<0, 9>;
    let sut = Ranged::<OneToOneHundred>::from(41);
    let b = Ranged::<ZeroToNine>::from(1);
    let expected = Ranged::<OneToOneHundred>::from(42);

    // When
    let result = sut.wrapping_add(b);

    // Then
    assert!(result == expected);
}
