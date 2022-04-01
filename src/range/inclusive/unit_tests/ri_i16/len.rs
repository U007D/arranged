use super::*;
use assert2::assert;

type Range<const START: i16, const END: i16> = RiI16<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: i16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: i16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let expected = Some(65);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
