use super::*;
use assert2::assert;

type Range<const START: u32, const END: u32> = RiU32<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN_CPU_HZ: u32 = 350_000_000;
    const MAX_CPU_HZ: u32 = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const EXPECTED: u32 = 350_000_000;

    // When
    const RESULT: u32 = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
