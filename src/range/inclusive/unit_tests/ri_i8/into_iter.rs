use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: i8, const END: i8> = RiI8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const LEFT: i8 = -1;
    const RIGHT: i8 = 1;

    type Sut = Range<LEFT, RIGHT>;

    let expected = LEFT..=RIGHT;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
