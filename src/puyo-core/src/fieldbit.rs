use std::{arch::x86_64::*, simd::{u16x8, u64x2}};
use crate::{util, m128i::M128i};
#[derive(Clone, Copy)] 
pub struct FieldBit {
    pub data: M128i,
}
impl FieldBit {
    pub fn new() -> FieldBit {
        FieldBit {
            data: M128i::new(),
        }
    }

    pub fn set_bit(&mut self, x: i8, y: i8) {
        assert!(x >= 0 && x < 6);
        
        let mut v= u16x8::splat(0);
        let v_mut_ptr = v.as_mut_array().as_mut_ptr();
        unsafe {
            _mm_store_si128(v_mut_ptr as *mut __m128i, self.data.0);
        }
        v[x as usize] |= 1 << y;
        self.data.0 = unsafe { _mm_load_si128(v_mut_ptr as *const __m128i) };
    }

    pub fn get_bit(&self, x: i8, y: i8) -> bool {
        if x < 0 || x > 5 || y < 0 || y > 12 {
            return true;
        }
        let mut v = u16x8::splat(0);
        let v_mut_ptr = v.as_mut_array().as_mut_ptr();
        
        unsafe {
            _mm_store_si128(v_mut_ptr as *mut __m128i, self.data.0);
        }
        v[x as usize] & (1 << y) != 0
    }

    pub fn get_count(&self) -> u32 {
        let mut v = u64x2::splat(0);
        let v_mut_ptr = v.as_mut_array().as_mut_ptr();
        unsafe {
            _mm_store_si128(v_mut_ptr as *mut __m128i, self.data.0);
        }
        v[0].count_ones().wrapping_add(v[1].count_ones())
    }

    pub fn get_col(&self, x: i8) -> u16 {
        assert!(x >= 0 && x < 6);
                let mut v= u16x8::splat(0);
        let v_mut_ptr = v.as_mut_array().as_mut_ptr();
        unsafe {
            _mm_store_si128(v_mut_ptr as *mut __m128i, self.data.0);
        }
        v[x as usize]
    }

    pub fn get_expand(& mut self) -> FieldBit {
        let r = M128i(unsafe { _mm_srli_si128(self.data.0, 2) });
        let l = M128i(unsafe { _mm_slli_si128(self.data.0, 2) });
        let u = M128i(unsafe { _mm_srli_epi16(self.data.0, 1) });
        let d = M128i(unsafe { _mm_slli_epi16(self.data.0, 1) });
        self.data = self.data | (r | l | u | d);
        return self.clone();
    }

    fn get_mask_12(&self) -> FieldBit {
        let mut result = FieldBit::new();
        result.data.0 = unsafe {
            _mm_and_si128(
                self.data.0,
                _mm_set_epi16(0, 0, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF),
            )
        };
        result
    }

    fn get_mask_13(&self) -> FieldBit {
        let mut result = FieldBit::new();
        result.data.0 = unsafe {
            _mm_and_si128(
                self.data.0,
                _mm_set_epi16(0, 0, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF),
            )
        };
        result
    }

    pub unsafe fn get_mask_pop(&self) -> FieldBit {
        let m12 = self.get_mask_12();
        let r = M128i(_mm_srli_si128(self.data.0, 2)) & m12.data;
        let l = M128i(_mm_slli_si128(self.data.0, 2)) & m12.data;
        let u = M128i(_mm_srli_epi16(self.data.0, 1)) & m12.data;
        let d = M128i(_mm_slli_epi16(self.data.0, 1)) & m12.data;

        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;

        let m3 = (ud_and & lr_or) | (lr_and & ud_or);
        let m2 = ud_and | lr_and | (ud_or & lr_or);
        let m2_r = M128i(_mm_srli_si128(m2.0, 2)) & m2;
        let m2_l = M128i(_mm_slli_si128(m2.0, 2)) & m2;
        let m2_u = M128i(_mm_srli_epi16(m2.0, 1)) & m2;
        let m2_d = M128i(_mm_slli_epi16(m2.0, 1)) & m2;

        let mut result = FieldBit::new();
        result.data = m3 | m2_r | m2_l | m2_u | m2_d;
        result = result.get_expand();
        result.data = result.data & m12.data;
        return result;
    }

