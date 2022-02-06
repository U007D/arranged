use super::*;
use assert2::assert;

type ValueType = i128;
type Range<const START: ValueType, const END: ValueType> = RiI128<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: ValueType = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: ValueType = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
