//! Color definitions and palettes

/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Color {
    /// Create a new color from RGBA components
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let red = Color::rgb(255, 0, 0);
    /// let semi_transparent_blue = Color::rgba(0, 0, 255, 128);
    /// ```
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new opaque color from RGB components
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Convert to RGBA array
    #[must_use]
    pub const fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Linear interpolation between two colors
    ///
    /// # Arguments
    ///
    /// * `other` - The target color
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let red = Color::RED;
    /// let blue = Color::BLUE;
    /// let purple = red.lerp(&blue, 0.5); // 50% between red and blue
    /// ```
    #[must_use]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let r = (f32::from(self.r) * (1.0 - t) + f32::from(other.r) * t) as u8;
        let g = (f32::from(self.g) * (1.0 - t) + f32::from(other.g) * t) as u8;
        let b = (f32::from(self.b) * (1.0 - t) + f32::from(other.b) * t) as u8;
        let a = (f32::from(self.a) * (1.0 - t) + f32::from(other.a) * t) as u8;
        Self::rgba(r, g, b, a)
    }

    /// Create color from hex string
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let red = Color::from_hex("#FF0000").unwrap();
    /// let blue = Color::from_hex("#0000FF").unwrap();
    /// let with_alpha = Color::from_hex("#FF0000AA").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is invalid
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                Ok(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;
                Ok(Self::rgba(r, g, b, a))
            }
            _ => Err(format!("Invalid hex color length: {}", hex.len())),
        }
    }

    // Standard colors
    /// White color
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    /// Black color
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    /// Red color
    pub const RED: Self = Self::rgb(255, 0, 0);
    /// Green color
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    /// Blue color
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    /// Yellow color
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    /// Cyan color
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    /// Magenta color
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);
    /// Gray color
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    /// Orange color
    pub const ORANGE: Self = Self::rgb(255, 165, 0);
    /// Purple color
    pub const PURPLE: Self = Self::rgb(128, 0, 128);
    /// Pink color
    pub const PINK: Self = Self::rgb(255, 192, 203);
    /// Brown color
    pub const BROWN: Self = Self::rgb(165, 42, 42);
    /// Rebecca Purple color (named after Rebecca Meyer)
    pub const REBECCAPURPLE: Self = Self::rgb(102, 51, 153);
    /// Transparent color
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
}

/// Color palette for plotting
pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    /// Create a new palette from colors
    #[must_use]
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    /// Get color at index (wraps around)
    #[must_use]
    pub fn get(&self, index: usize) -> Color {
        self.colors[index % self.colors.len()]
    }

    /// Default matplotlib-like color cycle
    #[must_use]
    pub fn default() -> Self {
        Self::new(vec![
            Color::from_hex("#1f77b4").unwrap(), // blue
            Color::from_hex("#ff7f0e").unwrap(), // orange
            Color::from_hex("#2ca02c").unwrap(), // green
            Color::from_hex("#d62728").unwrap(), // red
            Color::from_hex("#9467bd").unwrap(), // purple
            Color::from_hex("#8c564b").unwrap(), // brown
            Color::from_hex("#e377c2").unwrap(), // pink
            Color::from_hex("#7f7f7f").unwrap(), // gray
        ])
    }

    /// Okabe-Ito colorblind-friendly palette
    #[must_use]
    pub fn colorblind_safe() -> Self {
        Self::new(vec![
            Color::from_hex("#E69F00").unwrap(), // orange
            Color::from_hex("#56B4E9").unwrap(), // sky blue
            Color::from_hex("#009E73").unwrap(), // bluish green
            Color::from_hex("#F0E442").unwrap(), // yellow
            Color::from_hex("#0072B2").unwrap(), // blue
            Color::from_hex("#D55E00").unwrap(), // vermillion
            Color::from_hex("#CC79A7").unwrap(), // reddish purple
        ])
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::default()
    }
}

/// Colormap for continuous data visualization
#[derive(Debug, Clone)]
pub struct Colormap {
    colors: Vec<Color>,
    name: String,
}

impl Colormap {
    /// Create a new colormap from a list of colors
    #[must_use]
    pub fn new(name: impl Into<String>, colors: Vec<Color>) -> Self {
        Self {
            name: name.into(),
            colors,
        }
    }

    /// Get the name of this colormap
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get color at normalized position (0.0 to 1.0)
    #[must_use]
    pub fn get(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        if self.colors.is_empty() {
            return Color::BLACK;
        }
        if self.colors.len() == 1 {
            return self.colors[0];
        }

        // Map t to color array index
        let scaled = t * (self.colors.len() - 1) as f32;
        let index = scaled.floor() as usize;
        let frac = scaled - index as f32;

        if index >= self.colors.len() - 1 {
            return self.colors[self.colors.len() - 1];
        }

        // Linear interpolation between adjacent colors
        self.colors[index].lerp(&self.colors[index + 1], frac)
    }

