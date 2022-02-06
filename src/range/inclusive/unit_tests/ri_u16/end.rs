use super::*;
use assert2::assert;

type ValueType = u16;
type Range<const START: ValueType, const END: ValueType> = RiU16<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: ValueType = 206;
    const MAX_BONES_IN_HUMAN_BODY: ValueType = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    const EXPECTED: ValueType = 270;

    // When
    const RESULT: ValueType = Sut::end();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
