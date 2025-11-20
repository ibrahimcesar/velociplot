//! Canvas abstraction for drawing

use super::{Bounds, Point2D};
use crate::error::Result;

/// Trait for objects that can be drawn on a canvas
pub trait Drawable {
    /// Draw the object on the given canvas
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()>;
}

/// Abstract canvas for drawing primitives
///
/// This trait abstracts over different rendering backends
pub trait Canvas {
    /// Get the canvas dimensions in pixels
    fn dimensions(&self) -> (u32, u32);

    /// Get the current data bounds
    fn bounds(&self) -> Bounds;

    /// Set the data bounds for coordinate transformation
    fn set_bounds(&mut self, bounds: Bounds);

    /// Transform data coordinates to canvas pixel coordinates
    fn transform(&self, point: &Point2D) -> (f32, f32);

    /// Draw a line between two points (in data coordinates)
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw_line(
        &mut self,
        from: &Point2D,
        to: &Point2D,
        color: &[u8; 4],
        width: f32,
    ) -> Result<()>;

    /// Draw a circle at a point (in data coordinates)
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw_circle(
        &mut self,
        center: &Point2D,
        radius: f32,
        color: &[u8; 4],
        filled: bool,
    ) -> Result<()>;

    /// Draw a filled rectangle (in data coordinates)
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw_rectangle(
        &mut self,
        top_left: &Point2D,
        width: f64,
        height: f64,
        color: &[u8; 4],
    ) -> Result<()>;

    /// Draw text at a position (in pixel coordinates)
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: &[u8; 4]) -> Result<()>;

    /// Fill the background with a color
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn fill_background(&mut self, color: &[u8; 4]) -> Result<()>;

    /// Draw a line between two points (in pixel coordinates)
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw_line_pixels(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: &[u8; 4],
        width: f32,
    ) -> Result<()>;

    /// Draw text at a position (in pixel coordinates)
    ///
    /// # Errors
    ///
    /// Returns an error if drawing fails
    fn draw_text_pixels(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()>;

    /// Calculate density (non-white pixels) in the four corners
    /// Returns (`UpperRight`, `UpperLeft`, `LowerRight`, `LowerLeft`) densities
    /// Values are between 0.0 (empty/white) and 1.0 (full/colored)
    ///
    /// This is used for automatic legend positioning to find the least crowded corner.
    /// Default implementation returns all zeros (which defaults to `UpperRight`).
    fn calculate_corner_densities(
        &self,
        _legend_width: u32,
        _legend_height: u32,
    ) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 0.0, 0.0) // Default: no density info available
    }
}

/// Linear scale for coordinate transformation
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct LinearScale {
    data_min: f64,
    data_max: f64,
    pixel_min: f32,
    pixel_max: f32,
}

impl LinearScale {
    /// Create a new linear scale
    #[must_use]
    #[allow(dead_code)]
    pub const fn new(data_min: f64, data_max: f64, pixel_min: f32, pixel_max: f32) -> Self {
        Self {
            data_min,
            data_max,
            pixel_min,
            pixel_max,
        }
    }

    /// Transform a data value to pixel coordinates
    #[must_use]
    #[allow(dead_code)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn transform(&self, value: f64) -> f32 {
        let data_range = self.data_max - self.data_min;
        let pixel_range = self.pixel_max - self.pixel_min;
        let normalized = (value - self.data_min) / data_range;
        self.pixel_min + normalized as f32 * pixel_range
    }
}
