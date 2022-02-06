use super::*;
use assert2::assert;
use std::any::Any;

type ValueType = i8;
type Range<const START: ValueType, const END: ValueType> = RiI8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const LEFT: ValueType = -1;
    const RIGHT: ValueType = 1;

    type Sut = Range<LEFT, RIGHT>;

    let expected = LEFT..=RIGHT;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
