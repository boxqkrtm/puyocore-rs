use std::arch::x86_64::*;

pub fn pext16(input: u16, mask: u16) -> u16 {
    if cfg!(target_feature = "bmi2") {
        unsafe {
            return _pext_u32(input as u32, mask as u32) as u16;
        }
    } else {
        return pext16_emu(input, mask);
    }
}

//https://github.com/InstLatx64/InstLatX64_Demo/blob/master/PEXT_PDEP_Emu.cpp
pub fn pext16_emu(v: u16, m: u16) -> u16 {
    unsafe {
        let v: u32 = v as u32;
        let mut m: u32 = m as u32;
        let ret: u32;
        let pc = m.count_ones();
        match pc {
            0 => {
                ret = 0;
            }
            1 => {
                ret = ((v & m) as u32 != 0) as u32;
            }
            2 => {
                let msb = _bextr_u32(v, 31 - _lzcnt_u32(m), 1);
                let lsb = _bextr_u32(v, _tzcnt_u32(m), 1);
                ret = (msb << 1) | lsb;
            }
            3 => {
                let lz = 31 - _lzcnt_u32(m);
                let tz = _tzcnt_u32(m);
                let msb = _bextr_u32(v, lz, 1);
                let lsb = _bextr_u32(v, tz, 1);
                m = _blsr_u32(m);
                let csb = _bextr_u32(v, _tzcnt_u32(m), 1);
                ret = (msb << 2) | (csb << 1) | lsb;
            }
            4 => {
                let lz = 31 - _lzcnt_u32(m);
                let tz = _tzcnt_u32(m);
                let msb1 = _bextr_u32(v, lz, 1);
                let lsb1 = _bextr_u32(v, tz, 1);
                m &= !((1 << lz) | (1 << tz));
                let msb0 = _bextr_u32(v, 31 - _lzcnt_u32(m), 1);
                let lsb0 = _bextr_u32(v, _tzcnt_u32(m), 1);
                ret = (msb1 << 3) | (msb0 << 2) | (lsb0 << 1) | lsb1;
            }
            16 => {
                ret = v;
            }
            _ => {
                let mut mm = _mm_cvtsi32_si128(!m as i32);
                let mtwo = _mm_set1_epi64x((!0u64 - 1) as i64);
                let mut clmul = _mm_clmulepi64_si128(mm, mtwo, 0);
                let bit0 = _mm_cvtsi128_si32(clmul) as u32;
                let mut a = v & m;
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
        return ret as u16;
    }
}
// pub fn pext16_naive(input: u16, mut mask: u16) -> u16 {
//     let mut result: u16 = 0;
//     let mut bb: u16 = 1;

//     while mask != 0 {
//         if input & mask & ((!mask).wrapping_add(1)) != 0 {
//             result |= bb;
//         }
//         mask &= mask.wrapping_sub(1);
//         bb = bb.wrapping_add(bb);
//     }
//     return result;
// }
