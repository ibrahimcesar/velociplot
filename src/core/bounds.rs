//! Bounds for coordinate systems

use super::Point2D;

/// Rectangular bounds in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Minimum x value
    pub x_min: f64,
    /// Maximum x value
    pub x_max: f64,
    /// Minimum y value
    pub y_min: f64,
    /// Maximum y value
    pub y_max: f64,
}

impl Bounds {
    /// Create new bounds
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let bounds = Bounds::new(0.0, 10.0, 0.0, 5.0);
    /// assert_eq!(bounds.width(), 10.0);
    /// assert_eq!(bounds.height(), 5.0);
    /// ```
    #[must_use]
    pub const fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    /// Get the width of the bounds
    #[must_use]
    pub fn width(&self) -> f64 {
        self.x_max - self.x_min
    }

    /// Get the height of the bounds
    #[must_use]
    pub fn height(&self) -> f64 {
        self.y_max - self.y_min
    }

    /// Check if a point is within bounds
    #[must_use]
    pub fn contains(&self, point: &Point2D) -> bool {
        point.x >= self.x_min
            && point.x <= self.x_max
            && point.y >= self.y_min
            && point.y <= self.y_max
    }

    /// Expand bounds to include a point
    #[must_use]
    pub fn expand(&self, point: &Point2D) -> Self {
        Self {
            x_min: self.x_min.min(point.x),
            x_max: self.x_max.max(point.x),
            y_min: self.y_min.min(point.y),
            y_max: self.y_max.max(point.y),
        }
    }

    /// Add padding to bounds (as fraction of size)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let bounds = Bounds::new(0.0, 10.0, 0.0, 10.0);
    /// let padded = bounds.with_padding(0.1);
    /// assert_eq!(padded.width(), 12.0); // 10 + 10% on each side
    /// ```
    #[must_use]
    pub fn with_padding(&self, fraction: f64) -> Self {
        let dx = self.width() * fraction;
        let dy = self.height() * fraction;
        Self {
            x_min: self.x_min - dx,
            x_max: self.x_max + dx,
            y_min: self.y_min - dy,
            y_max: self.y_max + dy,
        }
    }

    /// Add padding only to the top (for vertical bar charts where baseline should stay at `y_min`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::core::Bounds;
    /// let bounds = Bounds::new(0.0, 10.0, 0.0, 100.0);
    /// let padded = bounds.with_padding_top(0.1);
    /// assert_eq!(padded.y_min, 0.0); // Bottom stays at 0
    /// assert_eq!(padded.y_max, 110.0); // Top has padding
    /// ```
    #[must_use]
    pub fn with_padding_top(&self, fraction: f64) -> Self {
        let dx = self.width() * fraction;
        let dy = self.height() * fraction;
        Self {
            x_min: self.x_min - dx,
            x_max: self.x_max + dx,
            y_min: self.y_min,      // Keep bottom at baseline
            y_max: self.y_max + dy, // Only add padding to top
        }
    }

    /// Add padding only to the right (for horizontal bar charts where baseline should stay at `x_min`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::core::Bounds;
    /// let bounds = Bounds::new(0.0, 100.0, 0.0, 10.0);
    /// let padded = bounds.with_padding_right(0.1);
    /// assert_eq!(padded.x_min, 0.0); // Left stays at 0
    /// assert_eq!(padded.x_max, 110.0); // Right has padding
    /// ```
    #[must_use]
    pub fn with_padding_right(&self, fraction: f64) -> Self {
        let dx = self.width() * fraction;
        let dy = self.height() * fraction;
        Self {
            x_min: self.x_min,      // Keep left at baseline
            x_max: self.x_max + dx, // Only add padding to right
            y_min: self.y_min - dy,
            y_max: self.y_max + dy,
        }
    }

    /// Create bounds from a collection of points
    ///
    /// # Panics
    ///
    /// Panics if the iterator is empty
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point2D>,
    {
        let mut iter = points.into_iter();
        let first = iter
            .next()
            .expect("Cannot create bounds from empty iterator");

        let mut bounds = Self::new(first.x, first.x, first.y, first.y);
        for point in iter {
            bounds = bounds.expand(&point);
        }
        bounds
    }

    /// Combine two bounds to create a new bounds that contains both
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let bounds1 = Bounds::new(0.0, 5.0, 0.0, 10.0);
    /// let bounds2 = Bounds::new(3.0, 8.0, 5.0, 15.0);
    /// let combined = bounds1.union(&bounds2);
    /// assert_eq!(combined.x_min, 0.0);
    /// assert_eq!(combined.x_max, 8.0);
    /// assert_eq!(combined.y_min, 0.0);
    /// assert_eq!(combined.y_max, 15.0);
    /// ```
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            x_min: self.x_min.min(other.x_min),
            x_max: self.x_max.max(other.x_max),
            y_min: self.y_min.min(other.y_min),
            y_max: self.y_max.max(other.y_max),
        }
    }
}
