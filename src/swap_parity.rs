use crate::Mask;

// ====

/// Count swaps needed to merge two basis elements.
#[inline]
pub fn naive_swap(a: Mask, mut b: Mask) -> bool {
    let mut p = 0;
    while b != 0 {
        let lsb = b & (!b + 1); // least significant bit of rhs
        b ^= lsb; // clear least significant bit
        let mask_lower = lsb - 1; // mask for bits below lsb
        let mask_upper = !mask_lower ^ lsb; // mask for bits above
        let a_upper = a & mask_upper; // bits lsb needs to move past
        p ^= a_upper.count_ones(); // flip parity if odd number of swaps
    }
    p & 1 != 0
}

// ====

#[inline]
pub fn aap_swap(a: Mask, b: Mask) -> bool {
    let mut s = a & b;
    for k in 0..Mask::BITS {
        s ^= (a >> k) & b;
    }
    s.count_ones() & 1 != 0
}

#[inline]
pub fn fun_aap_swap(a: Mask, b: Mask) -> bool {
    (0..Mask::BITS)
        .map(|k| (a >> k) & b)
        .fold(a & b, std::ops::BitXor::bitxor)
        .count_ones()
        & 1
        != 0
}

// ====

/// Unrolled bitwise version of swap_parity
#[inline]
pub fn gerenuk_swap(mut a: Mask, mut b: Mask) -> bool {
    a >>= 1;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b &= a;
    b.count_ones() & 1 != 0
}
/// Unrolled bitwise version of swap_parity with late a shift
#[inline]
pub fn gerenuk_late_a_rsh_swap(a: Mask, mut b: Mask) -> bool {
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b &= a >> 1;
    b.count_ones() & 1 != 0
}
/// Unrolled bitwise version of swap_parity with no a shift
#[inline]
pub fn gerenuk_no_a_rsh_swap(a: Mask, mut b: Mask) -> bool {
    b <<= 1;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b &= a;
    b.count_ones() & 1 != 0
}

// ====

pub fn starfighter_swap(a: Mask, b: Mask) -> bool {
    use std::arch::asm;
    use std::arch::x86_64::*;
    unsafe {
        let mut a = _mm_set_epi64x(0, a as i64);
        let mut b = _mm_set_epi64x(0, b as i64);

        // a = _mm_srai_epi64(a, 1);
        asm!(
            "vpsraq {0}, {0}, 1",
            in(xmm_reg) a,
        );

        let all_ones = _mm_set_epi64x(0, -1i64);
        b = _mm_clmulepi64_si128(b, all_ones, 0x00);

        a = _mm_and_si128(a, b);
        // a = _mm_popcnt_epi64(a);
        asm!(
            "vpopcntq {0}, {0}",
            in(xmm_reg) a,
        );

        _mm_cvtsi128_si64(a) & 1 != 0
    }
}

// ====

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
#[inline]
pub unsafe fn prod_parity_pclmul(a: i64, b: i64) -> u32 {
    use std::arch::x86_64::*;

    let x = _mm_set_epi64x(0, b);
    let c = _mm_set_epi64x(0, -1_i64);
    let transformed_b = _mm_clmulepi64_si128(x, c, 0);
    let result = (a >> 1) & (_mm_cvtsi128_si64(transformed_b));
    result.count_ones()
}
#[inline]
pub fn pixel_swap(a: Mask, b: Mask) -> bool {
    unsafe { prod_parity_pclmul(a as i64, b as i64) & 1 != 0 }
}

// ====

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
#[inline]
pub unsafe fn ppp2(a: i64, b: i64) -> u32 {
    use std::arch::x86_64::*;

    let x = _mm_set_epi64x(0, b);
    let c = _mm_set_epi64x(0, -2_i64);
    let transformed_b = _mm_clmulepi64_si128(x, c, 0);
    let result = a & _mm_cvtsi128_si64(transformed_b);
    result.count_ones()
}
#[inline]
pub fn ppp2_swap(a: Mask, b: Mask) -> bool {
    unsafe { ppp2(a as i64, b as i64) & 1 != 0 }
}

// ====

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mul_parity_matches(a in 0..Mask::MAX/2, b in 0..Mask::MAX/2) {
            let np = naive_swap(a, b);

            let gs = gerenuk_swap(a, b);
            prop_assert_eq!(np, gs);

            let glr = gerenuk_late_a_rsh_swap(a, b);
            prop_assert_eq!(np, glr);

            let gnr = gerenuk_no_a_rsh_swap(a, b);
            prop_assert_eq!(np, gnr);

            // let sf = starfighter_swap(a, b);
            // prop_assert_eq!(np, sf);

            let ps = pixel_swap(a, b);
            prop_assert_eq!(np, ps);

            let ppp2 = ppp2_swap(a, b);
            prop_assert_eq!(np, ppp2);
        }
    }
}
