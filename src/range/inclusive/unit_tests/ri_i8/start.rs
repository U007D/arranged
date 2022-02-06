use super::*;
use assert2::assert;

type ValueType = i8;
type Range<const START: ValueType, const END: ValueType> = RiI8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    const EXPECTED: ValueType = -1;

    // When
    const RESULT: ValueType = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
