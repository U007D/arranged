use super::*;
use assert2::assert;

type ValueType = i64;
type Range<const START: ValueType, const END: ValueType> = RiI64<START, END>;

#[test]
fn range_is_expected_len() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: ValueType = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: ValueType = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    let expected = Some(4_731_899_897);

    // When
    let result = Sut::len();

    // Then
    assert!(result == expected);
}
