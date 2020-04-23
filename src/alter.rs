#[derive(Debug)]
pub enum Alter {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp
}

use Alter::*;

impl Alter {
    pub fn value(&self) -> i32 {
        match self {
            DoubleFlat => -2,
            Flat => -1,
            Natural => 0,
            Sharp => 1,
            DoubleSharp => 2,
        }
    }
}
