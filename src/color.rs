use rand::random;
use std::fmt;

/// The Color object structure represents RGB
/// colors and has accessible methods for converting to other formats.
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl Default for Color {
    /// Standard implementation of a maximum white color with RGB values ​​of 255.
    fn default() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}

impl Color {
    /// Instance to create a new color.
    ///
    /// # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let c: Color = Color::new(0, 0, 255); // Blue
    /// println!("{}", cor); // Implements Display
    ///
    /// // Or
    ///
    /// assert_eq!(cor.to_hex(), "#0000FF".to_string());
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Instance to create a new random color.
    ///
    /// # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let c: Color = Color::random();
    /// println!("{}", cor); // Implements Display
    ///
    /// // Or
    ///
    /// println!("{}", cor.to_hex()); // Returns String
    /// ```
    pub fn random() -> Self {
        Self {
            red: random::<u8>(),
            green: random::<u8>(),
            blue: random::<u8>(),
        }
    }

    ///
    pub fn from_str(input: &str) -> Option<Color> {
        let input: &str = input.trim();

        // Format RGB rgb(0, 255, 255)
        if input.starts_with("rgb(") {
            let inner: &str = input.trim_start_matches("rgb(").trim_end_matches(")");
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 3 {
                let r: u8 = parts[0].trim().parse::<u8>().ok()?;
                let g: u8 = parts[1].trim().parse::<u8>().ok()?;
                let b: u8 = parts[2].trim().parse::<u8>().ok()?;
                return Some(Color::new(r, g, b));
            }
        }

        // Format HEX #00FFFF
        if input.starts_with('#') {
            let hex: &str = input.trim_start_matches('#');
            if hex.len() == 6 {
                let r: u8 = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g: u8 = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b: u8 = u8::from_str_radix(&hex[4..6], 16).ok()?;
                return Some(Color::new(r, g, b));
            }
        }

        None
    }

    /// Returns the RGB color of the text.
    ///
    ///  # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let c: Color = Color::new(0, 0, 255); // Blue
    ///
    /// assert_eq!(c.to_rgb(), "rgb(0, 0, 255)".to_string());
    /// assert_ne!(c.to_rgb(), "rgb(255, 0, 0)".to_string());
    /// ```
    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.red, self.green, self.blue)
    }

    /// Returns the HEX color of the text.
    ///
    ///  # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let c: Color = Color::new(0, 0, 255); // Blue
    ///
    /// assert_eq!(c.to_hex(), "#0000FF".to_string());
    /// assert_ne!(c.to_hex(), "#FF0000".to_string());
    /// ```
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    /// Returns the HEX color of the text.
    ///
    /// # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let color_red: Color = Color::new(255, 0, 0); // Red
    /// let color_green: Color = Color::new(0, 255, 0); // Green
    ///
    /// let color_yellow: Color = color_red.join(&color_green);
    ///
    /// assert_eq!(color_yellow.to_rgb(), "rgb(255, 255, 0)".to_string());
    /// assert_ne!(color_yellow.to_rgb(), "rgb(0, 255, 0)".to_string());
    /// ```
    pub fn join(&self, c: &Color) -> Color {
        Color::new(
            ((self.red as u16 + c.red as u16) / 2) as u8,
            ((self.green as u16 + c.green as u16) / 2) as u8,
            ((self.blue as u16 + c.blue as u16) / 2) as u8,
        )
    }

    /// Returns the opposite color.
    ///
    /// # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let c: Color = Color::default(); // #FFFFFF
    ///
    /// assert_eq!(c.reverse().to_hex(), "#000000".to_string());
    /// assert_ne!(c.reverse().to_hex(), "#FFFFFF".to_string());
    ///
    /// let c: Color = Color::new(0, 255, 0); // Green
    ///
    /// assert_eq!(c.reverse().to_hex(), "#FF00FF".to_string()); // Magenta
    /// assert_ne!(c.reverse().to_hex(), "#0000FF".to_string());
    /// ```
    pub fn reverse(&self) -> Self {
        Self {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
        }
    }

    /// Returns a random color based on the instantiated color.
    ///
    /// # Exemple
    /// ```
    /// use color::Color;
    ///
    /// let c: Color = Color::default(); // #FFFFFF
    /// let c: Color = c.random_magic();
    /// ```
    pub fn random_magic(&self) -> Color {
        Color::new(
            self.red.abs_diff(random::<u8>()),
            self.green.abs_diff(random::<u8>()),
            self.blue.abs_diff(random::<u8>()),
        )
    }

    /// Converts the color to HSL.
    /// Returns `(hue °, saturation 0-100, lightness 0-100)`.
    fn hsl(&self) -> (f64, f64, f64) {
        let r: f64 = self.red as f64 / 255.0;
        let g: f64 = self.green as f64 / 255.0;
        let b: f64 = self.blue as f64 / 255.0;

        let max: f64 = r.max(g).max(b);
        let min: f64 = r.min(g).min(b);
        let delta: f64 = max - min;

        let l: f64 = (max + min) / 2.0;

        let s: f64 = if delta == 0.0 {
            0.0
        } else if l > 0.5 {
            delta / (2.0 - max - min)
        } else {
            delta / (max + min)
        };

        let h: f64 = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        ((h + 360.0) % 360.0, s * 100.0, l * 100.0)
    }

    /// Returns the complementary color by rotating the hue 180° in HSL space.
    ///
    /// # Examples
    /// ```
    /// let blue = Color::new(40, 102, 189);
    /// let amber = blue.complementary();
    ///
    /// assert_eq!(amber.red,   189);
    /// assert_eq!(amber.green, 127);
    /// assert_eq!(amber.blue,   40);
    /// ```
    pub fn complementary(&self) -> Color {
        let (h, s, l) = self.hsl();
        let h_comp: f64 = (h + 180.0) % 360.0;

        let s: f64 = s / 100.0;
        let l: f64 = l / 100.0;
        let a: f64 = s * l.min(1.0 - l);

        let channel = |n: f64| -> u8 {
            let k: f64 = (n + h_comp / 30.0) % 12.0;
            let value: f64 = l - a * (-1.0_f64).max((k - 3.0).min(9.0 - k).min(1.0));
            (value * 255.0).round() as u8
        };

        Color::new(channel(0.0), channel(8.0), channel(4.0))
    }

    /// Returns the HSL color of the text.
    pub fn to_hsl(&self) -> String {
        let (h, s, l) = self.hsl();
        format!("hsl({}, {}%, {}%)", h as u8, s as u8, l as u8)
    }
}
