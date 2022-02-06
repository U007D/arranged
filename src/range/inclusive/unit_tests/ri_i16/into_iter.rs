use super::*;
use assert2::assert;
use std::any::Any;

type ValueType = i16;
type Range<const START: ValueType, const END: ValueType> = RiI16<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const MIN_BONES_IN_HUMAN_BODY: ValueType = 206;
    const MAX_BONES_IN_HUMAN_BODY: ValueType = 270;

    type Sut = Range<MIN_BONES_IN_HUMAN_BODY, MAX_BONES_IN_HUMAN_BODY>;

    let expected = MIN_BONES_IN_HUMAN_BODY..=MAX_BONES_IN_HUMAN_BODY;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
