//! Axis rendering and configuration

mod ticks;

pub use ticks::{Tick, TickGenerator};

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable};
use crate::error::Result;

/// Axis position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisPosition {
    /// Bottom axis (X axis)
    Bottom,
    /// Left axis (Y axis)
    Left,
    /// Top axis
    Top,
    /// Right axis
    Right,
}

/// Label alignment along the axis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelAlignment {
    /// Label at the start of the axis
    Start,
    /// Label centered on the axis (default)
    Center,
    /// Label at the end of the axis
    End,
}

/// Axis configuration and rendering
pub struct Axis {
    position: AxisPosition,
    label: Option<String>,
    label_alignment: LabelAlignment,
    label_rotation: f32, // Rotation angle in degrees (0 = horizontal, 90 = vertical)
    color: Color,
    tick_color: Color,
    show_ticks: bool,
    show_grid: bool,
    tick_length: f32,
    tick_count: usize,
}

impl Axis {
    /// Create a new axis at the given position
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::axes::{Axis, AxisPosition};
    /// let x_axis = Axis::new(AxisPosition::Bottom)
    ///     .label("Time (s)")
    ///     .tick_count(10);
    /// ```
    #[must_use]
    pub fn new(position: AxisPosition) -> Self {
        // Default rotation: X axes horizontal (0°), Y axes vertical (90°)
        let default_rotation = match position {
            AxisPosition::Left | AxisPosition::Right => 90.0,
            AxisPosition::Bottom | AxisPosition::Top => 0.0,
        };

        Self {
            position,
            label: None,
            label_alignment: LabelAlignment::Center,
            label_rotation: default_rotation,
            color: Color::BLACK,
            tick_color: Color::BLACK,
            show_ticks: true,
            show_grid: false,
            tick_length: 6.0,
            tick_count: 5,
        }
    }

    /// Set the axis label
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the label alignment (Start, Center, or End)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::axes::{Axis, AxisPosition, LabelAlignment};
    /// let axis = Axis::new(AxisPosition::Bottom)
    ///     .label("X Axis")
    ///     .label_alignment(LabelAlignment::End);
    /// ```
    #[must_use]
    pub fn label_alignment(mut self, alignment: LabelAlignment) -> Self {
        self.label_alignment = alignment;
        self
    }

    /// Set the label rotation angle in degrees
    ///
    /// - 0° = horizontal (left to right)
    /// - 90° = vertical (bottom to top) - default for Y axes
    /// - Other angles are not currently supported but may be in the future
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::axes::{Axis, AxisPosition};
    /// // Make Y-axis label horizontal instead of vertical
    /// let y_axis = Axis::new(AxisPosition::Left)
    ///     .label("Y Axis")
    ///     .label_rotation(0.0);
    ///
    /// // Make X-axis label vertical
    /// let x_axis = Axis::new(AxisPosition::Bottom)
    ///     .label("X Axis")
    ///     .label_rotation(90.0);
    /// ```
    #[must_use]
    pub fn label_rotation(mut self, angle: f32) -> Self {
        self.label_rotation = angle;
        self
    }

    /// Set the axis color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show ticks
    #[must_use]
    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    /// Set whether to show grid lines
    #[must_use]
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set the tick length in pixels
    #[must_use]
    pub fn tick_length(mut self, length: f32) -> Self {
        self.tick_length = length;
        self
    }

