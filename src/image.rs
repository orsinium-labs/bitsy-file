use alloc::{format, string::String, vec::Vec};
use core::fmt;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    pub pixels: Vec<u8>, // 64 for SD, 256 for HD
}

impl Image {
    pub fn invert(&mut self) {
        self.pixels = self.pixels.iter().map(|pixel| 1 - pixel).collect();
    }

    /// flip image vertically
    pub fn flip(&mut self) {
        let mut pixels = Vec::with_capacity(64);

        for row in self.pixels.chunks(8).rev() {
            for pixel in row {
                pixels.push(*pixel);
            }
        }

        self.pixels = pixels;
    }

    /// mirror image horizontally
    pub fn mirror(&mut self) {
        let mut pixels = Vec::with_capacity(64);

        for row in self.pixels.chunks(8) {
            for i in (0..8).rev() {
                pixels.push(row[i]);
            }
        }

        self.pixels = pixels;
    }

    /// rotate image 90° clockwise
    pub fn rotate(&mut self) {
        let mut pixels = Vec::with_capacity(64);

        // start from bottom-left corner, work upward in that column,
        // then work on the next column to the right
        // for counter-clockwise, just reverse x instead of y
        for x in 0..8 {
            for y in (0..8).rev() {
                pixels.push(self.pixels[(y * 8) + x]);
            }
        }

        self.pixels = pixels;
    }

    pub fn from_lines<'a, I>(lines: I) -> Result<Image, crate::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut dimension = 0;
        let mut pixels: Vec<u8> = Vec::new();
        for (i, line) in lines.enumerate() {
            if dimension == 0 {
                dimension = if line.len() >= 16 { 16 } else { 8 };
            }
            let line = &line[..dimension];
            for char in line.chars() {
                pixels.push(u8::from(char == '1'));
            }
            if i + 1 == dimension {
                break;
            }
        }

        if pixels.len() != 64 && pixels.len() != 256 {
            return Err(crate::Error::Image);
        }
        Ok(Image { pixels })
    }
}

impl FromStr for Image {
    type Err = crate::Error;
    fn from_str(str: &str) -> Result<Image, Self::Err> {
        Self::from_lines(str.trim().lines())
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();

        let is_hd = self.pixels.len() >= 16 * 16;
        let sqrt = if is_hd { 16 } else { 8 };
        for line in self.pixels.chunks(sqrt) {
            for pixel in line {
                string.push_str(&format!("{}", *pixel));
            }
            string.push('\n');
        }

        string.pop(); // remove trailing newline

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn image_from_string() {
        let output = Image::from_str(include_str!("test-resources/image")).unwrap();

        let expected = Image {
            pixels: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
            ],
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn image_to_string() {
        let output = mock::image::chequers_1().to_string();
        let expected = include_str!("test-resources/image-chequers-1").to_string();
        assert_eq!(output, expected);
    }

    /// lots of Bitsy games have editor errors where pixels can be placed out of bounds
    /// check that these extraneous pixels are stripped out
    #[test]
    fn image_out_of_bounds() {
        let output = Image::from_str(include_str!("test-resources/image-oob")).unwrap();

        let expected = Image {
            pixels: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn invert() {
        let mut output = crate::mock::image::chequers_1();
        output.invert();
        let expected = crate::mock::image::chequers_2();
        assert_eq!(output, expected);
    }

    #[test]
    fn flip() {
        let mut image = crate::mock::image::asymmetrical();
        image.flip();

        let flipped = Image {
            pixels: vec![
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };

        assert_eq!(image, flipped);
    }

    #[test]
    fn mirror() {
        let mut image = crate::mock::image::asymmetrical();
        image.mirror();

        let mirrored = Image {
            pixels: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 1, 0, 0, 0,
            ],
        };

        assert_eq!(image, mirrored);
    }

    #[test]
    fn rotate() {
        let mut image = crate::mock::image::asymmetrical();
        image.rotate();

        let rotated = Image {
            pixels: vec![
                0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };

        assert_eq!(image, rotated);
    }
}
