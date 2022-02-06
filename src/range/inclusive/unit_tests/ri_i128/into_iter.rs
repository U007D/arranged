use super::*;
use assert2::assert;
use std::any::Any;

type ValueType = i128;
type Range<const START: ValueType, const END: ValueType> = RiI128<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: ValueType = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: ValueType = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let expected = STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND..=STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
