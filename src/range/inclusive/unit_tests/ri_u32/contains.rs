use super::*;
use assert2::assert;

type ValueType = u32;
type Range<const START: ValueType, const END: ValueType> = RiU32<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: ValueType = 1_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: ValueType = 350_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: ValueType = 1_400_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: ValueType = 349_999_999;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: ValueType = 1_400_000_001;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&FREQ);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq: ValueType = 1_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq: ValueType = 350_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq: ValueType = 1_400_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq: ValueType = 349_999_999;
    let expected = false;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let freq: ValueType = 1_400_000_001;
    let expected = false;

    // When
    let result = Sut::contains(&freq);

    // Then
    assert!(result == expected);
}
