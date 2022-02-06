use super::*;
use assert2::assert;

type ValueType = usize;
type Range<const START: ValueType, const END: ValueType> = RiUsize<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN: ValueType = 0;
    const MAX: ValueType = 11;

    type Sut = Range<MIN, MAX>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
