use super::*;
use assert2::assert;

type Range<const START: usize, const END: usize> = RiUsize<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN: usize = 42;
    const MAX_VOLUME: usize = 84;

    type Sut = Range<MIN, MAX_VOLUME>;

    const EXPECTED: usize = 42;

    // When
    const RESULT: usize = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
