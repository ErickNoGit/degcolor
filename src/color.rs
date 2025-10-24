use std::fmt;

#[derive(Clone, Debug)]
#[warn(dead_code)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:X}{:X}{:X}", self.0, self.1, self.2)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({}, {}, {})", self.0, self.1, self.2)
    }
}
