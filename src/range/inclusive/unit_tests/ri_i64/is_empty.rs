use super::*;
use assert2::assert;

type Range<const START: i64, const END: i64> = RiI64<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: i64 = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: i64 = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
