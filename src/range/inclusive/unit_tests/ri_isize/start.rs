use super::*;
use assert2::assert;

type ValueType = isize;
type Range<const START: ValueType, const END: ValueType> = RiIsize<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN: ValueType = 42;
    const MAX_VOLUME: ValueType = 84;

    type Sut = Range<MIN, MAX_VOLUME>;

    const EXPECTED: ValueType = 42;

    // When
    const RESULT: ValueType = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}