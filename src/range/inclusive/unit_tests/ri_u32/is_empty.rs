use super::*;
use assert2::assert;

type Range<const START: u32, const END: u32> = RiU32<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN_CPU_HZ: u32 = 350_000_000;
    const MAX_CPU_HZ: u32 = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
