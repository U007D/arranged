use super::*;
use assert2::assert;

type Range<const START: isize, const END: isize> = RiIsize<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN: isize = 0;
    const MAX: isize = 11;

    type Sut = Range<MIN, MAX>;

    let expected = Some(12);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
