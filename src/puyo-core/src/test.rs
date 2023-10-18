#[cfg(test)]
pub mod test {
    use std::arch::x86_64::_pext_u32;

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
}
