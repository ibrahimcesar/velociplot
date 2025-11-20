//! 2D point representation

/// A point in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl Point2D {
    /// Create a new point
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let point = Point2D::new(1.0, 2.0);
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, 2.0);
    /// ```
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculate distance to another point
    #[must_use]
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Translate point by given offsets
    #[must_use]
    pub const fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl From<(f64, f64)> for Point2D {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl From<[f64; 2]> for Point2D {
    fn from([x, y]: [f64; 2]) -> Self {
        Self::new(x, y)
    }
}
