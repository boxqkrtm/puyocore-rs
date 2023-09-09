use crate::cell;

pub mod tsumo {
    use std::string;
    use crate::cell;

    pub struct Tsumo {
        pub first: cell::Cell,
        pub second: cell::Cell,
    }
    impl Tsumo {
        pub fn print(&self){
            println!("{}", self.to_string());
        }
        pub fn to_string(&self) -> String {
            return format!("{}{}", self.first.to_char(), self.second.to_char());
        }
    } 
}