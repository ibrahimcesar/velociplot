//! Bubble chart implementation for 3-variable visualization
//!
//! Bubble charts are scatter plots where marker size represents a third variable,
//! allowing visualization of three dimensions of data simultaneously.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = vec![2.0, 4.0, 3.0, 5.0, 4.0];
//! let sizes = vec![10.0, 20.0, 15.0, 25.0, 18.0];
//!
//! let bubble = BubbleChart::new(x, y, sizes).unwrap()
//!     .color(Color::from_hex("#3498db").unwrap())
//!     .label("Sales Data");
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::{Error, Result};
use crate::legend::LegendEntry;
use crate::plots::scatter::{MarkerShape, MarkerStyle};

/// Bubble chart for visualizing three variables (x, y, size)
pub struct BubbleChart {
    x: Vec<f64>,
    y: Vec<f64>,
    sizes: Vec<f64>,
    color: Color,
    marker_shape: MarkerShape,
    marker_style: MarkerStyle,
    size_scale: f32,
    min_size: f32,
    max_size: f32,
    label: Option<String>,
    opacity: f32,
}

impl BubbleChart {
    /// Create a new bubble chart
    ///
    /// # Arguments
    ///
    /// * `x` - X-axis values
    /// * `y` - Y-axis values
    /// * `sizes` - Size values for each point (will be scaled to visual size)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let x = vec![1.0, 2.0, 3.0];
    /// let y = vec![2.0, 4.0, 3.0];
    /// let sizes = vec![10.0, 20.0, 15.0];
    /// let bubble = BubbleChart::new(x, y, sizes);
    /// ```
    pub fn new(x: Vec<f64>, y: Vec<f64>, sizes: Vec<f64>) -> Result<Self> {
        if x.is_empty() || y.is_empty() || sizes.is_empty() {
            return Err(Error::InvalidData(
                "Bubble chart data cannot be empty".into(),
            ));
        }

        if x.len() != y.len() || x.len() != sizes.len() {
            return Err(Error::InvalidData(
                "x, y, and sizes must have the same length".into(),
            ));
        }

        // Check for negative sizes
        for &size in &sizes {
            if size < 0.0 {
                return Err(Error::InvalidData(
                    "Size values must be non-negative".into(),
                ));
            }
        }

        Ok(Self {
            x,
            y,
            sizes,
            color: Color::from_hex("#3498db").unwrap(),
            marker_shape: MarkerShape::Circle,
            marker_style: MarkerStyle::Filled,
            size_scale: 1.0,
            min_size: 3.0,
            max_size: 30.0,
            label: None,
            opacity: 0.6,
        })
    }

    /// Set the color for all bubbles
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the marker shape
    #[must_use]
    pub fn marker_shape(mut self, shape: MarkerShape) -> Self {
        self.marker_shape = shape;
        self
    }

    /// Set the marker style (filled or outline)
    #[must_use]
    pub fn marker_style(mut self, style: MarkerStyle) -> Self {
        self.marker_style = style;
        self
    }

    /// Set the size scale factor
    #[must_use]
    pub fn size_scale(mut self, scale: f32) -> Self {
        self.size_scale = scale.max(0.1);
        self
    }

