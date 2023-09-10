use crate::fieldbit;
struct Field {
    datas: [fieldbit::FieldBit; 6],
}

impl Field {
    pub fn new(&self) -> Field {
        let f = Field {
            datas: [fieldbit::FieldBit::new(); 6],
        };
        f
    }
}