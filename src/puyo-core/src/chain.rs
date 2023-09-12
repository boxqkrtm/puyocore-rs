use crate::{field::Field, cell};

pub struct Score {
    pub count: i32,
    pub score: i32,
}
const COLOR_BONUS: [u32; 6] = [0, 0, 3, 6, 12, 24];
const GROUP_BONUS: [u32; 12] = [0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 10];
const POWER: [u32; 19] = [
    0, 8, 16, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512,
];
pub fn get_score(mut mask: Vec<Field>) -> Score {
    let mut result = Score { count: mask.len() as i32, score: 0 };

    for index in 0..mask.len() {
        let pop_count = mask[index].get_count();

        let chain_power = POWER[index];

        let mut color = 0;
        for cell in 0..cell::COUNT - 1 {
            color += if mask[index].data[cell].get_count() > 0 {
                1   
            }else{
                0
            };
        }
        let bonus_color = COLOR_BONUS[color as usize];

        let mut group_bonus = 0;
        for i in 0..cell::COUNT {
            while mask[index].data[i].get_count() > 0 {
                let group = mask[index].data[i].get_mask_group_lsb();
                mask[index].data[i] = mask[index].data[i] & (!group);
                group_bonus += GROUP_BONUS[std::cmp::min(11, group.get_count() as usize)];
            }
        }
        result.score += pop_count as i32 * 10 * std::cmp::max(1, std::cmp::min(999, chain_power + bonus_color + group_bonus)) as i32;
    }
    result
}
