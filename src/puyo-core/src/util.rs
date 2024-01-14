#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::simd::{u16x8, u8x16};

pub fn pext16(input: u16, mask: u16) -> u16 {
    //#[cfg(target_feature = "bmi2")]
    //return unsafe { _pext_u32(input as u32, mask as u32) as u16 };
    //#[cfg(not(target_feature = "bmi2"))]
    return pext15_emu(input, mask);
}

//https://github.com/InstLatx64/InstLatX64_Demo/blob/master/PEXT_PDEP_Emu.cpp - edit for puyo (not all mask support for performance)
pub fn pext15_emu(v: u16, m: u16) -> u16 {
    let v_u32: u32 = v as u32;
    let m_u32: u32 = m as u32;
    let pc = m_u32.count_ones();
    if pc == 16 {
        return v;
    }
    //implement other case like consecutive 2+ bit is make slower
    if pc == 15 {
        //if zero bit count is 1 in mask
        //shift once
        let zero_location: u16 = !m;
        let zero_location_index = zero_location.trailing_zeros() + 1;
        let left_mask = (!0u16)
            .wrapping_shr(zero_location_index)
            .wrapping_shl(zero_location_index);
        let right_mask = (!left_mask).wrapping_shr(1);
        let shifted_left_value = (v & left_mask).wrapping_shr(1);
        return (v & right_mask) | shifted_left_value;
    } else {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            //0b0111_1111_1111_1111 mask not working
            let mut mm = _mm_cvtsi32_si128(!m_u32 as i32);
            let mtwo = _mm_set1_epi64x((!0u64 - 1) as i64);
            let mut clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit0 = _mm_cvtsi128_si32(clmul) as u32;
            let mut a = v_u32 & m_u32;
            a = (!bit0 & a) | ((bit0 & a).wrapping_shr(1));
            mm = _mm_and_si128(mm, clmul);
            clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit1 = _mm_cvtsi128_si32(clmul) as u32;
            a = (!bit1 & a) | ((bit1 & a).wrapping_shr(2));
            mm = _mm_and_si128(mm, clmul);
            clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit2 = _mm_cvtsi128_si32(clmul) as u32;
            a = (!bit2 & a) | ((bit2 & a).wrapping_shr(4));
            mm = _mm_and_si128(mm, clmul);
            clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
            let bit3 = _mm_cvtsi128_si32(clmul) as u32;
            return ((!bit3 & a) | ((bit3 & a).wrapping_shr(8))) as u16;
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            return pext16_naive(v, m);
        }
    }
}

pub fn pext16_naive(input: u16, mut mask: u16) -> u16 {
    let mut result: u16 = 0;
    let mut bb: u16 = 1;
    while mask != 0 {
        if input & mask & ((!mask).wrapping_add(1)) != 0 {
            result |= bb;
        }
        mask &= mask.wrapping_sub(1);
        bb = bb.wrapping_add(bb);
    }
    return result;
}

pub fn mm_srli_si128_1(data_u16x8: u16x8) -> u16x8 {
    unsafe {
        let data = std::mem::transmute::<u16x8, u8x16>(data_u16x8);
        return std::mem::transmute::<u8x16, u16x8>(u8x16::from_array([
            data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
            data[10], data[11], data[12], data[13], data[14], data[15], 0,
        ]));
    }
}

pub fn mm_srli_si128_2(data_u16x8: u16x8) -> u16x8 {
    unsafe {
        let data = std::mem::transmute::<u16x8, u8x16>(data_u16x8);
        return std::mem::transmute::<u8x16, u16x8>(u8x16::from_array([
            data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10],
            data[11], data[12], data[13], data[14], data[15], 0, 0,
        ]));
    }
}
pub fn mm_srli_epi16_1(data_u16x8: u16x8) -> u16x8 {
    data_u16x8 >> u16x8::splat(1)
}

pub fn mm_slli_epi16_1(data_u16x8: u16x8) -> u16x8 {
    data_u16x8 << u16x8::splat(1)
}

pub fn mm_slli_si128_1(data_u16x8: u16x8) -> u16x8 {
    unsafe {
        let data = std::mem::transmute::<u16x8, u8x16>(data_u16x8);
        return std::mem::transmute::<u8x16, u16x8>(u8x16::from_array([
            0, data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
            data[9], data[10], data[11], data[12], data[13], data[14],
        ]));
    }
}

pub fn mm_slli_si128_2(data_u16x8: u16x8) -> u16x8 {
    unsafe {
        let data = std::mem::transmute::<u16x8, u8x16>(data_u16x8);
        return std::mem::transmute::<u8x16, u16x8>(u8x16::from_array([
            0, 0, data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
            data[9], data[10], data[11], data[12], data[13],
        ]));
    }
}
