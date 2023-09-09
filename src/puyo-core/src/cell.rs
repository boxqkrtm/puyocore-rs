#[derive(Debug, PartialEq)]
pub enum CellType {
    NONE,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    GARBAGE,
}
#[derive(Debug, PartialEq)]
pub struct Cell{
    element: CellType
}
impl Cell {
    pub fn to_char(&self) -> char {
        match self.element {
            CellType::RED => 'R',
            CellType::YELLOW => 'Y',
            CellType::GREEN => 'G',
            CellType::BLUE => 'B',
            CellType::GARBAGE => '#',
            CellType::NONE => '.',
        }
    }
}
pub fn from_char(c: char) -> Cell {
    let cell_type = match c {
        'R' => CellType::RED,
        'Y' => CellType::YELLOW,
        'G' => CellType::GREEN,
        'B' => CellType::BLUE,
        '#' => CellType::GARBAGE,
        _ => CellType::NONE,
    };
    let cell = Cell {
        element: cell_type
    };
    return cell;
}
pub fn from_celltype(cell_type: CellType) -> Cell {
    let cell = Cell {
        element: cell_type
    };
    return cell;
}
pub const COUNT:i8 = 5; // without NONE