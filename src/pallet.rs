use crate::Color;
use rand::random;

#[warn(dead_code)]
pub struct Pallet;

impl Pallet {
    pub fn reverse(&self, c: Color) -> Color {
        Color(255 - c.0, 255 - c.1, 255 - c.2)
    }

    pub fn random(&self) -> Color {
        Color(random::<u8>(), random::<u8>(), random::<u8>())
    }

    pub fn random_magic(&self, c: Color) -> Color {
        let c_rand: Color = Color(random(), random(), random());

        let one: u8 = if c.0 > c_rand.0 {
            c.0 - c_rand.0
        } else {
            c_rand.0 - c.0
        };

        let two: u8 = if c.1 > c_rand.1 {
            c.1 - c_rand.1
        } else {
            c_rand.1 - c.1
        };

        let three: u8 = if c.2 > c_rand.2 {
            c.2 - c_rand.2
        } else {
            c_rand.2 - c.2
        };

        Color(one, two, three)
    }

    pub fn join(&self, color_one: Color, color_two: Color) -> Color {
        let one: u8 = if color_one.0 as u16 + color_two.0 as u16 >= 255 {
            255
        } else {
            color_one.0 + color_two.0
        };

        let two: u8 = if color_one.1 as u16 + color_two.1 as u16 >= 255 {
            255
        } else {
            color_one.1 + color_two.1
        };

        let three: u8 = if color_one.2 as u16 + color_two.2 as u16 >= 255 {
            255
        } else {
            color_one.2 + color_two.2
        };

        Color(one, two, three)
    }
}
