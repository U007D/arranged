use super::*;
use assert2::assert;

type Range<const START: i128, const END: i128> = RiI128<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: i128 = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: i128 = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    #[cfg(target_pointer_width = "128")]
    let expected = Some(900_000_000_000_000_000_000_000);

    #[cfg(not(target_pointer_width = "128"))]
    // `UPPER_BOUND` - `LOWER_BOUND` > isize::MAX
    let expected = None;

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
