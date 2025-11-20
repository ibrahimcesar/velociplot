//! Scatter plot implementation with configurable markers

use crate::color::Color;
use crate::core::{Bounds, Canvas, DataSeries, Drawable};
use crate::error::Result;
use crate::legend::LegendEntry;

/// Marker shape for scatter plots
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerShape {
    /// Circular marker (default)
    Circle,
    /// Square marker
    Square,
    /// Triangle marker (pointing up)
    Triangle,
    /// Diamond marker (rotated square)
    Diamond,
    /// Plus sign (+)
    Plus,
    /// Cross sign (×)
    Cross,
}

/// Marker style for scatter plots
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerStyle {
    /// Filled marker
    Filled,
    /// Outlined marker only
    Outline,
}

/// Scatter plot with configurable markers
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::plots::scatter::{ScatterPlot, MarkerShape, MarkerStyle};
/// let data = Series::from_function(0.0, 10.0, 20, |x| x * x);
///
/// let scatter = ScatterPlot::new(data)
///     .marker_shape(MarkerShape::Circle)
///     .marker_size(6.0)
///     .color(Color::from_hex("#e74c3c").unwrap())
///     .label("Data points");
/// ```
pub struct ScatterPlot {
    data: Box<dyn DataSeries>,
    color: Color,
    marker_shape: MarkerShape,
    marker_size: f32,
    marker_style: MarkerStyle,
    outline_width: f32,
    label: Option<String>,
}

impl ScatterPlot {
    /// Create a new scatter plot from a data series
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// # use velociplot::plots::scatter::ScatterPlot;
    /// let data = Series::from_function(0.0, 5.0, 10, |x| x.sin());
    /// let scatter = ScatterPlot::new(data);
    /// ```
    #[must_use]
    pub fn new(data: impl DataSeries + 'static) -> Self {
        Self {
            data: Box::new(data),
            color: Color::BLUE,
            marker_shape: MarkerShape::Circle,
            marker_size: 5.0,
            marker_style: MarkerStyle::Filled,
            outline_width: 1.5,
            label: None,
        }
    }

    /// Set the marker color
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

    /// Set the marker size (radius in pixels)
    #[must_use]
    pub fn marker_size(mut self, size: f32) -> Self {
        self.marker_size = size;
        self
    }

    /// Set the marker style (filled or outline)
    #[must_use]
    pub fn marker_style(mut self, style: MarkerStyle) -> Self {
        self.marker_style = style;
        self
    }

    /// Set the outline width (for outline markers)
    #[must_use]
    pub fn outline_width(mut self, width: f32) -> Self {
        self.outline_width = width;
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
        Some(Bounds::from_points(points))
    }

