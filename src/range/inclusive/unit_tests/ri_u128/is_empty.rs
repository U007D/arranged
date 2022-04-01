use super::*;
use assert2::assert;

type Range<const START: u128, const END: u128> = RiU128<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
