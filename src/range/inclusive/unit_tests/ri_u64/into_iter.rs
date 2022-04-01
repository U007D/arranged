use super::*;
use assert2::assert;
use std::any::Any;

type Range<const START: u64, const END: u64> = RiU64<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: u64 = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: u64 = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let expected = WORLD_POPULATION_AT_START_OF_CENTURY..=WORLD_POPULATION_AT_END_OF_CENTURY;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
