#[allow(unused_imports)]
use super::*;
use crate::{RiI64, RiU64};
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

#[test]
fn range_with_negative_start_nonnegative_end_and_nonnegative_value_returns_original_value() {
    // Given
    type NegativeOneHundredToOneHundred = RiI64<-100, 100>;
    let value = 21;
    let sut = Ranged::<NegativeOneHundredToOneHundred>::from(value);
    let expected = Ranged::<NegativeOneHundredToOneHundred>::from(value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_nonnegative_end_and_negative_value_returns_positive_in_bounds_value() {
    // Given
    type NegativeOneHundredToOneHundred = RiI64<-100, 100>;
    let value = -21;
    let sut = Ranged::<NegativeOneHundredToOneHundred>::from(value);
    let expected = Ranged::<NegativeOneHundredToOneHundred>::from(-value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_nonnegative_end_and_negative_value_returns_wrapped_value() {
    // Given
    type ZeroToNineNinetyNine = RiI64<-100, 20>;
    let value = -35;
    let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
    let expected = Ranged::<ZeroToNineNinetyNine>::from(-85);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_nonnegative_end_and_min_value_returns_wrapped_value() {
    // Given
    type ZeroToNineNinetyNine = RiI64<-100, 20>;
    let value = -100;
    let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
    let expected = Ranged::<ZeroToNineNinetyNine>::from(-20);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_nonnegative_end_and_max_value_returns_wrapped_value() {
    // Given
    type ZeroToNineNinetyNine = RiI64<-100, 20>;
    let value = 20;
    let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
    let expected = Ranged::<ZeroToNineNinetyNine>::from(value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_negative_end_and_negative_value_returns_wrapped_value() {
    // Given
    type NegativeOneHundredToOneHundred = RiI64<-50, -1>;
    let value = -11;
    let sut = Ranged::<NegativeOneHundredToOneHundred>::from(value);
    let expected = Ranged::<NegativeOneHundredToOneHundred>::from(-38);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_negative_end_and_min_value_returns_wrapped_value() {
    // Given
    type ZeroToNineNinetyNine = RiI64<-50, -1>;
    let value = -50;
    let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
    let expected = Ranged::<ZeroToNineNinetyNine>::from(-49);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

#[test]
fn range_with_negative_start_negative_end_and_max_value_returns_wrapped_value() {
    // Given
    type ZeroToNineNinetyNine = RiI64<-50, -1>;
    let value = -1;
    let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
    let expected = Ranged::<ZeroToNineNinetyNine>::from(-48);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}