    fn get_mask_group(&self, x: i8, y: i8) -> FieldBit {
        let m12 = self.get_mask_12();

        let mut m = FieldBit::new();
        m.set_bit(x, y);

        while !self.data == m.data {
            let m_expand = m.get_expand().data & m12.data;
            if unsafe { _mm_testc_si128(m.data.0, m_expand.0) != 0 } {
                break;
            }
            m.data = m_expand;
        }

        m
    }

    fn get_mask_group_4(&self, x: i8, y: i8) -> FieldBit {
        let m12 = self.get_mask_12();

        let mut m = FieldBit::new();
        m.set_bit(x, y);

        for _ in 0..4 {
            let m_expand = m.get_expand().data & m12.data;
            if unsafe { _mm_testc_si128(m.data.0, m_expand.0) != 0 } {
                break;
            }
            m.data = m_expand;
        }

        m
    }

    pub fn get_mask_group_lsb(&self) -> FieldBit {
        let m12 = self.get_mask_12();

        let mut v = u64x2::splat(0);
        let v_mut_ptr = v.as_mut_array().as_mut_ptr();
        let v_ptr = v.as_array().as_ptr();
        unsafe {
            _mm_store_si128(v_mut_ptr as *mut __m128i, m12.data.0);
        }

        if v[0] == 0 {
            v[1] &= (!v[1]).wrapping_add(1);
        } else {
            v[0] &= (!v[0]).wrapping_add(1);
            v[1] = 0;
        }

        let mut m = FieldBit::new();
        m.data.0 = unsafe { _mm_load_si128(v_ptr as *const __m128i) };
        
        while !self.data == m.data {
            let m_expand = m.get_expand().data & m12.data;
            if unsafe { _mm_testc_si128(m.data.0, m_expand.0) != 0 } {
                break;
            }
            m.data = m_expand;
        }

        m
    }

    pub fn pop(&mut self, mask: &FieldBit) {
        let mut v= u16x8::splat(0);
        let v_mut_ptr = v.as_mut_array().as_mut_ptr();
        let v_ptr = v.as_array().as_ptr();
        let mut v_mask = u16x8::splat(0);
        let v_mask_mut_ptr = v_mask.as_mut_array().as_mut_ptr();
        unsafe {
            _mm_store_si128(v_mut_ptr as *mut __m128i, self.data.0);
            _mm_store_si128(v_mask_mut_ptr as *mut __m128i, mask.data.0);
        }

        for i in 0..6 {
            v[i] = util::pext16(v[i], !v_mask[i]);
        }

        self.data.0 = unsafe { _mm_load_si128(v_ptr as *const __m128i) };
    }

    pub fn print(&self) {
        for i in (0..6).rev() {
            println!("{:013b}", self.get_col(i));
        }
        println!();
    }
}

impl core::cmp::PartialEq for FieldBit {
    fn eq(&self, other: &FieldBit) -> bool {
        self.data == other.data
    }
}
impl core::ops::Not for FieldBit {
    type Output = FieldBit;
    fn not(self) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = !self.data;
        result
    }
}
impl core::ops::BitAnd for FieldBit {
    type Output = FieldBit;
    fn bitand(self, other: FieldBit) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = self.data & other.data;
        result
    }
}
impl core::ops::BitOr for FieldBit {
    type Output = FieldBit;
    fn bitor(self, other: FieldBit) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = self.data | other.data;
        result
    }
}
impl core::ops::BitXor for FieldBit {
    type Output = FieldBit;
    fn bitxor(self, other: FieldBit) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = self.data ^ other.data;
        result
    }
}
