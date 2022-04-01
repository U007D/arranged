use super::*;
use assert2::assert;

type Range<const START: u16, const END: u16> = RiU16<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const EXPECTED: u16 = 206;

    // When
    const RESULT: u16 = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
