use super::*;
use assert2::assert;

type ValueType = i128;
type Range<const START: ValueType, const END: ValueType> = RiI128<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND: ValueType = 100_000_000_000_000_000_000_000;
    const STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND: ValueType = 1_000_000_000_000_000_000_000_000;

    type Sut = Range<STARS_IN_VISIBLE_UNIVERSE_LOWER_BOUND, STARS_IN_VISIBLE_UNIVERSE_UPPER_BOUND>;

    const EXPECTED: ValueType = 1_000_000_000_000_000_000_000_000;

    // When
    const RESULT: ValueType = Sut::end();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
