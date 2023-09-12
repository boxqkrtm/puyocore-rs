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
    pub element: CellType
}
impl Cell {
    pub fn to_char(&self) -> char {
        match self.element {
            CellType::RED => 'R',
            CellType::GREEN => 'G',
            CellType::BLUE => 'B',
            CellType::YELLOW => 'Y',
            CellType::GARBAGE => '#',
            CellType::NONE => '.',
        }
    }
    pub fn to_usize(&self) -> usize {
        match self.element{
            CellType::RED => 0,
            CellType::GREEN => 1,
            CellType::BLUE => 2,
            CellType::YELLOW => 3,
            CellType::GARBAGE => 4,
            CellType::NONE => 5
        }
    }
}
pub fn from_char(c: char) -> Cell {
    let cell_type = match c {
        'R' => CellType::RED,
        'G' => CellType::GREEN,
        'B' => CellType::BLUE,
        'Y' => CellType::YELLOW,
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
pub const COUNT:usize = 5;

pub fn from_usize(i: usize) -> Cell {
    return match i {
        0 => from_celltype(CellType::RED),
        1 => from_celltype(CellType::GREEN),
        2 => from_celltype(CellType::BLUE),
        3 => from_celltype(CellType::YELLOW),
        4 => from_celltype(CellType::GARBAGE),
        5 => from_celltype(CellType::NONE),
        _ => from_celltype(CellType::NONE)
    }
} // without NONE