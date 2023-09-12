use crate::fieldbit::FieldBit;
use crate::cell;
use crate::direction;
use crate::tsumo::Tsumo;
use std::arch::x86_64::*;

pub struct Field {
    pub data: [FieldBit; cell::COUNT],//r g b y garbage
}
impl Field {
    pub fn new() -> Field {
        let data = [FieldBit::new(); cell::COUNT];//RGBYG
        Field { data }
    }

    pub fn set_cell(&mut self, x: i8, y: i8, cell: cell::Cell) {
        self.data[cell.to_usize()].set_bit(x, y);
    }

    pub fn get_cell(&self, x: i8, y: i8) -> cell::Cell {
        if x < 0 || x > 5 || y < 0 || y > 12 {
            return cell::from_celltype(cell::CellType::NONE);
        }
        for i in 0..cell::COUNT {
            if self.data[i].get_bit(x, y) {
                return cell::from_usize(i);
            }
        }
        return cell::from_celltype(cell::CellType::NONE);
    }

    pub fn get_count(&self) -> u32 {
        let mut result = 0;
        for cell in 0..cell::COUNT {
            result += self.data[cell].get_count();
        }
        result
    }

    pub fn get_height(&self, x: i8) -> u8 {
        let mask = self.get_mask();
        let v = mask.data;
        let mut heights = [0u16; 8];
        unsafe {
            _mm_storeu_si128(&mut heights as *mut [u16; 8] as *mut __m128i, v.0);
        }
        16 - heights[x as usize].leading_zeros() as u8
    }

    pub fn get_height_max(&self) -> u8 {
        let mut heights = [0u8; 6];
        self.get_heights(&mut heights);
        *heights.iter().max().unwrap()
    }

        

    pub fn get_heights(&self, heights: &mut [u8; 6]) {
        let mask = self.get_mask();
        let v = mask.data;
        let mut heights_arr = [0u16; 8];
        unsafe {
            _mm_storeu_si128(&mut heights_arr as *mut [u16; 8] as *mut __m128i, v.0);
        }
        for i in 0..6 {
            heights[i] = 16 - heights_arr[i] as u8;
        }
    }

    pub fn get_mask(&self) -> FieldBit {
        let mut result = FieldBit::new();
        for cell in 0..cell::COUNT {
            result = result | self.data[cell];
        }
        result
    }

    unsafe fn get_mask_pop(&self) -> Field {
        let mut result = Field::new();
        for cell in 0..cell::COUNT - 1 {
            result.data[cell] = self.data[cell].get_mask_pop();
        }
        result
    }

    fn get_drop_pair_frame(&self, x: i8, direction: direction::Type) -> u8 {
        if self.get_height(x) != self.get_height(x + direction::get_offset_x(direction)) {
            return 2;
        }
        return 1;
    }

    fn is_occupied(&self, x: i8, y: i8) -> bool {
        if x < 0 || x > 5 || y < 0 {
            return true;
        }
        y < self.get_height(x) as i8
    }

    fn is_occupied_with_heights(&self, x: i8, y: i8, heights: &[u8; 6]) -> bool {
        if x < 0 || x > 5 || y < 0 {
            return true;
        }
        y < heights[x as usize] as i8
    }

    fn is_colliding_pair(&self, x: i8, y: i8, direction: direction::Type) -> bool {
        let mut heights = [0u8; 6];
        self.get_heights(&mut heights);
        self.is_colliding_pair_with_heights(x, y, direction, &heights)
    }

    fn is_colliding_pair_with_heights(
        &self,
        x: i8,
        y: i8,
        direction: direction::Type,
        heights: &[u8; 6],
    ) -> bool {
        self.is_occupied_with_heights(x, y, heights)
            || self.is_occupied_with_heights(
                x + direction::get_offset_x(direction),
                y + direction::get_offset_y(direction),
                heights,
            )
    }

    fn drop_puyo(&mut self, x: i8, cell: cell::Cell) {
        assert!(x >= 0 && x < 6);
        let height = self.get_height(x);
        if height < 13 {
            self.set_cell(x, height as i8, cell);
        }
    }

    fn drop_pair(&mut self, x: i8, direction: direction::Type, pair: Tsumo) {
        assert!(x >= 0 && x < 6);
        match direction {
            direction::Type::UP => {
                self.drop_puyo(x, pair.first);
                self.drop_puyo(x, pair.second);
            }
            direction::Type::RIGHT => {
                self.drop_puyo(x, pair.first);
                self.drop_puyo(x + 1, pair.second);
            }
            direction::Type::DOWN => {
                self.drop_puyo(x, pair.second);
                self.drop_puyo(x, pair.first);
            }
            direction::Type::LEFT => {
                self.drop_puyo(x, pair.first);
                self.drop_puyo(x - 1, pair.second);
            }
        }
    }

    pub fn pop(&mut self) -> Vec<Field> {
        let mut result = Vec::new();
        let current = self;

        for _ in 0..20 {
            let pop = unsafe { current.get_mask_pop() };
            let mut mask_pop = pop.get_mask();
            if unsafe { _mm_testz_si128(mask_pop.data.0, mask_pop.data.0) == 1 } {
                break;
            }
            result.push(pop);
            let mut mask_pop_expanded = mask_pop | (mask_pop.get_expand() & current.data[cell::from_celltype(cell::CellType::GARBAGE).to_usize()]);
            for cell in 0..cell::COUNT {
                current.data[cell].pop(&mut mask_pop_expanded);
            }
        }

        result
    }

    pub fn from(&mut self, c: &[[char; 7]; 13]) {
        *self = Field::new();
        for y in 0..13 {
            for x in 0..6 {
                if c[12 - y][x] == '.' || c[12 - y][x] == ' ' {
                    continue;
                }
                self.set_cell(x as i8, y as i8, cell::from_char(c[12 - y][x]));
            }
        }
    }

    pub fn print(&self) {
        for y in (0..13).rev() {
            for x in 0..6 {
                print!("{}", self.get_cell(x, y as i8).to_char());
            }
            println!();
        }
    }
}

impl Clone for Field {
    fn clone(&self) -> Field {
        Field {
            data: self.data.clone(),
        }
    }
}