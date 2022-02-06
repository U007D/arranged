use super::*;
use assert2::assert;

type ValueType = i32;
type Range<const START: ValueType, const END: ValueType> = RiI32<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    const AVERAGE_ELEVATION: ValueType = 800;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&AVERAGE_ELEVATION);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    const CORE: ValueType = -4_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&CORE);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    const KARMAN_LINE: ValueType = 100_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&KARMAN_LINE);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    const DEEPER_THAN_CENTER: ValueType = 5_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&DEEPER_THAN_CENTER);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    const ABOVE_ATMOSPHERE: ValueType = 2_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&ABOVE_ATMOSPHERE);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    let average_elevation: ValueType = 800;
    let expected = true;

    // When
    let result = Sut::contains(&average_elevation);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    let core: ValueType = -4_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&core);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    let karman_line: ValueType = 100_000;
    let expected = true;

    // When
    let result = Sut::contains(&karman_line);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    let beyond_center: ValueType = 5_000_000;
    let expected = false;

    // When
    let result = Sut::contains(&beyond_center);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const DEPTH_TO_CORE_IN_M: ValueType = -4_000_000;
    const HEIGHT_TO_KARMAN_LINE_IN_M: ValueType = 100_000;

    type Sut = Range<DEPTH_TO_CORE_IN_M, HEIGHT_TO_KARMAN_LINE_IN_M>;

    let above_atmosphere: ValueType = 2_000_000;
    let expected = false;

    // When
    let result = Sut::contains(&above_atmosphere);

    // Then
    assert!(result == expected);
}
