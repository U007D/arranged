use super::*;
use assert2::assert;

type Range<const START: i16, const END: i16> = RiI16<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: i16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: i16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const EXPECTED: i16 = 270;

    // When
    const RESULT: i16 = Sut::end();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
