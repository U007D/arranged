use arranged::RiU8;

fn main() {
    const MIN_BOUND: u8 = 42;
    const MAX_BOUND: u8 = 1;
    drop(RiU8::<MIN_BOUND, MAX_BOUND>);
}
