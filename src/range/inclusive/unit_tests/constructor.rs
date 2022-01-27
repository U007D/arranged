use super::*;
use assert2::assert;
use std::path::PathBuf;

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

#[test]
fn invalid_range_does_not_construct() {
    // Given
    let try_build = trybuild::TestCases::new();
    let path = PathBuf::from("tests/compile_fail/invalid_range_does_not_construct.rs");

    // Then
    try_build.compile_fail(path);
}

// #[test]
// #[allow(clippy::missing_const_for_fn)]
// fn invalid_range_does_not_construct_local() {
//     // Given
//     const MIN_BOUND: i8 = 42;
//     const MAX_BOUND: i8 = -42;
//     let _actual = RiI8::<MIN_BOUND, MAX_BOUND>;
// }
