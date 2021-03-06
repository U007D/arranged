use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: isize, const END: isize> = RiIsize<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const MIN: isize = 42;
    const MAX: isize = 84;

    type Sut = Range<MIN, MAX>;

    let expected = MIN..=MAX;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
