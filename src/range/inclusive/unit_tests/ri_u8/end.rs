use super::*;
use assert2::assert;

type ValueType = u8;
type Range<const START: ValueType, const END: ValueType> = RiU8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN: ValueType = 42;
    const MAX_VOLUME: ValueType = 84;

    type Sut = Range<MIN, MAX_VOLUME>;

    const EXPECTED: ValueType = 84;

    // When
    const RESULT: ValueType = Sut::end();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
