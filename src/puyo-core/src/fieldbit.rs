use std::arch::x86_64::*;

use crate::util;

pub struct FieldBit {
    data: __m128i,
}

impl FieldBit {
    pub fn new() -> FieldBit {
        FieldBit {
            data: unsafe { _mm_setzero_si128() },
        }
    }

    fn equals(&self, other: &FieldBit) -> bool {
        let neq = unsafe { _mm_xor_si128(self.data, other.data) };
        unsafe { _mm_test_all_zeros(neq, neq) != 0 }
    }

    fn not_equals(&self, other: &FieldBit) -> bool {
        !self.equals(other)
    }

    fn bitwise_or(&self, other: &FieldBit) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = unsafe { _mm_or_si128(self.data, other.data) };
        result
    }

    fn bitwise_and(&self, other: &FieldBit) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = unsafe { _mm_and_si128(self.data, other.data) };
        result
    }

    fn bitwise_not(&self) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = unsafe { _mm_not_si128(self.data) };
        result
    }

    fn set_bit(&mut self, x: i8, y: i8) {
        assert!(x >= 0 && x < 6);
        let mut v: [u16; 8] = [0; 8];
        unsafe {
            _mm_store_si128(v.as_mut_ptr() as *mut __m128i, self.data);
        }
        v[x as usize] |= 1 << y;
        self.data = unsafe { _mm_load_si128(v.as_ptr() as *const __m128i) };
    }

    fn get_bit(&self, x: i8, y: i8) -> bool {
        if x < 0 || x > 5 || y < 0 || y > 12 {
            return true;
        }
        let mut v: [u16; 8] = [0; 8];
        unsafe {
            _mm_store_si128(v.as_mut_ptr() as *mut __m128i, self.data);
        }
        v[x as usize] & (1 << y) != 0
    }

    fn get_count(&self) -> u32 {
        let mut v: [u64; 2] = [0; 2];
        unsafe {
            _mm_store_si128(v.as_mut_ptr() as *mut __m128i, self.data);
        }
        v[0].count_ones() + v[1].count_ones()
    }

    fn get_col(&self, x: i8) -> u16 {
        assert!(x >= 0 && x < 6);
        let mut v: [u16; 8] = [0; 8];
        unsafe {
            _mm_store_si128(v.as_mut_ptr() as *mut __m128i, self.data);
        }
        v[x as usize]
    }

    fn get_expand(&self) -> FieldBit {
        let r = unsafe { _mm_srli_si128(self.data, 2) };
        let l = unsafe { _mm_slli_si128(self.data, 2) };
        let u = unsafe { _mm_srli_epi16(self.data, 1) };
        let d = unsafe { _mm_slli_epi16(self.data, 1) };

        let mut result = FieldBit::new();
        result.data = unsafe { _mm_or_si128(self.data, _mm_or_si128(r, _mm_or_si128(l, _mm_or_si128(u, d)))) };
        result
    }

    fn get_mask_12(&self) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = unsafe { _mm_and_si128(self.data, _mm_set_epi16(0, 0, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF)) };
        result
    }

    fn get_mask_13(&self) -> FieldBit {
        let mut result = FieldBit::new();
        result.data = unsafe { _mm_and_si128(self.data, _mm_set_epi16(0, 0, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF)) };
        result
    }

    fn get_mask_pop(&self) -> FieldBit {
        let m12 = self.get_mask_12();

        let r = unsafe { _mm_srli_si128(self.data, 2) } & m12.data;
        let l = unsafe { _mm_slli_si128(self.data, 2) } & m12.data;
        let u = unsafe { _mm_srli_epi16(self.data, 1) } & m12.data;
        let d = unsafe { _mm_slli_epi16(self.data, 1) } & m12.data;

        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;

        let m3 = (ud_and & lr_or) | (lr_and & ud_or);
        let m2 = ud_and | lr_and | (ud_or & lr_or);
        let m2_r = unsafe { _mm_srli_si128(m2, 2) } & m2;
        let m2_l = unsafe { _mm_slli_si128(m2, 2) } & m2;
        let m2_u = unsafe { _mm_srli_epi16(m2, 1) } & m2;
        let m2_d = unsafe { _mm_slli_epi16(m2, 1) } & m2;

        let mut result = FieldBit::new();
        result.data = m3 | m2_r | m2_l | m2_u | m2_d;
        result = result.get_expand();
        result.data & m12.data
    }

    fn get_mask_group(&self, x: i8, y: i8) -> FieldBit {
        let m12 = self.get_mask_12();

        let mut m = FieldBit::new();
        m.set_bit(x, y);

        while !self.data == m.data {
            let m_expand = m.get_expand().data & m12.data;
            if unsafe { _mm_testc_si128(m.data, m_expand) != 0 } {
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
            if unsafe { _mm_testc_si128(m.data, m_expand) != 0 } {
                break;
            }
            m.data = m_expand;
        }

        m
    }

    fn get_mask_group_lsb(&self) -> FieldBit {
        let m12 = self.get_mask_12();

        let mut v: [u64; 2] = [0; 2];
        unsafe {
            _mm_store_si128(v.as_mut_ptr() as *mut __m128i, m12.data);
        }

        if v[0] == 0 {
            v[1] &= !v[1] + 1;
        } else {
            v[0] &= !v[0] + 1;
            v[1] = 0;
        }

        let mut m = FieldBit::new();
        m.data = unsafe { _mm_load_si128(v.as_ptr() as *const __m128i) };

        while !self.data == m.data {
            let m_expand = m.get_expand().data & m12.data;
            if unsafe { _mm_testc_si128(m.data, m_expand) != 0 } {
                break;
            }
            m.data = m_expand;
        }

        m
    }

    fn pop(&mut self, mask: &FieldBit) {
        let mut v: [u16; 8] = [0; 8];
        let mut v_mask: [u16; 8] = [0; 8];
        unsafe {
            _mm_store_si128(v.as_mut_ptr() as *mut __m128i, self.data);
            _mm_store_si128(v_mask.as_mut_ptr() as *mut __m128i, mask.data);
        }

        for i in 0..6 {
            v[i] = util::pext16(v[i], !v_mask[i]);
        }

        self.data = unsafe { _mm_load_si128(v.as_ptr() as *const __m128i) };
    }

    fn print(&self) {
        for i in (0..6).rev() {
            println!("{:013b}", self.get_col(i));
        }
        println!();
    }
}