#[cfg(test)]
pub mod test {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_and_si128;
    use std::arch::x86_64::_mm_set_epi16;
    use std::arch::x86_64::_mm_slli_epi16;
    use std::arch::x86_64::_mm_slli_si128;
    use std::arch::x86_64::_mm_srli_si128;
    use std::arch::x86_64::_pext_u32;
    use std::simd::u16x8;

    use crate::cell;
    use crate::chain;
    use crate::field::Field;
    use crate::tsumo;
    use crate::util;

    #[test]
    fn cell_test() {
        //from char test
        let cell = cell::from_char('R');
        assert_eq!(cell.to_char(), 'R');
    }

    #[test]
    fn pext() {
        unsafe {
            let n0 = 0b1011_1110_1001_0011u16;
            let m0 = 0b0110_0011_1000_0101u16;
            assert_eq!(
                format!("{:016b}", util::pext15_emu(n0, m0)),
                format!("{:016b}", _pext_u32(n0 as u32, m0 as u32) as u16)
            );
        }
        unsafe {
            let n0 = 0b0000_1110_1001_0011u16;
            let m0 = 0b1111_0111_1111_1111u16;
            assert_eq!(
                format!("{:016b}", util::pext15_emu(n0, m0)),
                format!("{:016b}", _pext_u32(n0 as u32, m0 as u32) as u16)
            );
        }
    }

    #[test]
    fn tsumo() {
        let t = tsumo::Tsumo {
            first: cell::from_char('R'),
            second: cell::from_char('Y'),
        };
        assert_eq!(t.to_string(), "RY");
    }

    #[test]
    fn rensa() {
        let mut f = Field::new(); // Field 구조체 초기화 코드 추가
        let c: [[char; 7]; 13] = [
            ['B', '.', 'Y', 'R', 'G', 'Y', '\0'],
            ['B', 'B', 'B', 'Y', 'R', 'B', '\0'],
            ['G', 'B', 'Y', 'R', 'G', 'G', '\0'],
            ['B', 'G', 'Y', 'R', 'G', 'B', '\0'],
            ['G', 'R', 'G', 'Y', 'R', 'B', '\0'],
            ['R', 'G', 'Y', 'R', 'Y', 'B', '\0'],
            ['G', 'R', 'G', 'Y', 'R', 'Y', '\0'],
            ['G', 'R', 'G', 'Y', 'R', 'Y', '\0'],
            ['G', 'B', 'B', 'G', 'Y', 'G', '\0'],
            ['B', 'Y', 'R', 'B', 'G', 'G', '\0'],
            ['G', 'B', 'Y', 'R', 'B', 'Y', '\0'],
            ['G', 'B', 'Y', 'R', 'B', 'Y', '\0'],
            ['G', 'B', 'Y', 'R', 'B', 'Y', '\0'],
        ];
        f.from(&c);
        let mut mask = f.pop();
        let chain = chain::get_score(&mut mask);
        assert_eq!(chain.count, 19);
        assert_eq!(chain.score, 177640);
    }

    #[test]
    fn shi() {
        let data = u16x8::from_array([3300,9000,2,3,4,5,6,7]);
        unsafe {
            let data128 = std::mem::transmute::<u16x8, __m128i> (data);
            let r128 = _mm_slli_epi16(data128, 1);
            let r = std::mem::transmute::<__m128i, u16x8> (r128);
            let r2 = util::mm_slli_epi16_1(data);
            assert_eq!(r, r2);
        }
        unsafe {
            let data128 = std::mem::transmute::<u16x8, __m128i> (data);
            let r128 = _mm_srli_si128(data128, 1);
            let r = std::mem::transmute::<__m128i, u16x8> (r128);
            let r2 = util::mm_srli_si128_1(data);
            assert_eq!(r, r2);
        }
        unsafe {
            let data128 = std::mem::transmute::<u16x8, __m128i> (data);
            let r128 = _mm_slli_si128(data128, 2);
            let r = std::mem::transmute::<__m128i, u16x8> (r128);
            let r2 = util::mm_slli_si128_2(data);
            assert_eq!(r, r2);
        }
        unsafe {
            let data128 = std::mem::transmute::<u16x8, __m128i> (data);
            let r128 = _mm_srli_si128(data128, 1);
            let r = std::mem::transmute::<__m128i, u16x8> (r128);
            let r2 = util::mm_srli_si128_1(data);
            assert_eq!(r, r2);
        }
        unsafe {
            let data128 = std::mem::transmute::<u16x8, __m128i> (data);
            let r128 = _mm_srli_si128(data128, 2);
            let r = std::mem::transmute::<__m128i, u16x8> (r128);
            let r2 = util::mm_srli_si128_2(data);
            assert_eq!(r, r2);
        }
    }

    #[test]
    fn set_simd(){
        unsafe {
        let r = std::mem::transmute::<__m128i, u16x8>(
            _mm_set_epi16(0, 0, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF, 0x1FFF)
        );
        let r2 = u16x8::from_array([0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x1FFF,0x0000,0x0000,]);
        assert_eq!(r, r2);
        }
    }
}
