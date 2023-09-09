pub mod cell {
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Type {
        RED,
        YELLOW,
        GREEN,
        BLUE,
        GARBAGE,
        NONE,
    }

    pub const COLOR_COUNT:i8 = 5;

    pub fn to_char(cell: Type) -> char {
        match cell {
            Type::RED => 'R',
            Type::YELLOW => 'Y',
            Type::GREEN => 'G',
            Type::BLUE => 'B',
            Type::GARBAGE => '#',
            Type::NONE => '.',
        }
    }


    pub fn from_char(c: char) -> Type {
        match c {
            'R' => Type::RED,
            'Y' => Type::YELLOW,
            'G' => Type::GREEN,
            'B' => Type::BLUE,
            '#' => Type::GARBAGE,
            _ => Type::NONE,
        }
    }
}
#[test]
fn test() {
    //from char test
    assert_eq!(cell::Type::NONE, cell::from_char('.'));
    assert_eq!(cell::Type::RED, cell::from_char('R'));
    assert_eq!(cell::Type::GREEN, cell::from_char('G'));
    assert_eq!(cell::Type::BLUE, cell::from_char('B'));
    assert_eq!(cell::Type::YELLOW, cell::from_char('Y'));
    assert_eq!(cell::Type::GARBAGE, cell::from_char('#'));

    //to char test
    assert_eq!(cell::to_char(cell::Type::NONE), '.');
    assert_eq!(cell::to_char(cell::Type::RED), 'R');
    assert_eq!(cell::to_char(cell::Type::GREEN), 'G');
    assert_eq!(cell::to_char(cell::Type::BLUE), 'B');
    assert_eq!(cell::to_char(cell::Type::YELLOW), 'Y');
    assert_eq!(cell::to_char(cell::Type::GARBAGE), '#');
}
