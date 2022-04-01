#[allow(unused_imports)]
use super::*;
use crate::RiU64;
use assert2::assert;

#[test]
fn range_with_nonnegative_start_and_end_returns_original_value() {
    // Given
    type OneToOneHundred = RiU64<1, 100>;
    let value = 42;
    let sut = Ranged::<OneToOneHundred>::from(value);
    let expected = Ranged::<OneToOneHundred>::from(value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_nonnegative_start_and_end_with_minimum_value_returns_original_value() {
    // Given
    type ZeroToNineNinetyNine = RiU64<0, 999>;
    let value = 0;
    let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
    let expected = Ranged::<ZeroToNineNinetyNine>::from(value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_nonnegative_start_and_end_with_maximum_value_returns_original_value() {
    // Given
    type OneToTen = RiU64<1, 10>;
    let value = 10;
    let sut = Ranged::<OneToTen>::from(value);
    let expected = Ranged::<OneToTen>::from(value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}
