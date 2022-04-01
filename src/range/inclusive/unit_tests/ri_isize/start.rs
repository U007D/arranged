use super::*;
use assert2::assert;

type Range<const START: isize, const END: isize> = RiIsize<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN: isize = 42;
    const MAX_VOLUME: isize = 84;

    type Sut = Range<MIN, MAX_VOLUME>;

    const EXPECTED: isize = 42;

    // When
    const RESULT: isize = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
