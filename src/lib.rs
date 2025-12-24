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

#[inline(always)]
#[must_use]
pub fn parity(mut a: u32) -> bool {
    a ^= a >> 16;
    a ^= a >> 8;
    a ^= a >> 4;
    a &= 0xF;
    (0x6996 >> a) & 1 != 0
}

#[inline(always)]
#[must_use]
pub fn fast_parity(a: u32) -> bool {
    a.count_ones() & 1 != 0
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
        let mut p = false;
        while rhs != 0 {
            let rhs_bit = rhs & (!rhs + 1); // least significant bit of rhs
            rhs ^= rhs_bit; // clear least significant bit
            let mask_lower = rhs_bit - 1; // masks for bits below rhs_bit
            let mask_upper = !mask_lower ^ rhs_bit; // masks for bits above
            let lhs_upper = lhs & mask_upper; // bits rhs_bit needs to move past
            p ^= parity(lhs_upper); // flip parity if odd number of swaps
        }
        p
    }

    /// Determine the swap parity between two basis elements, using `fast_parity`. Does not account for metric.
    #[inline]
    #[must_use]
    pub fn swap_parity_fast(&self, lhs: u32, mut rhs: u32) -> bool {
        let mut p = false;
        while rhs != 0 {
            let rhs_bit = rhs & (!rhs + 1); // least significant bit of rhs
            rhs ^= rhs_bit; // clear least significant bit
            let mask_lower = rhs_bit - 1; // masks for bits below rhs_bit
            let mask_upper = !mask_lower ^ rhs_bit; // masks for bits above
            let lhs_upper = lhs & mask_upper; // bits rhs_bit needs to move past
            p ^= fast_parity(lhs_upper); // flip parity if odd number of swaps
        }
        p
    }
    /// Unrolled bitwise version of swap_parity
    #[inline]
    #[must_use]
    pub fn gerenuk_parity(&self, a: u32, mut b: u32) -> bool {
        b ^= b << 16;
        b ^= b << 8;
        b ^= b << 4;
        b ^= b << 2;
        b ^= b << 1;
        b &= a >> 1;
        parity(b)
    }

    /// Unrolled bitwise version of swap_parity, using `fast_parity`.
    #[inline]
    #[must_use]
    pub fn gerenuk_parity_fast(&self, a: u32, mut b: u32) -> bool {
        b ^= b << 16;
        b ^= b << 8;
        b ^= b << 4;
        b ^= b << 2;
        b ^= b << 1;
        b &= a >> 1;
        fast_parity(b)
    }
    /// Determine the metric parity of a basis element. Does not account for swaps.
    #[inline]
    #[must_use]
    pub fn metric_parity(&self, basis: u32) -> bool {
        // the metric is determined by the oddness of the number of negative basis elements
        parity(basis & self.imagimum)
    }

    /// Determine the metric parity of a basis element, using `fast_parity`. Does not account for swaps.
    #[inline]
    #[must_use]
    pub fn metric_parity_fast(&self, basis: u32) -> bool {
        // the metric is determined by the oddness of the number of negative basis elements
        fast_parity(basis & self.imagimum)
    }
    /// Determine the multiplication parity of two basis elements.
    #[inline]
    #[must_use]
    pub fn mul_parity(&self, lhs: u32, rhs: u32) -> bool {
        self.swap_parity(lhs, rhs) ^ self.metric_parity(lhs & rhs)
    }

    /// Determine the multiplication parity of two basis elements, using `fast_parity` internally.
    #[inline]
    #[must_use]
    pub fn mul_parity_fast(&self, lhs: u32, rhs: u32) -> bool {
        self.swap_parity_fast(lhs, rhs) ^ self.metric_parity_fast(lhs & rhs)
    }
    /// Simplified version of mul_parity
    #[inline]
    #[must_use]
    pub fn aap_parity(&self, lhs: u32, rhs: u32) -> bool {
        let mut s = self.hypermum & lhs & rhs;
        for k in 0..self.dimensions {
            s ^= (lhs >> k) & rhs;
        }
        parity(s)
    }

    /// Simplified version of mul_parity, using `fast_parity`.
    #[inline]
    #[must_use]
    pub fn aap_parity_fast(&self, lhs: u32, rhs: u32) -> bool {
        let mut s = self.hypermum & lhs & rhs;
        for k in 0..self.dimensions {
            s ^= (lhs >> k) & rhs;
        }
        fast_parity(s)
    }
    /// Functional simple version of aap_parity
    #[inline]
    #[must_use]
    pub fn fun_aap_parity(&self, lhs: u32, rhs: u32) -> bool {
        parity(
            (0..self.dimensions)
                .map(|k| (lhs >> k) & rhs)
                .fold(self.hypermum & lhs & rhs, BitXor::bitxor),
        )
    }

    /// Functional simple version of aap_parity, using `fast_parity`.
    #[inline]
    #[must_use]
    pub fn fun_aap_parity_fast(&self, lhs: u32, rhs: u32) -> bool {
        fast_parity(
            (0..self.dimensions)
                .map(|k| (lhs >> k) & rhs)
                .fold(self.hypermum & lhs & rhs, BitXor::bitxor),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn parity_matches_fast_parity(a in any::<u32>()) {
            prop_assert_eq!(parity(a), fast_parity(a));
        }

        #[test]
        fn mul_parity_matches_aap_mul_parity(positive in 1u32..=32, lhs in any::<u32>(), rhs in any::<u32>()) {
            // prop_assume!(positive + negative <= 32);

            let metric = FrameMetric::new(positive, 0);
            let mask = metric.supremum;
            let lhs = lhs & mask;
            let rhs = rhs & mask;

            let sp = metric.mul_parity(lhs, rhs);
            let ap = metric.aap_parity(lhs, rhs);
            let fp = metric.fun_aap_parity(lhs, rhs);
            let gp = metric.gerenuk_parity(lhs, rhs) ^ metric.metric_parity(lhs & rhs);

            prop_assert_eq!(sp, ap);
            prop_assert_eq!(sp, fp);
            prop_assert_eq!(sp, gp);
        }

        #[test]
        fn all_algos_match_fast_variants(positive in 1u32..=32, lhs in any::<u32>(), rhs in any::<u32>()) {
            let metric = FrameMetric::new(positive, 0);
            let mask = metric.supremum;
            let lhs = lhs & mask;
            let rhs = rhs & mask;

            prop_assert_eq!(metric.swap_parity(lhs, rhs), metric.swap_parity_fast(lhs, rhs));
            prop_assert_eq!(metric.gerenuk_parity(lhs, rhs), metric.gerenuk_parity_fast(lhs, rhs));
            prop_assert_eq!(metric.metric_parity(lhs & rhs), metric.metric_parity_fast(lhs & rhs));
            prop_assert_eq!(metric.mul_parity(lhs, rhs), metric.mul_parity_fast(lhs, rhs));
            prop_assert_eq!(metric.aap_parity(lhs, rhs), metric.aap_parity_fast(lhs, rhs));
            prop_assert_eq!(metric.fun_aap_parity(lhs, rhs), metric.fun_aap_parity_fast(lhs, rhs));

            prop_assert_eq!(
                metric.gerenuk_parity(lhs, rhs) ^ metric.metric_parity(lhs & rhs),
                metric.gerenuk_parity_fast(lhs, rhs) ^ metric.metric_parity_fast(lhs & rhs)
            );
        }
    }
}
