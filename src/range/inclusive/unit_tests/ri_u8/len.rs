use super::*;
use assert2::assert;

type Range<const START: u8, const END: u8> = RiU8<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN: u8 = 0;
    const MAX: u8 = 11;

    type Sut = Range<MIN, MAX>;

    let expected = Some(12);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
