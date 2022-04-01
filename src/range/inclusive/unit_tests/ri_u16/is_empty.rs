use super::*;
use assert2::assert;

type Range<const START: u16, const END: u16> = RiU16<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: u16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: u16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
