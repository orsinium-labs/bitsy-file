use crate::*;

pub trait AnimationFrames {
    fn to_string(&self) -> String;
}

impl AnimationFrames for Vec<Image> {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let last_frame = self.len() - 1;

        for (i, frame) in self.iter().enumerate() {
            string.push_str(&frame.to_string());

            if i < last_frame {
                string.push_str("\n>\n");
            }
        }

        string
    }
}
