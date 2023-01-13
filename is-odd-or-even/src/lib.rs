pub fn is_odd_or_even(val: u32) -> bool {
    if cfg!(feature = "even") {
        val % 2 == 0
    } else {
        val % 2 != 0
    }
}
