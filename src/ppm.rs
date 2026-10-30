use crate::volume::Volume;

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    todo!();
}
