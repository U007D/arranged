use super::*;
use assert2::assert;

type Range<const START: u16, const END: u16> = RiU16<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const CHILD_BONE_COUNT: u16 = 242;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&CHILD_BONE_COUNT);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const ADULT_BONE_COUNT: u16 = 206;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&ADULT_BONE_COUNT);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const INFANT_BONE_COUNT: u16 = 270;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&INFANT_BONE_COUNT);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const TOO_LOW: u16 = 205;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&TOO_LOW);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const TOO_HIGH: u16 = 271;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&TOO_HIGH);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let child_bone_count: u16 = 242;
    let expected = true;

    // When
    let result = Sut::contains(&child_bone_count);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let adult_bone_count: u16 = 206;
    let expected = true;

    // When
    let result = Sut::contains(&adult_bone_count);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let infant_bone_count: u16 = 270;
    let expected = true;

    // When
    let result = Sut::contains(&infant_bone_count);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let too_low: u16 = 205;
    let expected = false;

    // When
    let result = Sut::contains(&too_low);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let too_high: u16 = 271;
    let expected = false;

    // When
    let result = Sut::contains(&too_high);

    // Then
    assert!(result == expected);
}
