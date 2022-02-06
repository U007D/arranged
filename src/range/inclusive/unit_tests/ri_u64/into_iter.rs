use super::*;
use assert2::assert;
use std::any::Any;

type ValueType = u64;
type Range<const START: ValueType, const END: ValueType> = RiU64<START, END>;

#[test]
#[allow(clippy::assertions_on_constants, clippy::items_after_statements)]
fn returns_expected_iterator() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let expected = WORLD_POPULATION_AT_START_OF_CENTURY..=WORLD_POPULATION_AT_END_OF_CENTURY;

    // When
    let result = <Sut as IRangeIntoIterator>::into_iter();

    // Then
    std::assert!(result.range == expected);
}
