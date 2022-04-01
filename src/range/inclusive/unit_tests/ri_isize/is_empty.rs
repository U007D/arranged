use super::*;
use assert2::assert;

type Range<const START: isize, const END: isize> = RiIsize<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN: isize = 0;
    const MAX: isize = 11;

    type Sut = Range<MIN, MAX>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
