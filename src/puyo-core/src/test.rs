use crate::cell;

#[test]
fn cell_test() {
    //from char test
    let cell1 = cell::Cell {
        element: cell::CellType::RED,
    };
    let cell2 = cell::from_char('R');
    assert_eq!(cell1.to_char(), cell2.to_char());
    assert_eq!(cell1, cell2);
}

#[test]
fn pext16(){
    let n = 0b1011_1110_1001_0011u16;
    let m0 = 0b0110_0011_1000_0101u16;
    let s0 = 0b0000_0000_0011_0101u16;
    assert_eq!(util::pext16(n, m0), s0);
}

#[test]
fn tsumo(){
    let t = tsumo::Tsumo {
        first: cell::from_char('R'),
        second: cell::from_char('Y')
    };
    assert_eq!(t.to_string(), "RY");
}