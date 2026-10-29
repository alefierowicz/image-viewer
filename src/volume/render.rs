use crate::volume::Volume;

impl Volume {
    fn to_display(&self, value: u16) -> u8 {
        ((value as f64 / self.maxval as f64).clamp(0.0, 1.0) * 255.0).round() as u8
    }

    pub fn render(&self) -> Vec<u8> {
        self.data.iter().map(|v| self.to_display(*v)).collect()
    }

    pub fn pixel_value(&self, x: usize, y: usize) -> Option<[u16; 3]> {
        if x >= self.w || y >= self.h {
            return None;
        }
        let idx = y * self.w + x;

        let start = idx * 3;
        let pixel = self.data.get(start..start + 3)?;

        Some([pixel[0], pixel[1], pixel[2]])
    }
}