    /// Set the minimum marker size in pixels
    #[must_use]
    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = size.max(1.0);
        self
    }

    /// Set the maximum marker size in pixels
    #[must_use]
    pub fn max_size(mut self, size: f32) -> Self {
        self.max_size = size.max(self.min_size);
        self
    }

    /// Set the size range (min and max)
    #[must_use]
    pub fn size_range(mut self, min: f32, max: f32) -> Self {
        self.min_size = min.max(1.0);
        self.max_size = max.max(self.min_size);
        self
    }

    /// Set the opacity (0.0 to 1.0)
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the label for legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get legend entry for this bubble chart
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label
            .as_ref()
            .map(|label| LegendEntry::new(label.clone()).color(self.color))
    }

    /// Scale a size value to pixel radius
    fn scale_size(&self, size: f64) -> f32 {
        if self.sizes.is_empty() {
            return self.min_size;
        }

        let min_data = self.sizes.iter().copied().fold(f64::INFINITY, f64::min);
        let max_data = self.sizes.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        if (max_data - min_data).abs() < f64::EPSILON {
            return (self.min_size + self.max_size) / 2.0;
        }

        let normalized = ((size - min_data) / (max_data - min_data)) as f32;
        (self.min_size + normalized * (self.max_size - self.min_size)) * self.size_scale
    }

    /// Draw a single marker at the given position
    fn draw_marker(
        &self,
        canvas: &mut dyn Canvas,
        point: &Point2D,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        match self.marker_shape {
            MarkerShape::Circle => {
                let filled = matches!(self.marker_style, MarkerStyle::Filled);
                canvas.draw_circle(point, size, color, filled)?;
            }
            MarkerShape::Square => {
                self.draw_square(canvas, point, size, color)?;
            }
            MarkerShape::Triangle => {
                self.draw_triangle(canvas, point, size, color)?;
            }
            MarkerShape::Diamond => {
                self.draw_diamond(canvas, point, size, color)?;
            }
            MarkerShape::Plus => {
                self.draw_plus(canvas, point, size, color)?;
            }
            MarkerShape::Cross => {
                self.draw_cross(canvas, point, size, color)?;
            }
        }
        Ok(())
    }

    fn draw_square(
        &self,
        canvas: &mut dyn Canvas,
        center: &Point2D,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let half = f64::from(size);
        let top_left = Point2D::new(center.x - half, center.y - half);

        if matches!(self.marker_style, MarkerStyle::Filled) {
            canvas.draw_rectangle(&top_left, half * 2.0, half * 2.0, color)?;
        } else {
            // Draw outline
            let bottom_right = Point2D::new(center.x + half, center.y + half);
            let top_right = Point2D::new(center.x + half, center.y - half);
            let bottom_left = Point2D::new(center.x - half, center.y + half);

            canvas.draw_line(&top_left, &top_right, color, 1.5)?;
            canvas.draw_line(&top_right, &bottom_right, color, 1.5)?;
            canvas.draw_line(&bottom_right, &bottom_left, color, 1.5)?;
            canvas.draw_line(&bottom_left, &top_left, color, 1.5)?;
        }
        Ok(())
    }

    fn draw_triangle(
        &self,
        canvas: &mut dyn Canvas,
        center: &Point2D,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let h = f64::from(size);
        let top = Point2D::new(center.x, center.y - h);
        let left = Point2D::new(center.x - h * 0.866, center.y + h * 0.5);
        let right = Point2D::new(center.x + h * 0.866, center.y + h * 0.5);

        canvas.draw_line(&top, &left, color, 1.5)?;
        canvas.draw_line(&left, &right, color, 1.5)?;
        canvas.draw_line(&right, &top, color, 1.5)?;
        Ok(())
    }

    fn draw_diamond(
        &self,
        canvas: &mut dyn Canvas,
        center: &Point2D,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let h = f64::from(size);
        let top = Point2D::new(center.x, center.y - h);
        let left = Point2D::new(center.x - h, center.y);
        let bottom = Point2D::new(center.x, center.y + h);
        let right = Point2D::new(center.x + h, center.y);

        canvas.draw_line(&top, &right, color, 1.5)?;
        canvas.draw_line(&right, &bottom, color, 1.5)?;
        canvas.draw_line(&bottom, &left, color, 1.5)?;
        canvas.draw_line(&left, &top, color, 1.5)?;
        Ok(())
    }

    fn draw_plus(
        &self,
        canvas: &mut dyn Canvas,
        center: &Point2D,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let h = f64::from(size);
        canvas.draw_line(
            &Point2D::new(center.x - h, center.y),
            &Point2D::new(center.x + h, center.y),
            color,
            2.0,
        )?;
        canvas.draw_line(
            &Point2D::new(center.x, center.y - h),
            &Point2D::new(center.x, center.y + h),
            color,
            2.0,
        )?;
        Ok(())
    }

    fn draw_cross(
        &self,
        canvas: &mut dyn Canvas,
        center: &Point2D,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let h = f64::from(size) * 0.707; // sqrt(2)/2
        canvas.draw_line(
            &Point2D::new(center.x - h, center.y - h),
            &Point2D::new(center.x + h, center.y + h),
            color,
            2.0,
        )?;
        canvas.draw_line(
            &Point2D::new(center.x - h, center.y + h),
            &Point2D::new(center.x + h, center.y - h),
            color,
            2.0,
        )?;
        Ok(())
    }
}

impl Drawable for BubbleChart {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        // Apply opacity to color
        let mut color = self.color.to_rgba();
        color[3] = (f32::from(color[3]) * self.opacity) as u8;

        for i in 0..self.x.len() {
            let point = Point2D::new(self.x[i], self.y[i]);
            let size = self.scale_size(self.sizes[i]);
            self.draw_marker(canvas, &point, size, &color)?;
        }

        Ok(())
    }
}

impl BubbleChart {
    /// Get bounds for this bubble chart
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.x.is_empty() || self.y.is_empty() {
            return None;
        }

        let x_min = self.x.iter().copied().fold(f64::INFINITY, f64::min);
        let x_max = self.x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let y_min = self.y.iter().copied().fold(f64::INFINITY, f64::min);
        let y_max = self.y.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        Some(Bounds::new(x_min, x_max, y_min, y_max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_creation() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 3.0];
        let sizes = vec![10.0, 20.0, 15.0];
        let bubble = BubbleChart::new(x, y, sizes).unwrap();
        assert!(bubble.bounds().is_some());
    }

    #[test]
    fn test_bubble_empty_data() {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let sizes: Vec<f64> = vec![];
        let result = BubbleChart::new(x, y, sizes);
        assert!(result.is_err());
    }

    #[test]
    fn test_bubble_mismatched_lengths() {
        let x = vec![1.0, 2.0];
        let y = vec![2.0, 4.0, 3.0];
        let sizes = vec![10.0, 20.0];
        let result = BubbleChart::new(x, y, sizes);
        assert!(result.is_err());
    }

    #[test]
    fn test_bubble_negative_sizes() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 3.0];
        let sizes = vec![10.0, -5.0, 15.0];
        let result = BubbleChart::new(x, y, sizes);
        assert!(result.is_err());
    }

    #[test]
    fn test_size_scaling() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 3.0];
        let sizes = vec![10.0, 20.0, 30.0];
        let bubble = BubbleChart::new(x, y, sizes).unwrap().size_range(5.0, 25.0);

        // Min size should map to 5.0
        let min_scaled = bubble.scale_size(10.0);
        assert!((min_scaled - 5.0).abs() < 0.1);

        // Max size should map to 25.0
        let max_scaled = bubble.scale_size(30.0);
        assert!((max_scaled - 25.0).abs() < 0.1);
    }

    #[test]
    fn test_bubble_bounds() {
        let x = vec![1.0, 5.0, 3.0];
        let y = vec![2.0, 8.0, 4.0];
        let sizes = vec![10.0, 20.0, 15.0];
        let bubble = BubbleChart::new(x, y, sizes).unwrap();
        let bounds = bubble.bounds().unwrap();

        assert_eq!(bounds.x_min, 1.0);
        assert_eq!(bounds.x_max, 5.0);
        assert_eq!(bounds.y_min, 2.0);
        assert_eq!(bounds.y_max, 8.0);
    }
}
