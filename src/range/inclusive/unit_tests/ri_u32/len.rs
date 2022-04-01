use super::*;
use assert2::assert;

type Range<const START: u32, const END: u32> = RiU32<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const MIN_CPU_HZ: u32 = 350_000_000;
    const MAX_CPU_HZ: u32 = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let expected = Some(1_050_000_001);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
