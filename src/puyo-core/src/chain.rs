use crate::{field::Field, cell};

pub struct Chain {
    pub count: u32,
    pub score: u32,
}
const COLOR_BONUS: [u32; 6] = [0, 0, 3, 6, 12, 24];
const GROUP_BONUS: [u32; 12] = [0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 10];
const MAX_GROUP_BONUS: usize = 11;
const MAX_CHAIN_POWER: u32 = 999;
const POWER: [u32; 19] = [
    0, 8, 16, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512,
];
pub fn get_score(mask: &mut Vec<Field>) -> Chain {
    let mut result = Chain { count: mask.len() as u32, score: 0 };

    for index in 0..mask.len() {
        let pop_count = mask[index].get_count();

        let chain_power = POWER[index];

        let mut color: i32 = 0;
        for cell in &mask[index].data[..cell::COUNT - 1] {
            color = color.wrapping_add(if cell.get_count() > 0 { 1 } else { 0 });
        }
        
        let bonus_color = COLOR_BONUS[color as usize];

        let mut group_bonus:u32 = 0;
        for i in 0..cell::COUNT {
            while mask[index].data[i].get_count() > 0 {
                let group = mask[index].data[i].get_mask_group_lsb();
                mask[index].data[i] = mask[index].data[i] & (!group);
                group_bonus = group_bonus.wrapping_add(GROUP_BONUS[std::cmp::min(11, group.get_count() as usize)]);
            }
        }
        let bonus_score = std::cmp::max(1, std::cmp::min(MAX_CHAIN_POWER, chain_power.wrapping_add(bonus_color).wrapping_add(group_bonus))) as u32;
        result.score = result.score.wrapping_add(pop_count.wrapping_mul(10).wrapping_mul(bonus_score));
    }
    result
}
