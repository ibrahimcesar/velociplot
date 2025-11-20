//! Bar plot implementation for categorical and discrete data

use crate::color::Color;
use crate::core::{Bounds, Canvas, DataSeries, Drawable};
use crate::error::Result;
use crate::legend::LegendEntry;

/// Bar plot orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarOrientation {
    /// Vertical bars (default)
    Vertical,
    /// Horizontal bars
    Horizontal,
}

/// Bar plot for categorical or discrete data
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::plots::bar::{BarPlot, BarOrientation};
/// let data = Series::from_tuples(&[(1.0, 5.0), (2.0, 8.0), (3.0, 3.0)]);
///
/// let bar_plot = BarPlot::new(data)
///     .orientation(BarOrientation::Vertical)
///     .bar_width(0.8)
///     .color(Color::from_hex("#3498db").unwrap())
///     .label("Sales");
/// ```
pub struct BarPlot {
    data: Box<dyn DataSeries>,
    color: Color,
    orientation: BarOrientation,
    bar_width: f64,
    label: Option<String>,
}

impl BarPlot {
    /// Create a new bar plot from a data series
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// # use velociplot::plots::bar::BarPlot;
    /// let data = Series::from_tuples(&[(1.0, 10.0), (2.0, 20.0), (3.0, 15.0)]);
    /// let bar_plot = BarPlot::new(data);
    /// ```
    #[must_use]
    pub fn new(data: impl DataSeries + 'static) -> Self {
        Self {
            data: Box::new(data),
            color: Color::from_hex("#3498db").unwrap_or(Color::BLUE),
            orientation: BarOrientation::Vertical,
            bar_width: 0.8,
            label: None,
        }
    }

    /// Set the bar color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the bar orientation
    #[must_use]
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the bar width (as a fraction of spacing, typically 0.5-1.0)
    #[must_use]
    pub fn bar_width(mut self, width: f64) -> Self {
        self.bar_width = width.clamp(0.1, 2.0);
        self
    }

    /// Set the label for the legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the bounding box of the data
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.data.is_empty() {
            return None;
        }

        let points: Vec<_> = self.data.points().collect();
        let mut bounds = Bounds::from_points(points);

        // Extend bounds to include zero baseline for bars
        match self.orientation {
            BarOrientation::Vertical => {
                if bounds.y_min > 0.0 {
                    bounds.y_min = 0.0;
                }
                if bounds.y_max < 0.0 {
                    bounds.y_max = 0.0;
                }
            }
            BarOrientation::Horizontal => {
                if bounds.x_min > 0.0 {
                    bounds.x_min = 0.0;
                }
                if bounds.x_max < 0.0 {
                    bounds.x_max = 0.0;
                }
            }
        }

        Some(bounds)
    }

    /// Create a legend entry for this bar plot
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label
            .as_ref()
            .map(|label| LegendEntry::new(label).color(self.color).line_width(2.0))
    }

    /// Draw a single bar as a filled rectangle
    fn draw_bar(&self, canvas: &mut dyn Canvas, x1: f32, y1: f32, x2: f32, y2: f32) -> Result<()> {
        let color = self.color.to_rgba();

        // Ensure we draw from top-left to bottom-right
        let left = x1.min(x2);
        let right = x1.max(x2);
        let top = y1.min(y2);
        let bottom = y1.max(y2);

        // Fill rectangle with horizontal lines
        let height = (bottom - top).abs();
        let steps = (height.ceil() as i32).max(1);

        for i in 0..steps {
            let y = top + i as f32;
            if y <= bottom {
                canvas.draw_line_pixels(left, y, right, y, &color, 1.0)?;
            }
        }

        Ok(())
    }
}

