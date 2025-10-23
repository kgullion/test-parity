use std::ops::BitXor;

use abstalg::Domain;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMetric {
    /// number of basis elements with positive square
    pub positive: u32,
    /// number of basis elements with negative square
    pub negative: u32,
    /// total number of basis elements
    pub dimensions: u32,
    /// bitmask representing all basis elements
    pub supremum: u32,
    /// bitmask representing the basis elements with positive square
    pub hypermum: u32,
    /// bitmask representing the basis elements with negative square
    pub imagimum: u32,
    /// bitmask representing the empty basis element, corresponds to the scalar 1
    pub infinum: u32,
}

impl Domain for FrameMetric {
    type Elem = u32;
    fn equals(&self, elem1: &Self::Elem, elem2: &Self::Elem) -> bool {
        elem1 == elem2
    }
    fn contains(&self, elem: &Self::Elem) -> bool {
        self.supremum & *elem == *elem
    }
}

fn mask(n: u32) -> u32 {
    match n {
        0 => 0,
        32 => !0,
        1..32 => (1 << n) - 1,
        _ => panic!("n must be in 0..=32"),
    }
}

#[inline]
#[must_use]
pub fn parity(n: u32) -> bool {
    n.count_ones() & 1 != 0
}

impl FrameMetric {
    /// Create a new FrameMetric given the count of positive and negative basis elements.
    #[inline]
    #[must_use]
    pub fn new(positive: u32, negative: u32) -> Self {
        Self {
            positive,
            negative,
            dimensions: positive + negative,
            supremum: mask(positive + negative),
            hypermum: mask(positive),
            imagimum: mask(negative) << positive.clamp(0, 31),
            infinum: 0,
        }
    }
    /// Determine the swap parity between two basis elements. Does not account for metric.
    #[inline]
    #[must_use]
    pub fn swap_parity(&self, lhs: u32, mut rhs: u32) -> bool {
        let mut parity = false;
        while rhs != 0 {
            let rhs_bit = rhs & (!rhs + 1); // least significant bit of rhs
            rhs ^= rhs_bit; // clear least significant bit
            let mask_lower = rhs_bit - 1; // masks for bits below rhs_bit
            let mask_upper = !mask_lower ^ rhs_bit; // masks for bits above
            let lhs_upper = lhs & mask_upper; // bits rhs_bit needs to move past
            let swaps = lhs_upper.count_ones(); // how many swaps needed
            parity ^= swaps & 1 != 0; // flip parity if odd number of swaps
        }
        parity
    }
    /// Simplified version of swap_parity
    #[inline]
    #[must_use]
    pub fn aap_parity(&self, lhs: u32, rhs: u32) -> bool {
        let mut s = 0;
        for k in 1..self.dimensions {
            s ^= (lhs >> k) & rhs;
        }
        s.count_ones() & 1 != 0
    }
    /// Functional simple version of aap_parity
    #[inline]
    #[must_use]
    pub fn fun_aap_parity(&self, lhs: u32, rhs: u32) -> bool {
        (1..self.dimensions)
            .map(|k| (lhs >> k) & rhs)
            .fold(0, BitXor::bitxor)
            .count_ones()
            & 1
            != 0
    }
    /// Determine the metric parity of a basis element. Does not account for swaps.
    #[inline]
    #[must_use]
    pub fn metric_parity(&self, basis: u32) -> bool {
        // the metric is determined by the oddness of the number of negative basis elements
        let negative_count = (basis & self.imagimum).count_ones();
        negative_count % 2 != 0
    }
    /// Determine the multiplication parity of two basis elements.
    #[inline]
    #[must_use]
    pub fn mul_parity(&self, lhs: u32, rhs: u32) -> bool {
        self.swap_parity(lhs, rhs) ^ self.metric_parity(lhs & rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mul_parity_matches_aap_mul_parity(positive in 0u32..=32, negative in 0u32..=32, lhs in any::<u32>(), rhs in any::<u32>()) {
            prop_assume!(positive + negative <= 32);

            let metric = FrameMetric::new(positive, negative);
            let mask = metric.supremum;
            let lhs = lhs & mask;
            let rhs = rhs & mask;

            let sp = metric.swap_parity(lhs, rhs);
            let ap = metric.aap_parity(lhs, rhs);
            let fp = metric.fun_aap_parity(lhs, rhs);

            prop_assert_eq!(sp, ap);
            prop_assert_eq!(sp, fp);
        }
    }
}