    /// Set the approximate number of ticks
    #[must_use]
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }

    /// Get the position
    #[must_use]
    pub fn position(&self) -> AxisPosition {
        self.position
    }

    /// Render the axis on a canvas
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails
    pub fn render(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let bounds = canvas.bounds();
        let (width, height) = canvas.dimensions();

        match self.position {
            AxisPosition::Bottom => self.render_bottom(canvas, bounds, width, height),
            AxisPosition::Left => self.render_left(canvas, bounds, width, height),
            AxisPosition::Top => self.render_top(canvas, bounds, width, height),
            AxisPosition::Right => self.render_right(canvas, bounds, width, height),
        }
    }

    fn render_bottom(
        &self,
        canvas: &mut dyn Canvas,
        bounds: Bounds,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let margin_left = 60.0;
        let margin_right = 20.0;
        let margin_bottom = 40.0;

        // Draw axis line
        let y_pos = height as f32 - margin_bottom;
        canvas.draw_line_pixels(
            margin_left,
            y_pos,
            width as f32 - margin_right,
            y_pos,
            &self.color.to_rgba(),
            1.5,
        )?;

        // Draw ticks
        if self.show_ticks {
            let ticks = TickGenerator::linear(bounds.x_min, bounds.x_max, self.tick_count);

            for tick in &ticks {
                let x_pixel = self.value_to_pixel(
                    tick.value,
                    bounds.x_min,
                    bounds.x_max,
                    margin_left,
                    width as f32 - margin_right,
                );

                // Draw tick mark
                canvas.draw_line_pixels(
                    x_pixel,
                    y_pos,
                    x_pixel,
                    y_pos + self.tick_length,
                    &self.tick_color.to_rgba(),
                    1.0,
                )?;

                // Draw tick label (centered horizontally with tick)
                let label = format!("{:.1}", tick.value);
                // Text is approximately 40 pixels wide, so offset by -20 to center
                let text_x = x_pixel - 20.0;
                canvas.draw_text_pixels(
                    &label,
                    text_x,
                    y_pos + 15.0,
                    12.0,
                    &self.color.to_rgba(),
                )?;

                // Draw grid line if enabled
                if self.show_grid {
                    let grid_color = Color::rgba(200, 200, 200, 255);
                    canvas.draw_line_pixels(
                        x_pixel,
                        margin_bottom,
                        x_pixel,
                        height as f32 - margin_bottom,
                        &grid_color.to_rgba(),
                        0.5,
                    )?;
                }
            }
        }

        // Draw axis label with configurable alignment
        if let Some(ref label) = self.label {
            // Tick labels are at y_pos + 15, which is (height - margin_bottom) + 15
            // Tick label text size is 12px, so they end at y_pos + 15 + 12 = y_pos + 27
            // Add 8px spacing below tick labels, then 14px for the axis label text
            // Total: (height - margin_bottom) + 27 + 8 = (height - 40) + 35 = height - 5
            // This is too close to edge. Need at least 14px for text + 5px margin = 19px from bottom
            // So: label_y = height - 19
            let label_y = height as f32 - 19.0; // 14px text height + 5px margin from bottom
            let text_width_estimate = label.len() as f32 * 8.0; // Approximate text width

            let label_x = match self.label_alignment {
                LabelAlignment::Start => margin_left,
                LabelAlignment::Center => {
                    (margin_left + width as f32 - margin_right) / 2.0 - text_width_estimate / 2.0
                }
                LabelAlignment::End => width as f32 - margin_right - text_width_estimate,
            };
            canvas.draw_text_pixels(label, label_x, label_y, 14.0, &self.color.to_rgba())?;
        }

        Ok(())
    }

    fn render_left(
        &self,
        canvas: &mut dyn Canvas,
        bounds: Bounds,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let margin_left = 60.0;
        let margin_top = 40.0;
        let margin_bottom = 40.0;

        // Draw axis line
        canvas.draw_line_pixels(
            margin_left,
            margin_top,
            margin_left,
            height as f32 - margin_bottom,
            &self.color.to_rgba(),
            1.5,
        )?;

        // Draw ticks
        if self.show_ticks {
            let ticks = TickGenerator::linear(bounds.y_min, bounds.y_max, self.tick_count);

            for tick in &ticks {
                let y_pixel = self.value_to_pixel_y(
                    tick.value,
                    bounds.y_min,
                    bounds.y_max,
                    margin_top,
                    height as f32 - margin_bottom,
                );

                // Draw tick mark
                canvas.draw_line_pixels(
                    margin_left - self.tick_length,
                    y_pixel,
                    margin_left,
                    y_pixel,
                    &self.tick_color.to_rgba(),
                    1.0,
                )?;

                // Draw tick label (centered vertically with tick)
                let label = format!("{:.1}", tick.value);
                let text_size = 12.0;
                // Use same vertical alignment as legend (0.70 of text height)
                // This centers the text with the tick line
                let text_y = y_pixel - (text_size * 0.70);
                canvas.draw_text_pixels(
                    &label,
                    margin_left - 45.0,
                    text_y,
                    text_size,
                    &self.color.to_rgba(),
                )?;

                // Draw grid line if enabled
                if self.show_grid {
                    let grid_color = Color::rgba(200, 200, 200, 255);
                    canvas.draw_line_pixels(
                        margin_left,
                        y_pixel,
                        width as f32 - 20.0,
                        y_pixel,
                        &grid_color.to_rgba(),
                        0.5,
                    )?;
                }
            }
        }

        // Draw axis label with rotation support
        if let Some(ref label) = self.label {
            let text_size = 14.0;

            if self.label_rotation.abs() < 1.0 {
                // Horizontal text (0 degrees)
                let label_y = match self.label_alignment {
                    LabelAlignment::Start => height as f32 - margin_bottom,
                    LabelAlignment::Center => (margin_top + height as f32 - margin_bottom) / 2.0,
                    LabelAlignment::End => margin_top,
                };
                canvas.draw_text_pixels(label, 10.0, label_y, text_size, &self.color.to_rgba())?;
            } else if (self.label_rotation - 90.0).abs() < 1.0 {
                // Vertical text (90 degrees) - render character by character from bottom to top
                let char_height = text_size + 2.0; // Add small spacing between characters
                let total_height = label.len() as f32 * char_height;

                let start_y = match self.label_alignment {
                    LabelAlignment::Start => height as f32 - margin_bottom,
                    LabelAlignment::Center => {
                        (margin_top + height as f32 - margin_bottom) / 2.0 + total_height / 2.0
                    }
                    LabelAlignment::End => margin_top + total_height,
                };

                // Draw each character vertically, from top to bottom on screen (reading bottom to top)
                for (i, ch) in label.chars().enumerate() {
                    let char_y = start_y - (i as f32 * char_height);
                    let char_str = ch.to_string();
                    canvas.draw_text_pixels(
                        &char_str,
                        10.0,
                        char_y,
                        text_size,
                        &self.color.to_rgba(),
                    )?;
                }
            } else {
                // Other angles not supported yet - just draw horizontally
                canvas.draw_text_pixels(
                    label,
                    10.0,
                    (margin_top + height as f32 - margin_bottom) / 2.0,
                    text_size,
                    &self.color.to_rgba(),
                )?;
            }
        }

        Ok(())
    }

    fn render_top(
        &self,
        _canvas: &mut dyn Canvas,
        _bounds: Bounds,
        _width: u32,
        _height: u32,
    ) -> Result<()> {
        // Top axis implementation (similar to bottom)
        Ok(())
    }

    fn render_right(
        &self,
        _canvas: &mut dyn Canvas,
        _bounds: Bounds,
        _width: u32,
        _height: u32,
    ) -> Result<()> {
        // Right axis implementation (similar to left)
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    fn value_to_pixel(
        &self,
        value: f64,
        min: f64,
        max: f64,
        pixel_min: f32,
        pixel_max: f32,
    ) -> f32 {
        let range = max - min;
        let pixel_range = pixel_max - pixel_min;
        let normalized = (value - min) / range;
        pixel_min + normalized as f32 * pixel_range
    }

    #[allow(clippy::cast_precision_loss)]
    fn value_to_pixel_y(
        &self,
        value: f64,
        min: f64,
        max: f64,
        pixel_min: f32,
        pixel_max: f32,
    ) -> f32 {
        let range = max - min;
        let pixel_range = pixel_max - pixel_min;
        let normalized = (value - min) / range;
        pixel_max - normalized as f32 * pixel_range // Flip for screen coordinates
    }
}

impl Drawable for Axis {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        self.render(canvas)
    }
}
