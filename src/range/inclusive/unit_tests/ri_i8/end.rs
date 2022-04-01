use super::*;
use assert2::assert;

type Range<const START: i8, const END: i8> = RiI8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const LEFT: i8 = -1;
    const RIGHT: i8 = 1;

    type Sut = Range<LEFT, RIGHT>;

    const EXPECTED: i8 = 1;

    // When
    const RESULT: i8 = Sut::end();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
