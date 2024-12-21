use crate::cell;
pub struct Tsumo {
    pub first: cell::Cell,
    pub second: cell::Cell,
}
impl ToString for Tsumo {
    fn to_string(&self) -> String {
        return format!("{}{}", self.first.to_char(), self.second.to_char());
    }
}