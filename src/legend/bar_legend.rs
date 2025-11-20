//! Color bar legend for heatmaps and gradient visualizations
//!
//! `BarLegend` displays a continuous color gradient with labeled values,
//! commonly used to show the mapping between colors and data values in heatmaps.

use crate::color::{Color, Colormap};
use crate::core::{Canvas, Drawable};
use crate::error::Result;
use crate::plots::bar::BarOrientation;

/// Position for the color bar legend
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarLegendPosition {
    /// Right side of the plot (vertical bar)
    Right,
    /// Bottom of the plot (horizontal bar)
    Bottom,
    /// Custom position (x, y in pixels from top-left)
    Custom(i32, i32),
}

/// Color bar legend for gradient/heatmap visualizations
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::legend::bar_legend::BarLegend;
/// let bar_legend = BarLegend::new(0.0, 100.0, Colormap::viridis())
///     .label("Temperature (°C)");
/// ```
pub struct BarLegend {
    min_value: f64,
    max_value: f64,
    colormap: Colormap,
    label: Option<String>,
    position: BarLegendPosition,
    orientation: BarOrientation,
    width: f32,
    height: f32,
    num_ticks: usize,
    show_ticks: bool,
    show_values: bool,
    text_color: Color,
    border_color: Color,
    show_border: bool,
}

impl BarLegend {
    /// Create a new color bar legend
    ///
    /// # Arguments
    ///
    /// * `min_value` - Minimum value for the color scale
    /// * `max_value` - Maximum value for the color scale
    /// * `colormap` - Colormap to display
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// # use velociplot::legend::bar_legend::BarLegend;
    /// let bar = BarLegend::new(0.0, 100.0, Colormap::viridis());
    /// ```
    #[must_use]
    pub fn new(min_value: f64, max_value: f64, colormap: Colormap) -> Self {
        Self {
            min_value,
            max_value,
            colormap,
            label: None,
            position: BarLegendPosition::Right,
            orientation: BarOrientation::Vertical,
            width: 30.0,
            height: 200.0,
            num_ticks: 5,
            show_ticks: true,
            show_values: true,
            text_color: Color::BLACK,
            border_color: Color::BLACK,
            show_border: true,
        }
    }

    /// Set the label for the color bar
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the position of the color bar
    #[must_use]
    pub fn position(mut self, position: BarLegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the orientation of the color bar
    #[must_use]
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the dimensions (width and height for vertical, swapped for horizontal)
    #[must_use]
    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        self.width = width.max(10.0);
        self.height = height.max(50.0);
        self
    }

    /// Set the number of tick marks
    #[must_use]
    pub fn num_ticks(mut self, num: usize) -> Self {
        self.num_ticks = num.max(2);
        self
    }

    /// Set whether to show tick marks
    #[must_use]
    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    /// Set whether to show values
    #[must_use]
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set whether to show border
    #[must_use]
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Calculate the position based on canvas dimensions
    #[allow(clippy::cast_precision_loss)]
    fn calculate_position(&self, canvas: &dyn Canvas) -> (f32, f32) {
        let (canvas_width, canvas_height) = canvas.dimensions();
        let margin = 20.0;

        match self.position {
            BarLegendPosition::Right => {
                let x = canvas_width as f32 - self.width - margin - 80.0; // Extra space for labels
                let y = (canvas_height as f32 - self.height) / 2.0;
                (x, y)
            }
            BarLegendPosition::Bottom => {
                let x = (canvas_width as f32 - self.height) / 2.0; // height becomes width for horizontal
                let y = canvas_height as f32 - self.width - margin - 60.0;
                (x, y)
            }
            BarLegendPosition::Custom(x, y) => (x as f32, y as f32),
        }
    }

