use alloc::{format, string::String, vec::Vec};
use core::fmt;

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

    fn from_str(str: &str) -> Result<(Image, Vec<crate::Error>), crate::Error> {
        let mut warnings = Vec::new();

        if str.contains("NaN") {
            warnings.push(crate::Error::Image);
        }

        let string = str.trim().replace("NaN", "0");

        let lines: Vec<&str> = string.lines().collect();
        let dimension = lines.len();
        let mut pixels: Vec<u8> = Vec::new();

        for line in lines {
            let line = &line[..dimension];
            for char in line.chars() {
                // todo push warning on integers other than 0/1
                pixels.push(match char {
                    '1' => 1,
                    _ => 0,
                });
            }
        }

        if [64, 256].contains(&pixels.len()) {
            Ok((Image { pixels }, warnings))
        } else {
            Err(crate::Error::Image)
        }
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

/// todo return Result<(Vec<Image>, Vec<crate::Error>), crate::Error>
pub fn animation_frames_from_str(str: &str) -> Vec<Image> {
    str.split('>')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&frame| Image::from_str(frame).unwrap().0)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::image::{animation_frames_from_str, Image};
    use crate::mock;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn image_from_string() {
        let (output, _) = Image::from_str(include_str!("test-resources/image")).unwrap();

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

    #[test]
    fn test_animation_frames_from_string() {
        let output = animation_frames_from_str(include_str!("test-resources/animation_frames"));

        let expected = mock::image::animation_frames();

        assert_eq!(output, expected);
    }

    /// lots of Bitsy games have editor errors where pixels can be placed out of bounds
    /// check that these extraneous pixels are stripped out
    #[test]
    fn image_out_of_bounds() {
        let (output, _) = Image::from_str(include_str!("test-resources/image-oob")).unwrap();

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
