use super::*;
use assert2::assert;

type Range<const START: u8, const END: u8> = RiU8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN: u8 = 42;
    const MAX_VOLUME: u8 = 84;

    type Sut = Range<MIN, MAX_VOLUME>;

    const EXPECTED: u8 = 42;

    // When
    const RESULT: u8 = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
