use super::*;
use assert2::assert;

type ValueType = i16;
type Range<const START: ValueType, const END: ValueType> = RiI16<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let expected = Some(3);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
