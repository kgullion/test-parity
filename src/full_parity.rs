use crate::Mask;

// ====

#[inline]
pub fn naive_full(a: Mask, mut b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let mut p = (a & b & qmask).count_ones();
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
pub fn aap_full(a: Mask, b: Mask, pmask: Mask, _qmask: Mask, dims: u32) -> bool {
    let mut s = a & b & pmask;
    for k in 0..dims {
        s ^= (a >> k) & b;
    }
    s.count_ones() & 1 != 0
}

#[inline]
pub fn fun_aap_full(a: Mask, b: Mask, pmask: Mask, _qmask: Mask, dims: u32) -> bool {
    (0..dims)
        .map(|k| (a >> k) & b)
        .fold(a & b & pmask, std::ops::BitXor::bitxor)
        .count_ones()
        & 1
        != 0
}

// ====

#[inline]
pub fn gerenuk_full(mut a: Mask, mut b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let negs = a & b & qmask;
    a >>= 1;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    (((b ^ (b << 1)) & a) ^ negs).count_ones() & 1 != 0
}

#[inline]
pub fn gerenuk_late_a_rsh_full(
    a: Mask,
    mut b: Mask,
    _pmask: Mask,
    qmask: Mask,
    _dims: u32,
) -> bool {
    let negs = a & b & qmask;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    (((b ^ (b << 1)) & (a >> 1)) ^ negs).count_ones() & 1 != 0
}

#[inline]
pub fn gerenuk_no_a_rsh_full(a: Mask, mut b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let negs = a & b & qmask;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    ((((b << 1) ^ (b << 2)) & a) ^ negs).count_ones() & 1 != 0
}
#[inline]
pub fn gerenuk_full_per(mut a: Mask, mut b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let negs = a & b & qmask;
    a >>= 1;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b &= a;
    b ^= negs;
    b.count_ones() & 1 != 0
}

#[inline]
pub fn gerenuk_late_a_rsh_full_per(
    a: Mask,
    mut b: Mask,
    _pmask: Mask,
    qmask: Mask,
    _dims: u32,
) -> bool {
    let negs = a & b & qmask;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b &= a >> 1;
    b ^= negs;
    b.count_ones() & 1 != 0
}

#[inline]
pub fn gerenuk_no_a_rsh_full_per(
    a: Mask,
    mut b: Mask,
    _pmask: Mask,
    qmask: Mask,
    _dims: u32,
) -> bool {
    let negs = a & b & qmask;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b <<= 1;
    b &= a;
    b ^= negs;
    b.count_ones() & 1 != 0
}

#[inline]
pub fn gerenuk_curried(
    mut b: Mask,
    _pmask: Mask,
    qmask: Mask,
    _dims: u32,
) -> impl Fn(Mask) -> bool {
    let negs = b & qmask;
    b ^= b << 32;
    b ^= b << 16;
    b ^= b << 8;
    b ^= b << 4;
    b ^= b << 2;
    b ^= b << 1;
    b <<= 1;
    b ^= negs;
    move |a| (a & b).count_ones() & 1 != 0
}

#[inline]
pub fn antelope_curried(qmask: Mask, mut a: Mask) -> impl Fn(Mask) -> bool {
    let negs = a & qmask;
    a ^= a >> 32;
    a ^= a >> 16;
    a ^= a >> 8;
    a ^= a >> 4;
    a ^= a >> 2;
    a ^= a >> 1;
    a >>= 1;
    a ^= negs;
    move |b| {
        let mut b = b & a;
        b ^= b >> 32;
        b ^= b >> 16;
        b ^= b >> 8;
        b ^= b >> 4;
        b &= 0xF;
        (0x6996 >> b) & 1 != 0
    }
}

// ====

#[inline]
pub fn starfighter_full(a: Mask, b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let negs = a & b & qmask;
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
        a = _mm_xor_si128(a, _mm_set_epi64x(0, negs as i64));

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
pub unsafe fn pclmul_table(a: i64, b: i64) -> i64 {
    use std::arch::x86_64::*;

    let x = _mm_set_epi64x(0, b);
    let c = _mm_set_epi64x(0, -1_i64);
    let transformed_b = _mm_clmulepi64_si128(x, c, 0);
    let result = (a >> 1) & (_mm_cvtsi128_si64(transformed_b));
    result
}

#[inline]
pub fn pixel_full(a: Mask, b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let lu = unsafe { pclmul_table(a as i64, b as i64) as Mask };
    (lu ^ (a & b & qmask)).count_ones() & 1 != 0
}

// ====

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
#[inline]
pub unsafe fn pppt2(a: i64, b: i64) -> i64 {
    use std::arch::x86_64::*;

    let x = _mm_set_epi64x(0, b);
    let c = _mm_set_epi64x(0, -2_i64);
    let transformed_b = _mm_clmulepi64_si128(x, c, 0);
    let result = a & (_mm_cvtsi128_si64(transformed_b));
    result
}

#[inline]
pub fn pppt2_full(a: Mask, b: Mask, _pmask: Mask, qmask: Mask, _dims: u32) -> bool {
    let lu = unsafe { pppt2(a as i64, b as i64) as Mask };
    (lu ^ (a & b & qmask)).count_ones() & 1 != 0
}

// ====

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mul_parity_matches(p in 0..Mask::BITS, lhs in 0..=Mask::MAX/2, rhs in 0..=Mask::MAX/2) {
            let q = Mask::BITS - p - 1;
            let (pmask, qmask) = crate::clifford_masks(p, q);
            let dims = p + q;

            let np = naive_full(lhs, rhs, pmask, qmask, dims);

            let ap = aap_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, ap);

            let fp = fun_aap_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, fp);

            let gp = gerenuk_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, gp);

            let glr = gerenuk_late_a_rsh_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, glr);

            let gnr = gerenuk_no_a_rsh_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, gnr);

            let gp = gerenuk_full_per(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, gp);

            let glr = gerenuk_late_a_rsh_full_per(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, glr);

            let gnr = gerenuk_no_a_rsh_full_per(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, gnr);

            let gc = gerenuk_curried(rhs, pmask, qmask, dims);
            let gcp = gc(lhs);
            prop_assert_eq!(np, gcp);

            let ac = antelope_curried(qmask, lhs);
            let acp = ac(rhs);
            prop_assert_eq!(np, acp);

            // let sf = starfighter_full(lhs, rhs, pmask, qmask, dims);
            // prop_assert_eq!(np, sf);

            let pp = pixel_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, pp);

            let pp2 = pppt2_full(lhs, rhs, pmask, qmask, dims);
            prop_assert_eq!(np, pp2);
        }
    }
}