    /// Draw the color gradient
    fn draw_gradient(&self, canvas: &mut dyn Canvas, x: f32, y: f32) -> Result<()> {
        match self.orientation {
            BarOrientation::Vertical => {
                // Draw vertical gradient from bottom (min) to top (max)
                let steps = self.height as i32;
                for i in 0..steps {
                    let t = 1.0 - (i as f32 / steps as f32); // Invert so max is at top
                    let color = self.colormap.get(t);
                    let y_pos = y + i as f32;

                    canvas.draw_line_pixels(
                        x,
                        y_pos,
                        x + self.width,
                        y_pos,
                        &color.to_rgba(),
                        1.0,
                    )?;
                }
            }
            BarOrientation::Horizontal => {
                // Draw horizontal gradient from left (min) to right (max)
                let steps = self.height as i32; // height is the length for horizontal bars
                for i in 0..steps {
                    let t = i as f32 / steps as f32;
                    let color = self.colormap.get(t);
                    let x_pos = x + i as f32;

                    canvas.draw_line_pixels(
                        x_pos,
                        y,
                        x_pos,
                        y + self.width, // width is the thickness for horizontal bars
                        &color.to_rgba(),
                        1.0,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Draw border around the color bar
    fn draw_border(&self, canvas: &mut dyn Canvas, x: f32, y: f32) -> Result<()> {
        if !self.show_border {
            return Ok(());
        }

        let (bar_width, bar_height) = match self.orientation {
            BarOrientation::Vertical => (self.width, self.height),
            BarOrientation::Horizontal => (self.height, self.width),
        };

        // Top
        canvas.draw_line_pixels(x, y, x + bar_width, y, &self.border_color.to_rgba(), 1.0)?;
        // Right
        canvas.draw_line_pixels(
            x + bar_width,
            y,
            x + bar_width,
            y + bar_height,
            &self.border_color.to_rgba(),
            1.0,
        )?;
        // Bottom
        canvas.draw_line_pixels(
            x,
            y + bar_height,
            x + bar_width,
            y + bar_height,
            &self.border_color.to_rgba(),
            1.0,
        )?;
        // Left
        canvas.draw_line_pixels(x, y, x, y + bar_height, &self.border_color.to_rgba(), 1.0)?;

        Ok(())
    }

    /// Draw ticks and values
    fn draw_ticks(&self, canvas: &mut dyn Canvas, x: f32, y: f32) -> Result<()> {
        if !self.show_ticks && !self.show_values {
            return Ok(());
        }

        for i in 0..self.num_ticks {
            let t = i as f64 / (self.num_ticks - 1) as f64;
            let value = self.min_value + t * (self.max_value - self.min_value);

            match self.orientation {
                BarOrientation::Vertical => {
                    // Position from bottom to top (invert t for display)
                    let tick_y = y + self.height - (t as f32 * self.height);

                    if self.show_ticks {
                        // Draw tick mark
                        canvas.draw_line_pixels(
                            x + self.width,
                            tick_y,
                            x + self.width + 5.0,
                            tick_y,
                            &self.text_color.to_rgba(),
                            1.0,
                        )?;
                    }

                    if self.show_values {
                        // Draw value label
                        let value_str = format!("{value:.1}");
                        canvas.draw_text_pixels(
                            &value_str,
                            x + self.width + 8.0,
                            tick_y - 5.0, // Center text vertically on tick
                            10.0,
                            &self.text_color.to_rgba(),
                        )?;
                    }
                }
                BarOrientation::Horizontal => {
                    let tick_x = x + (t as f32 * self.height);

                    if self.show_ticks {
                        // Draw tick mark
                        canvas.draw_line_pixels(
                            tick_x,
                            y + self.width,
                            tick_x,
                            y + self.width + 5.0,
                            &self.text_color.to_rgba(),
                            1.0,
                        )?;
                    }

                    if self.show_values {
                        // Draw value label
                        let value_str = format!("{value:.1}");
                        canvas.draw_text_pixels(
                            &value_str,
                            tick_x - 10.0, // Center text horizontally on tick
                            y + self.width + 15.0,
                            10.0,
                            &self.text_color.to_rgba(),
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Draw the legend label
    fn draw_label(&self, canvas: &mut dyn Canvas, x: f32, y: f32) -> Result<()> {
        if let Some(label) = &self.label {
            match self.orientation {
                BarOrientation::Vertical => {
                    // Draw label above the bar
                    canvas.draw_text_pixels(
                        label,
                        x,
                        y - 15.0,
                        12.0,
                        &self.text_color.to_rgba(),
                    )?;
                }
                BarOrientation::Horizontal => {
                    // Draw label to the left of the bar
                    canvas.draw_text_pixels(
                        label,
                        x - 80.0,
                        y + self.width / 2.0,
                        12.0,
                        &self.text_color.to_rgba(),
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Drawable for BarLegend {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let (x, y) = self.calculate_position(canvas);

        // Draw gradient
        self.draw_gradient(canvas, x, y)?;

        // Draw border
        self.draw_border(canvas, x, y)?;

        // Draw ticks and values
        self.draw_ticks(canvas, x, y)?;

        // Draw label
        self.draw_label(canvas, x, y)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_legend_creation() {
        let bar = BarLegend::new(0.0, 100.0, Colormap::viridis());
        assert_eq!(bar.min_value, 0.0);
        assert_eq!(bar.max_value, 100.0);
        assert_eq!(bar.num_ticks, 5);
    }

    #[test]
    fn test_bar_legend_builder() {
        let bar = BarLegend::new(0.0, 100.0, Colormap::plasma())
            .label("Test")
            .dimensions(40.0, 250.0)
            .num_ticks(7)
            .show_border(false);

        assert!(bar.label.is_some());
        assert_eq!(bar.width, 40.0);
        assert_eq!(bar.height, 250.0);
        assert_eq!(bar.num_ticks, 7);
        assert!(!bar.show_border);
    }

    #[test]
    fn test_bar_legend_min_dimensions() {
        let bar = BarLegend::new(0.0, 100.0, Colormap::viridis()).dimensions(5.0, 30.0); // Too small

        assert_eq!(bar.width, 10.0); // Clamped to minimum
        assert_eq!(bar.height, 50.0); // Clamped to minimum
    }
}
