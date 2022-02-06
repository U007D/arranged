use super::*;
use assert2::assert;

type ValueType = u8;
type Range<const START: ValueType, const END: ValueType> = RiU8<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    const ANNOY_NEIGHBORS: ValueType = 9;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&ANNOY_NEIGHBORS);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    const MINIMUM: ValueType = 0;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&MINIMUM);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    const MAXIMUM: ValueType = 11;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&MAXIMUM);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const GUESS_A_NUMBER_MIN: ValueType = 1;
    const GUESS_A_NUMBER_MAX: ValueType = 10;

    type Sut = Range<GUESS_A_NUMBER_MIN, GUESS_A_NUMBER_MAX>;

    const BEYOND_MIN: ValueType = 0;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&BEYOND_MIN);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const GUESS_A_NUMBER_MIN: ValueType = 1;
    const GUESS_A_NUMBER_MAX: ValueType = 10;

    type Sut = Range<GUESS_A_NUMBER_MIN, GUESS_A_NUMBER_MAX>;

    const BEYOND_MAX: ValueType = 11;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&BEYOND_MAX);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    let annoy_neighbors: ValueType = 9;
    let expected = true;

    // When
    let result = Sut::contains(&annoy_neighbors);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    let minimum: ValueType = 0;
    let expected = true;

    // When
    let result = Sut::contains(&minimum);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    let maximum: ValueType = 11;
    let expected = true;

    // When
    let result = Sut::contains(&maximum);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_VOLUME: ValueType = 0;
    const MAX_VOLUME: ValueType = 11;

    type Sut = Range<MIN_VOLUME, MAX_VOLUME>;

    let beyond_max: ValueType = 12;
    let expected = false;

    // When
    let result = Sut::contains(&beyond_max);

    // Then
    assert!(result == expected);
}
