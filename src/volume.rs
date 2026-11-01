mod getters;
mod render;

pub struct Volume {
    h: usize,
    w: usize,
    maxval: u16,
    data: Vec<u16>,
}

impl Volume {
    pub fn new(w: usize, h: usize, maxval: u16, data: Vec<u16>) -> Result<Volume, String> {
        if w * h == 0 {
            return Err(format!("invalid image size: {} x {}", w, h));
        }
        if maxval == 0 {
            return Err("the maximum value of the image is invalid ".into());
        }
        if data.len() != w * h * 3 {
            return Err(format!("invalid image size - expected {} samples, got {}.", w * h * 3, data.len()));
        }

        Ok(Volume { h, w, maxval, data })
    }
}
