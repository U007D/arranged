#![allow(clippy::equatable_if_let, clippy::unwrap_used)]

use super::*;
use assert2::assert;

#[test]
const fn range_const_evaluates_contains_in_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type Sut = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: u64 = 1_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn range_const_evaluates_does_not_contain_low_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type Sut = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: u64 = 349_999_999;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn range_const_evaluates_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 1_400_000_001;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type Sut = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: u64 = 1_500_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type Sut = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq = 1_000_000_000_u64;
    let expected = true;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type Sut = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq = 349_999_999_u64;
    let expected = false;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type Sut = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq = 1_400_000_001_u64;
    let expected = false;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}
