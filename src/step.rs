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

impl Step {
    pub fn scale_index(&self) -> u32 {
        match self {
            Step::C => 0,
            Step::D => 1,
            Step::E => 2,
            Step::F => 3,
            Step::G => 4,
            Step::A => 5,
            Step::B => 6,
        }
    }

    pub fn chromatic_index(&self) -> u32 {
        match self {
            Step::C => 0,
            Step::D => 2,
            Step::E => 4,
            Step::F => 5,
            Step::G => 7,
            Step::A => 9,
            Step::B => 11,
        }
    }
}
