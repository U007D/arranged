use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: i16, const END: i16> = RiI16<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: i16 = 206;
    const MAX_BONES_IN_HUMAN_BODY: i16 = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let expected = MIN_BONES_IN_HUMAN_BODY..=MAX_BONES_IN_HUMAN_BODY;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
