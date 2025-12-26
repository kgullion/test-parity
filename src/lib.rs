pub mod full_parity;
pub mod swap_parity;

pub type Mask = u64;

/// Generate bit masks for the given number of positive and negative bits.
pub fn clifford_masks(p: u32, q: u32) -> (Mask, Mask) {
    match (p, q) {
        (Mask::BITS, 0) => (!0, 0),
        (0, Mask::BITS) => (0, !0),
        (p, q) if p + q <= Mask::BITS => ((1 << p) - 1, ((1 << q) - 1) << p),
        _ => unreachable!("Invalid mask sizes: p={}, q={}", p, q),
    }
}
