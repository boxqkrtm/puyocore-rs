use std::arch::x86_64::*;
#[derive(Clone, Copy)]
pub struct M128i(pub __m128i);
impl M128i {
    pub fn new() -> M128i {
        M128i(unsafe { _mm_setzero_si128() })
    }
    pub fn all_bytes_one() -> M128i {
        M128i(unsafe { _mm_set_epi32(0xFF, 0xFF, 0xFF, 0xFF)})
    }
    pub fn all_bytes_zero() -> M128i {
        M128i(unsafe { _mm_setzero_si128() })
    }
}
impl core::ops::Not for M128i{
    type Output = M128i;
    fn not(self) -> M128i {
        M128i(unsafe { _mm_xor_si128(self.0, M128i::all_bytes_one().0) })
    }
}
impl core::ops::BitAnd for M128i {
    type Output = M128i;
    fn bitand(self, other: M128i) -> M128i {
        M128i(unsafe { _mm_and_si128(self.0, other.0) })
    }
}
impl core::ops::BitOr for M128i {
    type Output = M128i;
    fn bitor(self, other: M128i) -> M128i {
        M128i(unsafe { _mm_or_si128(self.0, other.0) })
    }
}
impl core::ops::BitXor for M128i {
    type Output = M128i;
    fn bitxor(self, other: M128i) -> M128i {
        M128i(unsafe { _mm_xor_si128(self.0, other.0) })
    }
}
impl core::cmp::PartialEq for M128i {
    fn eq(&self, other: &M128i) -> bool {
        let neq =unsafe {_mm_xor_si128(self.0, other.0)};
        let tested = unsafe {_mm_test_all_zeros(neq, neq)};
        tested == 1
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
