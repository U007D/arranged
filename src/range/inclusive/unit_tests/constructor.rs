use super::*;
use assert2::assert;

#[test]
#[allow(clippy::equatable_if_let)]
fn valid_range_constructs_successfully() {
    // Given
    const MIN_BOUND: i8 = -42;
    const MAX_BOUND: i8 = 42;
    let expected = RiI8::<-42, 42>;

    // When
    let result = RiI8::<MIN_BOUND, MAX_BOUND>;

    // Then
    assert!(result == expected);
}

// `compile_fail` is failing to fail on an invalid `Range` construction (see also
// `tests/integration_tests::invalid_range_does_not_construct` & `tests/compile_fail/invalid_range_does_not_construct`).
// TODO: Find alternative to (or fix) `compile_fail` and uncomment the test below.
//
// #[test]
// fn invalid_range_does_not_construct() {
//     // Given
//     let try_build = trybuild::TestCases::new();
//     let path = PathBuf::from("tests/compile_fail/invalid_range_does_not_construct.rs");
//
//     // Then
//     try_build.compile_fail(path);
// }
