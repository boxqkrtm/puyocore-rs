
use std::arch::x86_64::*;
//use std::arch::arm::*;

pub fn pext16(input: u16, mask: u16) -> u16 {
    if cfg!(target_feature = "bmi2") {
        unsafe {
            return _pext_u32(input as u32, mask as u32) as u16;
        }
    } else {
        return pext15_emu(input, mask);
    }
}

pub fn bextr_u32(a: u32, start: u32, len: u32) -> u32 {
    if cfg!(target="x86_64") {
        unsafe{
            return _bextr_u32(a, start, len);
        }
    }
    // else if cfg!(target="aarch64") {
    //     unsafe{
    //         return 
    //     }
    // }
}
//https://github.com/InstLatx64/InstLatX64_Demo/blob/master/PEXT_PDEP_Emu.cpp - edit for puyo (not all mask support for performance)
pub fn pext15_emu(v: u16, m: u16) -> u16 {
    unsafe {
        let v_u32: u32 = v as u32;
        let m_u32: u32 = m as u32;
        let ret: u32;
        let pc = m_u32.count_ones();
        //TODO 이어진 2,3,4,5,6pop의 경우 빠르게 처리
        if false && pc >= 10 && pc <= 14 {
            let popcnt = 16 - pc;
            //0이 이어져 있는지 확인

            //시프트 연산으로 한번에 이동
        }
        match pc {
            0 => {
                ret = 0;
            }
            1 => {
                ret = ((v_u32 & m_u32) as u32 != 0) as u32;
            }
            2 => {
                let msb = _bextr_u32(v_u32, 31 - _lzcnt_u32(m_u32), 1);
                let lsb = _bextr_u32(v_u32, _tzcnt_u32(m_u32), 1);
                ret = (msb.wrapping_shl(1)) | lsb;
            }
            3 => {
                let lz = 31 - _lzcnt_u32(m_u32);
                let tz = _tzcnt_u32(m_u32);
                let msb = _bextr_u32(v_u32, lz, 1);
                let lsb = _bextr_u32(v_u32, tz, 1);
                let m_u32_2 = _blsr_u32(m_u32);
                let csb = _bextr_u32(v_u32, _tzcnt_u32(m_u32_2), 1);
                ret = (msb.wrapping_shl(2)) | (csb.wrapping_shl(1)) | lsb;
            }
            4 => {
                let lz = 31 - _lzcnt_u32(m_u32);
                let tz = _tzcnt_u32(m_u32);
                let msb1 = _bextr_u32(v_u32, lz, 1);
                let lsb1 = _bextr_u32(v_u32, tz, 1);
                let m_u32_2 = (m as u32 & (!((1u32.wrapping_shl(lz)) | (1u32.wrapping_shl(tz))))) as u32;
                let msb0 = _bextr_u32(v_u32, 31 - _lzcnt_u32(m_u32_2), 1);
                let lsb0 = _bextr_u32(v_u32, _tzcnt_u32(m_u32_2), 1);
                ret = (msb1.wrapping_shl(3)) | (msb0.wrapping_shl(2)) | (lsb0.wrapping_shl(1)) | lsb1;
            }
            15 => {
                //not working with 0b0111111111111111
                let zero_location: u16 = !m;
                let zero_location_index = zero_location.trailing_zeros() + 1;
                let left_mask = (!0u16).wrapping_shr(zero_location_index).wrapping_shl(zero_location_index);
                let right_mask = (!left_mask).wrapping_shr(1);
                let shifted_left_value = (v & left_mask).wrapping_shr(1);
                ret = ((v & right_mask) | shifted_left_value) as u32;
            }
            16 => {
                ret = v_u32;
            }
            _ => {
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
                ret = ((!bit3 & a) | ((bit3 & a).wrapping_shr(8))) as u32;
            }
        }
        return ret as u16;
    }
}

pub fn pext32_emu(v: u32, m: u32) -> u32 {
    unsafe {
        let v_u32: u32 = v;
        let m_u32: u32 = m;
        let ret: u32;
        let pc = m_u32.count_ones();
        match pc {
            0 => {
                ret = 0;
            }
            1 => {
                ret = ((v_u32 & m_u32) as u32 != 0) as u32;
            }
            2 => {
                let msb = _bextr_u32(v_u32, 31 - _lzcnt_u32(m_u32), 1);
                let lsb = _bextr_u32(v_u32, _tzcnt_u32(m_u32), 1);
                ret = (msb << 1) | lsb;
            }
            3 => {
                let lz = 31 - _lzcnt_u32(m_u32);
                let tz = _tzcnt_u32(m_u32);
                let msb = _bextr_u32(v_u32, lz, 1);
                let lsb = _bextr_u32(v_u32, tz, 1);
                let m_u32_2 = _blsr_u32(m_u32);
                let csb = _bextr_u32(v_u32, _tzcnt_u32(m_u32_2), 1);
                ret = (msb << 2) | (csb << 1) | lsb;
            }
            4 => {
                let lz = 31 - _lzcnt_u32(m_u32);
                let tz = _tzcnt_u32(m_u32);
                let msb1 = _bextr_u32(v_u32, lz, 1);
                let lsb1 = _bextr_u32(v_u32, tz, 1);
                let m_u32_2 = (m & (!((1 << lz) | (1 << tz)))) as u32;
                let msb0 = _bextr_u32(v_u32, 31 - _lzcnt_u32(m_u32_2), 1);
                let lsb0 = _bextr_u32(v_u32, _tzcnt_u32(m_u32_2), 1);
                ret = (msb1 << 3) | (msb0 << 2) | (lsb0 << 1) | lsb1;
            }
            _ => {
                let mut mm = _mm_cvtsi32_si128(!m_u32 as i32);
                let mtwo = _mm_set1_epi64x((!0u64 - 1) as i64);
                let mut clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
                let bit0 = _mm_cvtsi128_si32(clmul) as u32;
                let mut a = v_u32 & m_u32;
                a = (!bit0 & a) | ((bit0 & a) >> 1);
                mm = _mm_and_si128(mm, clmul);
                clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
                let bit1 = _mm_cvtsi128_si32(clmul) as u32;
                a = (!bit1 & a) | ((bit1 & a) >> 2);
                mm = _mm_and_si128(mm, clmul);
                clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
                let bit2 = _mm_cvtsi128_si32(clmul) as u32;
                a = (!bit2 & a) | ((bit2 & a) >> 4);
                mm = _mm_and_si128(mm, clmul);
                clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
                let bit3 = _mm_cvtsi128_si32(clmul) as u32;
                ret = ((!bit3 & a) | ((bit3 & a) >> 8)) as u32;
            }
        }
        return ret;
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
