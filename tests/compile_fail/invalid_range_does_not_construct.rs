use arranged::RiI8;

fn main() {
    const MIN_BOUND: i8 = 42;
    const MAX_BOUND: i8 = -42;
    let _invalid_range = RiI8::<MIN_BOUND, MAX_BOUND>;
}
