use super::*;
use assert2::assert;

type Range<const START: i8, const END: i8> = RiI8<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const LEFT: i8 = -1;
    const RIGHT: i8 = 1;

    type Sut = Range<LEFT, RIGHT>;

    let expected = Some(3);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
