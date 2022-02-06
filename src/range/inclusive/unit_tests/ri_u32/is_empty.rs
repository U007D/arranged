use super::*;
use assert2::assert;

type ValueType = u32;
type Range<const START: ValueType, const END: ValueType> = RiU32<START, END>;

#[test]
fn non_empty_range_returns_expected_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let expected = false;

    // When
    let result = Sut::is_empty();

    // Then
    assert!(result == expected);
}
