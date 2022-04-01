use super::*;
use assert2::assert;

type Range<const START: usize, const END: usize> = RiUsize<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN: usize = 0;
    const MAX: usize = 11;

    type Sut = Range<MIN, MAX>;

    let expected = Some(12);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