impl Drawable for BarPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let bounds = canvas.bounds();
        let (width, height) = canvas.dimensions();

        // Same margins as axes
        let margin_left = 60.0;
        let margin_right = 20.0;
        let margin_top = 40.0;
        let margin_bottom = 40.0;

        let pixel_min_x = margin_left;
        let pixel_max_x = width as f32 - margin_right;
        let pixel_min_y = margin_top;
        let pixel_max_y = height as f32 - margin_bottom;

        match self.orientation {
            BarOrientation::Vertical => {
                // Find baseline (y=0) position
                let zero_y =
                    value_to_pixel_y(0.0, bounds.y_min, bounds.y_max, pixel_min_y, pixel_max_y);

                for point in self.data.points() {
                    let x_pixel = value_to_pixel_x(
                        point.x,
                        bounds.x_min,
                        bounds.x_max,
                        pixel_min_x,
                        pixel_max_x,
                    );
                    let y_pixel = value_to_pixel_y(
                        point.y,
                        bounds.y_min,
                        bounds.y_max,
                        pixel_min_y,
                        pixel_max_y,
                    );

                    // Calculate bar width in pixels
                    let spacing = (bounds.x_max - bounds.x_min) / (self.data.len() as f64).max(1.0);
                    let bar_width_pixels = (spacing * self.bar_width)
                        * f64::from(pixel_max_x - pixel_min_x)
                        / (bounds.x_max - bounds.x_min);
                    let half_width = (bar_width_pixels / 2.0) as f32;

                    // Draw bar from baseline to value
                    self.draw_bar(
                        canvas,
                        x_pixel - half_width,
                        zero_y,
                        x_pixel + half_width,
                        y_pixel,
                    )?;
                }
            }
            BarOrientation::Horizontal => {
                // Find baseline (x=0) position
                let zero_x =
                    value_to_pixel_x(0.0, bounds.x_min, bounds.x_max, pixel_min_x, pixel_max_x);

                for point in self.data.points() {
                    let x_pixel = value_to_pixel_x(
                        point.x,
                        bounds.x_min,
                        bounds.x_max,
                        pixel_min_x,
                        pixel_max_x,
                    );
                    let y_pixel = value_to_pixel_y(
                        point.y,
                        bounds.y_min,
                        bounds.y_max,
                        pixel_min_y,
                        pixel_max_y,
                    );

                    // Calculate bar height in pixels
                    let spacing = (bounds.y_max - bounds.y_min) / (self.data.len() as f64).max(1.0);
                    let bar_height_pixels = (spacing * self.bar_width)
                        * f64::from(pixel_max_y - pixel_min_y)
                        / (bounds.y_max - bounds.y_min);
                    let half_height = (bar_height_pixels / 2.0) as f32;

                    // Draw bar from baseline to value
                    self.draw_bar(
                        canvas,
                        zero_x,
                        y_pixel - half_height,
                        x_pixel,
                        y_pixel + half_height,
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[allow(clippy::cast_precision_loss)]
fn value_to_pixel_x(value: f64, min: f64, max: f64, pixel_min: f32, pixel_max: f32) -> f32 {
    let range = max - min;
    let pixel_range = pixel_max - pixel_min;
    let normalized = (value - min) / range;
    pixel_min + normalized as f32 * pixel_range
}

#[allow(clippy::cast_precision_loss)]
fn value_to_pixel_y(value: f64, min: f64, max: f64, pixel_min: f32, pixel_max: f32) -> f32 {
    let range = max - min;
    let pixel_range = pixel_max - pixel_min;
    let normalized = (value - min) / range;
    pixel_max - normalized as f32 * pixel_range // Flip for screen coordinates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Series;

    #[test]
    fn test_bar_creation() {
        let data = Series::from_tuples(&[(1.0, 10.0), (2.0, 20.0), (3.0, 15.0)]);
        let bar = BarPlot::new(data).color(Color::RED).bar_width(0.5);

        assert_eq!(bar.bar_width, 0.5);
        assert_eq!(bar.color, Color::RED);
    }

    #[test]
    fn test_bar_bounds_includes_zero() {
        let data = Series::from_tuples(&[(1.0, 5.0), (2.0, 10.0)]);
        let bar = BarPlot::new(data);

        let bounds = bar.bounds().unwrap();
        assert_eq!(bounds.y_min, 0.0); // Should include zero baseline
        assert_eq!(bounds.y_max, 10.0);
    }

    #[test]
    fn test_horizontal_orientation() {
        let data = Series::from_tuples(&[(1.0, 10.0), (2.0, 20.0)]);
        let bar = BarPlot::new(data).orientation(BarOrientation::Horizontal);

        assert_eq!(bar.orientation, BarOrientation::Horizontal);
    }

    #[test]
    fn test_bar_width_clamping() {
        let data = Series::from_tuples(&[(1.0, 10.0)]);
        let bar = BarPlot::new(data).bar_width(5.0);

        assert_eq!(bar.bar_width, 2.0); // Should clamp to max 2.0
    }
}
