#[derive(Debug)]
pub enum Alter {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp
}

impl Alter {
    pub fn value(&self) -> i32 {
        match self {
            Alter::DoubleFlat => -2,
            Alter::Flat => -1,
            Alter::Natural => 0,
            Alter::Sharp => 1,
            Alter::DoubleSharp => 2,
        }
    }
}
