use super::*;
use assert2::assert;

type ValueType = u64;
type Range<const START: ValueType, const END: ValueType> = RiU64<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    const POPULATION_TODAY: ValueType = 7_923_619_609;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&POPULATION_TODAY);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    const YEAR_2000: ValueType = 6_143_493_823;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&YEAR_2000);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    const YEAR_2100: ValueType = 10_875_393_719;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&YEAR_2100);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    const POPULATION_TODAY: ValueType = 6_143_493_822;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&POPULATION_TODAY);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    const POPULATION_TODAY: ValueType = 10_875_393_720;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&POPULATION_TODAY);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let population_today: ValueType = 7_923_619_609;
    let expected = true;

    // When
    let result = Sut::contains(&population_today);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let year_2000: ValueType = 6_143_493_823;
    let expected = true;

    // When
    let result = Sut::contains(&year_2000);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let year_2100: ValueType = 10_875_393_719;
    let expected = true;

    // When
    let result = Sut::contains(&year_2100);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let population_today: ValueType = 6_143_493_822;
    let expected = false;

    // When
    let result = Sut::contains(&population_today);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let population_today: ValueType = 10_875_393_720;
    let expected = false;

    // When
    let result = Sut::contains(&population_today);

    // Then
    assert!(result == expected);
}
