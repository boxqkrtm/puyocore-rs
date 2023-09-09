pub mod util {
    use bitintr::Pext;

    pub fn pext16(input: u16, mask: u16) -> u16 {
        //simd
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            return input.pext(mask);
        }
        //naive
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        {
            let result: u16 = 0;
            let mut bb: u16 = 1;
            while mask != 0 {
                if input & mask & (!mask + 1) != 0 {
                    result |= bb;
                }
                mask &= mask - 1;
                bb += bb;
            }
            return result;
        }
    }
}