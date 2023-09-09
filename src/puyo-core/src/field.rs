use crate::fieldbit;
struct Field {
    datas: [fieldbit::FieldBit; 6],
}

impl Field {
    pub fn new(&self) -> Field {
        let mut arr: [fieldbit::FieldBit; 6];
        for i in 0..6 {
            arr[i] = fieldbit::FieldBit::new();
        }
        let f = Field {
            datas: arr,
        };
        f
    }
}