    /// Viridis colormap (perceptually uniform, colorblind-friendly)
    #[must_use]
    pub fn viridis() -> Self {
        Self::new(
            "viridis",
            vec![
                Color::from_hex("#440154").unwrap(),
                Color::from_hex("#482878").unwrap(),
                Color::from_hex("#3e4989").unwrap(),
                Color::from_hex("#31688e").unwrap(),
                Color::from_hex("#26828e").unwrap(),
                Color::from_hex("#1f9e89").unwrap(),
                Color::from_hex("#35b779").unwrap(),
                Color::from_hex("#6ece58").unwrap(),
                Color::from_hex("#b5de2b").unwrap(),
                Color::from_hex("#fde725").unwrap(),
            ],
        )
    }

    /// Plasma colormap (perceptually uniform)
    #[must_use]
    pub fn plasma() -> Self {
        Self::new(
            "plasma",
            vec![
                Color::from_hex("#0d0887").unwrap(),
                Color::from_hex("#46039f").unwrap(),
                Color::from_hex("#7201a8").unwrap(),
                Color::from_hex("#9c179e").unwrap(),
                Color::from_hex("#bd3786").unwrap(),
                Color::from_hex("#d8576b").unwrap(),
                Color::from_hex("#ed7953").unwrap(),
                Color::from_hex("#fb9f3a").unwrap(),
                Color::from_hex("#fdca26").unwrap(),
                Color::from_hex("#f0f921").unwrap(),
            ],
        )
    }

    /// Inferno colormap (perceptually uniform)
    #[must_use]
    pub fn inferno() -> Self {
        Self::new(
            "inferno",
            vec![
                Color::from_hex("#000004").unwrap(),
                Color::from_hex("#1b0c41").unwrap(),
                Color::from_hex("#4a0c6b").unwrap(),
                Color::from_hex("#781c6d").unwrap(),
                Color::from_hex("#a52c60").unwrap(),
                Color::from_hex("#cf4446").unwrap(),
                Color::from_hex("#ed6925").unwrap(),
                Color::from_hex("#fb9b06").unwrap(),
                Color::from_hex("#f7d13d").unwrap(),
                Color::from_hex("#fcffa4").unwrap(),
            ],
        )
    }

    /// Magma colormap (perceptually uniform)
    #[must_use]
    pub fn magma() -> Self {
        Self::new(
            "magma",
            vec![
                Color::from_hex("#000004").unwrap(),
                Color::from_hex("#180f3d").unwrap(),
                Color::from_hex("#440f76").unwrap(),
                Color::from_hex("#721f81").unwrap(),
                Color::from_hex("#9e2f7f").unwrap(),
                Color::from_hex("#cd4071").unwrap(),
                Color::from_hex("#f1605d").unwrap(),
                Color::from_hex("#fd9668").unwrap(),
                Color::from_hex("#feca8d").unwrap(),
                Color::from_hex("#fcfdbf").unwrap(),
            ],
        )
    }

    /// Coolwarm diverging colormap
    #[must_use]
    pub fn coolwarm() -> Self {
        Self::new(
            "coolwarm",
            vec![
                Color::from_hex("#3b4cc0").unwrap(),
                Color::from_hex("#7396d3").unwrap(),
                Color::from_hex("#a5c5e7").unwrap(),
                Color::from_hex("#dddddd").unwrap(),
                Color::from_hex("#edac9c").unwrap(),
                Color::from_hex("#da746a").unwrap(),
                Color::from_hex("#b40426").unwrap(),
            ],
        )
    }

    /// Jet colormap (rainbow, not recommended for scientific use)
    #[must_use]
    pub fn jet() -> Self {
        Self::new(
            "jet",
            vec![
                Color::from_hex("#00007f").unwrap(),
                Color::from_hex("#0000ff").unwrap(),
                Color::from_hex("#007fff").unwrap(),
                Color::from_hex("#00ffff").unwrap(),
                Color::from_hex("#7fff7f").unwrap(),
                Color::from_hex("#ffff00").unwrap(),
                Color::from_hex("#ff7f00").unwrap(),
                Color::from_hex("#ff0000").unwrap(),
                Color::from_hex("#7f0000").unwrap(),
            ],
        )
    }

    /// Grayscale colormap
    #[must_use]
    pub fn grayscale() -> Self {
        Self::new("grayscale", vec![Color::BLACK, Color::WHITE])
    }
}
