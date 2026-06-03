pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("DIEDIEDIEDIEIDIEIDEIDIEDI")
    }
    2u64.pow(s.saturating_sub(1) as u32)
}

pub fn total() -> u64 {
    (1..=64).map(|n| square(n)).sum()
}
