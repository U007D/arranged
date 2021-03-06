use super::*;
use assert2::assert;

type Range<const START: i64, const END: i64> = RiI64<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const WORLD_POPULATION_AT_START_OF_CENTURY: i64 = 6_143_493_823;
    const WORLD_POPULATION_AT_END_OF_CENTURY: i64 = 10_875_393_719;

    type Sut = Range<WORLD_POPULATION_AT_START_OF_CENTURY, WORLD_POPULATION_AT_END_OF_CENTURY>;

    const EXPECTED: i64 = 10_875_393_719;

    // When
    const RESULT: i64 = Sut::end();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
