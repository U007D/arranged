use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: u8, const END: u8> = RiU8<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const MIN: u8 = 42;
    const MAX: u8 = 84;

    type Sut = Range<MIN, MAX>;

    let expected = MIN..=MAX;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
