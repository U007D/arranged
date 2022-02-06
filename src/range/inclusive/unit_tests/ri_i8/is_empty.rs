use super::*;
use assert2::assert;

type ValueType = i8;
type Range<const START: ValueType, const END: ValueType> = RiI8<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
