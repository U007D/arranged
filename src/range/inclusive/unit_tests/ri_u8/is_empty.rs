use super::*;
use assert2::assert;

type Range<const START: u8, const END: u8> = RiU8<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN: u8 = 0;
    const MAX: u8 = 11;

    type Sut = Range<MIN, MAX>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
