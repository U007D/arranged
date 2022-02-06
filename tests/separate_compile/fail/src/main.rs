use arranged::RiI8;

fn main() {
    const MIN_BOUND: i8 = 43;
    const MAX_BOUND: i8 = -43;
    drop(RiI8::<MIN_BOUND, MAX_BOUND>);
}
