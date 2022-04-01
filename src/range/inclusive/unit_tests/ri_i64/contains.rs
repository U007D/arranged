use super::*;
use assert2::assert;

type Range<const START: i64, const END: i64> = RiI64<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    const CANADA: i64 = -2_217_490_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&CANADA);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    const US: i64 = -30_000_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&US);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    const WALLIS_AND_FUTUNA: i64 = 0;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&WALLIS_AND_FUTUNA);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    const MORE_DEBT_THAN_US: i64 = -100_000_000_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&MORE_DEBT_THAN_US);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    const LESS_DEBT_THAN_WALLIS_AND_FUTUNA: i64 = 1_000_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&LESS_DEBT_THAN_WALLIS_AND_FUTUNA);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    let canada = -2_217_490_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&canada);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    let us = -30_000_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&us);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    let wallis_and_futuna = 0;
    let expected = true;

    // When
    let result = Sut::contains(&wallis_and_futuna);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    let more_debt_than_us = -100_000_000_000_000;
    let expected = false;

    // When
    let result = Sut::contains(&more_debt_than_us);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const LARGEST_NATIONAL_DEBT_AS_ASSET: i64 = -30_000_000_000_000;
    const SMALLEST_NATIONAL_DEBT_AS_ASSET: i64 = 0;

    type Sut = Range<LARGEST_NATIONAL_DEBT_AS_ASSET, SMALLEST_NATIONAL_DEBT_AS_ASSET>;

    let less_debt_than_wallis_and_futuna: i64 = 1_000_000;
    let expected = false;

    // When
    let result = Sut::contains(&less_debt_than_wallis_and_futuna);

    // Then
    assert!(result == expected);
}
