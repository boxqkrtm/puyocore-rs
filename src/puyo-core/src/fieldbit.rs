use std::simd::{u16x8, Simd};
use crate::util;
#[derive(Clone, Copy)] 
pub struct FieldBit {
    pub data: u16x8,
}
impl FieldBit {
    pub fn new() -> FieldBit {
        FieldBit {
            data: u16x8::splat(0),
        }
    }

    pub fn set_bit(&mut self, x: i8, y: i8) {
        assert!(x >= 0 && x < 6);
        self.data[x as usize] |= 1u16.wrapping_shl(y as u32);
    }

    pub fn get_bit(&self, x: i8, y: i8) -> bool {
        if x < 0 || x > 5 || y < 0 || y > 12 {
            return true;
        }
        let v = self.data;
        v[x as usize] & (1u16.wrapping_shl(y as u32)) != 0
    }

    pub fn get_count(&self) -> u32 {
        let v = self.data;
        v[0].count_ones().wrapping_add(v[1].count_ones())
    }

    pub fn get_col(&self, x: i8) -> u16 {
        assert!(x >= 0 && x < 6);
        self.data[x as usize]
    }

    pub fn get_expand(& mut self) -> FieldBit {
        let r = util::mm_srli_si128_2(self.data);
        let l = util::mm_slli_si128_2(self.data);
        let u = util::mm_srli_epi16_1(self.data);
        let d = util::mm_slli_epi16_1(self.data);

        self.data = self.data | (r | l | u | d);
        return self.clone();
    }

    fn get_mask_12(&self) -> FieldBit {
        let mut result = FieldBit::new();
        let mask: u16x8 = u16x8::from_array([0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF,0,0]);
        result.data = result.data & mask;
        //_mm_set_epi16(0, 0, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF, 0x0FFF),
        result
    }

    fn get_mask_13(&self) -> FieldBit {
        let mut result = FieldBit::new();
        let mask: u16x8 = u16x8::from_array([0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x0000,0x0000]);
        result.data = result.data & mask;
        //_mm_set_epi16(0, 0, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF),
        result
    }

    pub unsafe fn get_mask_pop(&self) -> FieldBit {
        let m12 = self.get_mask_12();
        let r = util::mm_srli_si128_2(self.data) & m12.data;
        let l = util::mm_slli_si128_2(self.data) & m12.data;
        let u = util::mm_srli_epi16_1(self.data) & m12.data;
        let d = util::mm_slli_epi16_1(self.data) & m12.data;

        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;

        let m3 = (ud_and & lr_or) | (lr_and & ud_or);
        let m2 = ud_and | lr_and | (ud_or & lr_or);
        let m2_r = util::mm_srli_si128_2(m2) & m2;
        let m2_l = util::mm_slli_si128_2(m2) & m2;
        let m2_u = util::mm_srli_epi16_1(m2) & m2;
        let m2_d = util::mm_slli_epi16_1(m2) & m2;

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
            let a =  unsafe {std::mem::transmute::<Simd<u16, 8>, Simd<u64,2>>(m.data)};
            let mask:Simd<u64,2> = unsafe {std::mem::transmute::<Simd<u16, 8>, Simd<u64,2>>(m_expand)};
            if (a[0] & mask[0]) == mask[0] && (a[1] & mask[1]) == mask[1] {
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
            let a =  unsafe {std::mem::transmute::<Simd<u16, 8>, Simd<u64,2>>(m.data)};
            let mask:Simd<u64,2> = unsafe {std::mem::transmute::<Simd<u16, 8>, Simd<u64,2>>(m_expand)};
            if (a[0] & mask[0]) == mask[0] && (a[1] & mask[1]) == mask[1] {
                break;
            }
            m.data = m_expand;
        }
        m
    }

    pub fn get_mask_group_lsb(&self) -> FieldBit {
        let m12 = self.get_mask_12();

        let mut v = m12.data;

        if v[0] == 0 {
            v[1] &= (!v[1]).wrapping_add(1);
        } else {
            v[0] &= (!v[0]).wrapping_add(1);
            v[1] = 0;
        }

        let mut m = FieldBit::new();
        m.data = v.clone();
        
        while !self.data == m.data {
            let m_expand = m.get_expand().data & m12.data;
            let a =  unsafe {std::mem::transmute::<Simd<u16, 8>, Simd<u64,2>>(m.data)};
            let mask:Simd<u64,2> = unsafe {std::mem::transmute::<Simd<u16, 8>, Simd<u64,2>>(m_expand)};
            if (a[0] & mask[0]) == mask[0] && (a[1] & mask[1]) == mask[1] {
                break;
            }
            m.data = m_expand;
        }

        m
    }

    pub fn pop(&mut self, mask: &FieldBit) {
        let mut v= self.data;
        let v_mask = mask.data;

        for i in 0..6 {
            v[i] = util::pext16(v[i], !v_mask[i]);
        }

        self.data = v;
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
