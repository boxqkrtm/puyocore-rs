use std::arch::x86_64::*;

pub fn pext16(input: u16, mask: u16) -> u16 {
    return unsafe {_pext_u32(input as u32, mask as u32) } as u16;
}
/*
pub fn pext16(input: u16, mut mask: u16) -> u16 {
    let mut result: u16 = 0;
    let mut bb: u16 = 0;
    while mask != 0 {
        if input & mask & ((!mask).wrapping_add(1)) != 0 {
            result |= bb.0;
        }
        mask &= mask - 1;
        bb = bb.wrapping_add(bb);
    }
    return result;
}
*/