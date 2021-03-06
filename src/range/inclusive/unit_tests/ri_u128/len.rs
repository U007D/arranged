use super::*;
use assert2::assert;

type Range<const START: u128, const END: u128> = RiU128<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: u128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: u128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    #[cfg(target_pointer_width = "128")]
    let expected = Some(900_000_000_000_000_000_000_000);

    #[cfg(not(target_pointer_width = "128"))]
    // `UPPER_BOUND` - `LOWER_BOUND` > usize::MAX
    let expected = None;

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
