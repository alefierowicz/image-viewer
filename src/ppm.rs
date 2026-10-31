use crate::volume::Volume;

pub enum PPMFormat {
    P3,
    P6,
}

impl std::fmt::Display for PPMFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PPMFormat::P3 => write!(f, "P3"),
            PPMFormat::P6 => write!(f, "P6"),
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.bytes.get(self.pos) {
                Some(b) if b.is_ascii_whitespace() => {
                    self.pos += 1;
                }
                Some(&b'#') => loop {
                    match self.bytes.get(self.pos) {
                        None => return,
                        Some(b'\n') => {
                            self.pos += 1;
                            break;
                        }
                        Some(_) => {
                            self.pos += 1;
                        }
                    }
                },
                _ => return,
            }
        }
    }
    fn read_uint(&mut self) -> Result<u32, String> {
        self.skip_whitespace_and_comments();
        let start = self.pos;
        loop {
            match self.bytes.get(self.pos) {
                Some(byte) if byte.is_ascii_digit() => self.pos += 1,
                _ => break,
            }
        }

        if self.pos == start {
            return match self.bytes.get(self.pos) {
                None => Err("unexpected EOF in header".into()),
                Some(b) => Err(format!("expected a number at byte {}, found '{}'", self.pos, *b as char)),
            };
        }

        let digits = &self.bytes[start..self.pos];
        let Ok(value) = std::str::from_utf8(digits) else {
            return Err("failure converting digits to u32".into());
        };
        let output = value.parse::<u32>().map_err(|e| e.to_string());

        output
    }
}

pub fn parse(bytes: &[u8]) -> Result<Volume, String> {
    let format = match bytes.get(0..2) {
        None => return Err("not a valid PPM file - too short.".into()),
        Some(b"P3") => PPMFormat::P3,
        Some(b"P6") => PPMFormat::P6,
        _ => return Err("unsupported file format".into()),
    };

    let mut reader = Reader { bytes, pos: 2 };
    let width = reader.read_uint()?;
    let height = reader.read_uint()?;
    let maxval = reader.read_uint()?;

    match bytes.get(reader.pos) {
        Some(b) if b.is_ascii_whitespace() => reader.pos += 1,
        Some(b) => return Err(format!("expected whitespace after max value, got {}", *b as char)),
        None => return Err("file ended right after header.".into()),
    }

    if width == 0 || height == 0 {
        return Err(format!("invalid image dimension: {} x {}", width, height));
    }
    // checking for overflow on bad files
    let Some(samples) = (width as usize).checked_mul(height as usize).and_then(|n| n.checked_mul(3)) else {
        return Err("image too big!".into());
    };

    if maxval > u16::MAX as u32 {
        return Err(format!("maximum value {} out of range: 1-65535", maxval));
    }
    if maxval == 0 {
        return Err(format!("maximum value cannot be zero"));
    }
    let data = match format {
        PPMFormat::P6 => read_p6(&bytes[reader.pos..], samples, width as usize, maxval)?,
        PPMFormat::P3 => return Err("not yet implemented".into()),
    };
    Volume::new(width as usize, height as usize, maxval as u16, data)
}

fn read_p6(data: &[u8], samples: usize, width: usize, maxval: u32) -> Result<Vec<u16>, String> {
    let bytes_per_sample = if maxval < 256 { 1 } else { 2 };
    let Some(expected) = samples.checked_mul(bytes_per_sample) else { return Err("invalid file length.".into()) };

    if data.len() < expected {
        return Err(format!("truncated file: expected {} bytes, got {}", expected, data.len()));
    }
    if data.len() > expected {
        return Err(format!("{} unexpected bytes after the pixel data", data.len() - expected));
    }
    // conversion
    let output: Vec<u16> = match bytes_per_sample {
        1 => data.iter().map(|&b| b as u16).collect(),
        2 => data.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect(),
        _ => unreachable!("not expecting any other values as its assigned to either 1 or 2 above."),
    };
    if let Some(i) = output.iter().position(|&v| v > maxval as u16) {
        let pixel = i / 3;
        return Err(format!(
            "sample value {} exceeds maxval {} at pixel ({}, {})",
            output[i],
            maxval,
            pixel % width,
            pixel / width
        ));
    }

    Ok(output)
}
