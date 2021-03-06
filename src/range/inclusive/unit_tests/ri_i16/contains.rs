use super::*;
use assert2::assert;

type Range<const START: i16, const END: i16> = RiI16<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    const COMFORTABLE: i16 = 20;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&COMFORTABLE);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    const ANTARCTICA: i16 = -100;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&ANTARCTICA);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    const DEATH_VALLEY: i16 = 50;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&DEATH_VALLEY);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    const UNNATURALLY_COLD: i16 = -200;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&UNNATURALLY_COLD);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    const UNNATURALLY_HOT: i16 = 100;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&UNNATURALLY_HOT);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    let comfortable: i16 = 20;
    let expected = true;

    // When
    let result = Sut::contains(&comfortable);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    let antarctica: i16 = -100;
    let expected = true;

    // When
    let result = Sut::contains(&antarctica);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    let death_valley: i16 = 50;
    let expected = true;

    // When
    let result = Sut::contains(&death_valley);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    let unnaturally_cold: i16 = -200;
    let expected = false;

    // When
    let result = Sut::contains(&unnaturally_cold);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const COLDEST_ON_EARTH_IN_C: i16 = -100;
    const HOTTEST_ON_EARTH_IN_C: i16 = 50;

    type Sut = Range<COLDEST_ON_EARTH_IN_C, HOTTEST_ON_EARTH_IN_C>;

    let unnaturally_hot: i16 = 100;
    let expected = false;

    // When
    let result = Sut::contains(&unnaturally_hot);

    // Then
    assert!(result == expected);
}
