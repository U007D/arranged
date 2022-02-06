use crate::{traits::IRangeIntoIterator, RiU8};
use assert2::assert;

#[test]
fn returns_expected_values() {
    // Given
    let mut sut = <RiU8<42, 42> as IRangeIntoIterator>::into_iter();
    let expected = [Some(42), None];

    // When
    let result = [sut.next(), sut.next()];

    // Then
    assert!(result == expected);
}
