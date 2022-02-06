use super::*;
use assert2::assert;

type ValueType = i16;
type Range<const START: ValueType, const END: ValueType> = RiI16<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: ValueType = 206;
    const MAX_BONES_IN_HUMAN_BODY: ValueType = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let expected = Some(65);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
