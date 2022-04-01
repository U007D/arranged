use super::*;
use assert2::assert;

type Range<const START: i8, const END: i8> = RiI8<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const LEFT: i8 = -1;
    const RIGHT: i8 = 1;

    type Sut = Range<LEFT, RIGHT>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
