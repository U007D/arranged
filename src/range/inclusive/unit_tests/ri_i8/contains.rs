use super::*;
use assert2::assert;

type ValueType = i8;
type Range<const START: ValueType, const END: ValueType> = RiI8<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    const STRAIGHT: ValueType = 0;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&STRAIGHT);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_minimum_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    const MINIMUM: ValueType = -1;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&MINIMUM);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_maximum_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    const MINIMUM: ValueType = 1;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&MINIMUM);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    const BEYOND_MAX: ValueType = -2;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&BEYOND_MAX);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    const BEYOND_MAX: ValueType = 2;
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
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let straight: ValueType = 0;
    let expected = true;

    // When
    let result = Sut::contains(&straight);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_minimum_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let minimum: ValueType = -1;
    let expected = true;

    // When
    let result = Sut::contains(&minimum);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_maximum_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let minimum: ValueType = 1;
    let expected = true;

    // When
    let result = Sut::contains(&minimum);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let beyond_max: ValueType = -2;
    let expected = false;

    // When
    let result = Sut::contains(&beyond_max);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let beyond_max: ValueType = 2;
    let expected = false;

    // When
    let result = Sut::contains(&beyond_max);

    // Then
    assert!(result == expected);
}
