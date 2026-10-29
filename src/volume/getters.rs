use crate::volume::Volume;

impl Volume {
    pub fn width(&self) -> usize {
        self.w
    }
    pub fn height(&self) -> usize {
        self.h
    }
    pub fn maxval(&self) -> u16 {
        self.maxval
    }
}
