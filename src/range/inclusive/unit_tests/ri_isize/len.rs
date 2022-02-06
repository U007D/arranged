use super::*;
use assert2::assert;

type ValueType = i16;
type Range<const START: ValueType, const END: ValueType> = RiI16<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN: ValueType = 0;
    const MAX: ValueType = 11;

    type Sut = Range<MIN, MAX>;

    let expected = Some(12);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
