use super::*;
use assert2::assert;

type Range<const START: u128, const END: u128> = RiU128<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    const MEAN: u128 = 550_000_000_000_000_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&MEAN);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    const LOW: u128 = 100_000_000_000_000_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&LOW);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    const HIGH: u128 = 1_000_000_000_000_000_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&HIGH);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    const MEAN: u128 = 99_999_999_999_999_999_999_999;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&MEAN);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    const MEAN: u128 = 1_000_000_000_000_000_000_000_001;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&MEAN);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let mean: u128 = 550_000_000_000_000_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&mean);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let low: u128 = 100_000_000_000_000_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&low);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let high: u128 = 1_000_000_000_000_000_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&high);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let mean: u128 = 99_999_999_999_999_999_999_999;
    let expected = false;

    // When
    let result = Sut::contains(&mean);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let mean: u128 = 1_000_000_000_000_000_000_000_001;
    let expected = false;

    // When
    let result = Sut::contains(&mean);

    // Then
    assert!(result == expected);
}
