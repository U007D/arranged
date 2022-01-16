#![allow(clippy::equatable_if_let, clippy::unwrap_used)]

use crate::ranges::*;
use assert2::assert;
use crate::ranges::RangeInclusiveI8;

// TODO: Many test cases represent compilation failures.  Figure out how to meaningfully test for compilation failure.

#[test]
fn adding_two_ri_u8_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiU8::<RangeInclusiveI8::<13, 23>>::from_value(19).unwrap();
    let r2 = RiU8::<RangeInclusiveI8::<19, 29>>::from_value(23).unwrap();
    let expected = RiU8::<RangeInclusiveI8<32, 52>>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_u16_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiU16::<13, 23>::from_value(19).unwrap();
    let r2 = RiU16::<19, 29>::from_value(23).unwrap();
    let expected = RiU16::<32, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_u32_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiU32::<13, 23>::from_value(19).unwrap();
    let r2 = RiU32::<19, 29>::from_value(23).unwrap();
    let expected = RiU32::<32, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_u64_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiU64::<13, 23>::from_value(19).unwrap();
    let r2 = RiU64::<19, 29>::from_value(23).unwrap();
    let expected = RiU64::<32, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_u128_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiU128::<13, 23>::from_value(19).unwrap();
    let r2 = RiU128::<19, 29>::from_value(23).unwrap();
    let expected = RiU128::<32, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_usize_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiUsize::<13, 23>::from_value(19).unwrap();
    let r2 = RiUsize::<19, 29>::from_value(23).unwrap();
    let expected = RiUsize::<32, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_i8_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiI8::<{ -13 }, 23>::from_value(19).unwrap();
    let r2 = RiI8::<{ -19 }, 29>::from_value(23).unwrap();
    let expected = RiI8::<{ -32 }, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_i16_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiI16::<{ -13 }, 23>::from_value(19).unwrap();
    let r2 = RiI16::<{ -19 }, 29>::from_value(23).unwrap();
    let expected = RiI16::<{ -32 }, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_i32_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiI32::<{ -13 }, 23>::from_value(19).unwrap();
    let r2 = RiI32::<{ -19 }, 29>::from_value(23).unwrap();
    let expected = RiI32::<{ -32 }, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_i64_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiI64::<{ -13 }, 23>::from_value(19).unwrap();
    let r2 = RiI64::<{ -19 }, 29>::from_value(23).unwrap();
    let expected = RiI64::<{ -32 }, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_i128_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiI128::<{ -13 }, 23>::from_value(19).unwrap();
    let r2 = RiI128::<{ -19 }, 29>::from_value(23).unwrap();
    let expected = RiI128::<{ -32 }, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}

#[test]
fn adding_two_ri_isize_ranges_whose_bounds_sum_is_valid_succeeds() {
    // Given
    let r1 = RiIsize::<{ -13 }, 23>::from_value(19).unwrap();
    let r2 = RiIsize::<{ -19 }, 29>::from_value(23).unwrap();
    let expected = RiIsize::<{ -32 }, 52>::from_value(42).unwrap();

    // When
    let result = r1 + r2;

    // Then
    assert!(result == expected);
}
