use super::*;
use assert2::assert;

type Range<const START: usize, const END: usize> = RiUsize<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN: usize = 0;
    const MAX: usize = 11;

    type Sut = Range<MIN, MAX>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
