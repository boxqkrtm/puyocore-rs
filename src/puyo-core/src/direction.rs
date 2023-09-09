#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub const COUNT: u32 = 4;

pub fn get_offset_x(direction: Type) -> i8 {
    match direction {
        Type::UP => 0,
        Type::RIGHT => 1,
        Type::DOWN => 0,
        Type::LEFT => -1,
    }
}

pub fn get_offset_y(direction: Type) -> i8 {
    match direction {
        Type::UP => 1,
        Type::RIGHT => 0,
        Type::DOWN => -1,
        Type::LEFT => 0,
    }
}