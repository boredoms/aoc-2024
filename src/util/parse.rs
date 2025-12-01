use std::str::Bytes;

pub struct ParseSigned<'a> {
    bytes: Bytes<'a>,
    sign_byte: u8,
}

fn byte_to_digit(b: u8) -> u8 {
    b.wrapping_sub(b'0')
}

impl ParseSigned<'_> {
    pub fn new(bytes: Bytes<'_>) -> ParseSigned<'_> {
        ParseSigned {
            bytes,
            sign_byte: b'-',
        }
    }

    pub fn new_with_sign(bytes: Bytes<'_>, sign_byte: u8) -> ParseSigned<'_> {
        ParseSigned { bytes, sign_byte }
    }
}

impl Iterator for ParseSigned<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = 0;
        let mut sign = 1;

        loop {
            let b = self.bytes.next()?;
            if b.is_ascii_digit() {
                n = byte_to_digit(b) as i32;
                break;
            } else if b == self.sign_byte {
                sign = -1;
                break;
            }
        }

        loop {
            let b = self.bytes.next()?;
            if b.is_ascii_digit() {
                n = n * 10 + byte_to_digit(b) as i32
            } else {
                break;
            }
        }

        Some(sign * n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_signed() {
        let a = "-13abca23ac-3b2n\n25nnn".bytes();
        let parsed: Vec<i32> = ParseSigned {
            bytes: a,
            sign_byte: b'-',
        }
        .collect();

        dbg!(&parsed);
        assert_eq!(parsed, vec![-13, 23, -3, 2, 25])
    }
}
