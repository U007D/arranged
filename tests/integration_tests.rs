// Test to determine if `compile_fail` succeeding where it shouldn't is an artifact of Rust or `compile_fail`.
// Since the following test breaks compilation, the issue seems to be within `compile_fail`.
// TODO: Find alternative to (or fix) `compile_fail` for this use case and remove this test
// #[test]
// fn invalid_range_does_not_construct() {
//     use arranged::RiI8;
//
//     const MIN_BOUND: i8 = 43;
//     const MAX_BOUND: i8 = -43;
//     drop(RiI8::<MIN_BOUND, MAX_BOUND>);
// }
