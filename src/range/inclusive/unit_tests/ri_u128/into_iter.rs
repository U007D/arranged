use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: u128, const END: u128> = RiU128<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let expected = STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND..=STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