    /// Create a legend entry for this scatter plot
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label.as_ref().map(|label| {
            LegendEntry::new(label)
                .color(self.color)
                .line_width(self.outline_width)
        })
    }

    /// Draw a marker at the given pixel coordinates
    fn draw_marker(&self, canvas: &mut dyn Canvas, x: f32, y: f32) -> Result<()> {
        let color = self.color.to_rgba();
        let size = self.marker_size;

        match self.marker_shape {
            MarkerShape::Circle => {
                match self.marker_style {
                    MarkerStyle::Filled => {
                        // Draw filled circle by drawing concentric circles
                        let steps = (size * 2.0).ceil() as i32;
                        for i in 0..steps {
                            let radius = size * (1.0 - i as f32 / steps as f32);
                            if radius > 0.0 {
                                self.draw_circle_outline(canvas, x, y, radius, &color, 1.0)?;
                            }
                        }
                    }
                    MarkerStyle::Outline => {
                        self.draw_circle_outline(canvas, x, y, size, &color, self.outline_width)?;
                    }
                }
            }
            MarkerShape::Square => {
                match self.marker_style {
                    MarkerStyle::Filled => {
                        // Draw filled square
                        for dy in -(size as i32)..=(size as i32) {
                            canvas.draw_line_pixels(
                                x - size,
                                y + dy as f32,
                                x + size,
                                y + dy as f32,
                                &color,
                                1.0,
                            )?;
                        }
                    }
                    MarkerStyle::Outline => {
                        // Draw square outline
                        // Top
                        canvas.draw_line_pixels(
                            x - size,
                            y - size,
                            x + size,
                            y - size,
                            &color,
                            self.outline_width,
                        )?;
                        // Right
                        canvas.draw_line_pixels(
                            x + size,
                            y - size,
                            x + size,
                            y + size,
                            &color,
                            self.outline_width,
                        )?;
                        // Bottom
                        canvas.draw_line_pixels(
                            x + size,
                            y + size,
                            x - size,
                            y + size,
                            &color,
                            self.outline_width,
                        )?;
                        // Left
                        canvas.draw_line_pixels(
                            x - size,
                            y + size,
                            x - size,
                            y - size,
                            &color,
                            self.outline_width,
                        )?;
                    }
                }
            }
            MarkerShape::Triangle => {
                let height = size * 1.732; // sqrt(3) for equilateral triangle
                let half_base = size;

                let top_y = y - height * 0.666;
                let bottom_y = y + height * 0.333;

                match self.marker_style {
                    MarkerStyle::Filled => {
                        // Fill triangle with horizontal lines
                        let steps = (height as i32).max(1);
                        for i in 0..=steps {
                            let ratio = i as f32 / steps as f32;
                            let current_y = top_y + ratio * height;
                            let current_half_width = half_base * ratio;

                            canvas.draw_line_pixels(
                                x - current_half_width,
                                current_y,
                                x + current_half_width,
                                current_y,
                                &color,
                                1.0,
                            )?;
                        }
                    }
                    MarkerStyle::Outline => {
                        // Draw triangle outline
                        // Left edge
                        canvas.draw_line_pixels(
                            x,
                            top_y,
                            x - half_base,
                            bottom_y,
                            &color,
                            self.outline_width,
                        )?;
                        // Right edge
                        canvas.draw_line_pixels(
                            x,
                            top_y,
                            x + half_base,
                            bottom_y,
                            &color,
                            self.outline_width,
                        )?;
                        // Bottom edge
                        canvas.draw_line_pixels(
                            x - half_base,
                            bottom_y,
                            x + half_base,
                            bottom_y,
                            &color,
                            self.outline_width,
                        )?;
                    }
                }
            }
            MarkerShape::Diamond => {
                match self.marker_style {
                    MarkerStyle::Filled => {
                        // Fill diamond with horizontal lines
                        let steps = (size as i32).max(1);

                        // Top half
                        for i in 0..=steps {
                            let ratio = i as f32 / steps as f32;
                            let current_y = y - size + i as f32;
                            let half_width = size * ratio;

                            canvas.draw_line_pixels(
                                x - half_width,
                                current_y,
                                x + half_width,
                                current_y,
                                &color,
                                1.0,
                            )?;
                        }

                        // Bottom half
                        for i in 0..steps {
                            let ratio = 1.0 - i as f32 / steps as f32;
                            let current_y = y + i as f32;
                            let half_width = size * ratio;

                            canvas.draw_line_pixels(
                                x - half_width,
                                current_y,
                                x + half_width,
                                current_y,
                                &color,
                                1.0,
                            )?;
                        }
                    }
                    MarkerStyle::Outline => {
                        // Draw diamond outline
                        // Top left
                        canvas.draw_line_pixels(
                            x,
                            y - size,
                            x - size,
                            y,
                            &color,
                            self.outline_width,
                        )?;
                        // Top right
                        canvas.draw_line_pixels(
                            x,
                            y - size,
                            x + size,
                            y,
                            &color,
                            self.outline_width,
                        )?;
                        // Bottom right
                        canvas.draw_line_pixels(
                            x + size,
                            y,
                            x,
                            y + size,
                            &color,
                            self.outline_width,
                        )?;
                        // Bottom left
                        canvas.draw_line_pixels(
                            x,
                            y + size,
                            x - size,
                            y,
                            &color,
                            self.outline_width,
                        )?;
                    }
                }
            }
            MarkerShape::Plus => {
                // Horizontal line
                canvas.draw_line_pixels(x - size, y, x + size, y, &color, self.outline_width)?;
                // Vertical line
                canvas.draw_line_pixels(x, y - size, x, y + size, &color, self.outline_width)?;
            }
            MarkerShape::Cross => {
                let diag = size * 0.707; // 1/sqrt(2) for 45-degree lines
                                         // Diagonal from top-left to bottom-right
                canvas.draw_line_pixels(
                    x - diag,
                    y - diag,
                    x + diag,
                    y + diag,
                    &color,
                    self.outline_width,
                )?;
                // Diagonal from top-right to bottom-left
                canvas.draw_line_pixels(
                    x + diag,
                    y - diag,
                    x - diag,
                    y + diag,
                    &color,
                    self.outline_width,
                )?;
            }
        }

        Ok(())
    }

    /// Helper to draw a circle outline using line segments
    fn draw_circle_outline(
        &self,
        canvas: &mut dyn Canvas,
        cx: f32,
        cy: f32,
        radius: f32,
        color: &[u8; 4],
        width: f32,
    ) -> Result<()> {
        let segments = (radius * 8.0).max(12.0) as i32; // More segments for larger circles
        let angle_step = 2.0 * std::f32::consts::PI / segments as f32;

        for i in 0..segments {
            let angle1 = i as f32 * angle_step;
            let angle2 = (i + 1) as f32 * angle_step;

            let x1 = cx + radius * angle1.cos();
            let y1 = cy + radius * angle1.sin();
            let x2 = cx + radius * angle2.cos();
            let y2 = cy + radius * angle2.sin();

            canvas.draw_line_pixels(x1, y1, x2, y2, color, width)?;
        }

        Ok(())
    }
}

impl Drawable for ScatterPlot {
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

        // Draw markers for each data point
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

            self.draw_marker(canvas, x_pixel, y_pixel)?;
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
    fn test_scatter_creation() {
        let data = Series::from_function(0.0, 10.0, 5, |x| x * x);
        let scatter = ScatterPlot::new(data).color(Color::RED).marker_size(8.0);

        assert_eq!(scatter.marker_size, 8.0);
        assert_eq!(scatter.color, Color::RED);
    }

    #[test]
    fn test_scatter_bounds() {
        let data = Series::from_function(0.0, 10.0, 5, |x| x);
        let scatter = ScatterPlot::new(data);

        let bounds = scatter.bounds().unwrap();
        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 10.0);
    }

    #[test]
    fn test_marker_shapes() {
        let data = Series::from_function(0.0, 5.0, 3, |x| x);

        for shape in [
            MarkerShape::Circle,
            MarkerShape::Square,
            MarkerShape::Triangle,
            MarkerShape::Diamond,
            MarkerShape::Plus,
            MarkerShape::Cross,
        ] {
            let scatter = ScatterPlot::new(data.clone()).marker_shape(shape);
            assert_eq!(scatter.marker_shape, shape);
        }
    }

    #[test]
    fn test_legend_entry() {
        let data = Series::from_function(0.0, 5.0, 3, |x| x);
        let scatter = ScatterPlot::new(data).label("Test Data");

        let entry = scatter.legend_entry();
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().label, "Test Data");
    }
}
