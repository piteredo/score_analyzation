#[derive(Debug)]
pub enum Step {
    C,
    D,
    E,
    F,
    G,
    A,
    B
}

use Step::*;

impl Step {
    pub fn scale_index(&self) -> u32 {
        match self {
            C => 0,
            D => 1,
            E => 2,
            F => 3,
            G => 4,
            A => 5,
            B => 6,
        }
    }

    pub fn chromatic_index(&self) -> u32 {
        match self {
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
        }
    }
}
