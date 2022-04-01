use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: u32, const END: u32> = RiU32<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const MIN_CPU_HZ: u32 = 350_000_000;
    const MAX_CPU_HZ: u32 = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    let expected = MIN_CPU_HZ..=MAX_CPU_HZ;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
