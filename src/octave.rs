#[derive(Debug)]
pub struct Octave(pub u32);
impl Octave {
    pub fn value(&self) -> u32 {
        self.0
    }
}
