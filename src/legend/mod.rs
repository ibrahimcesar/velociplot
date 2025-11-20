//! Legend rendering for plots

pub mod bar_legend;

use crate::color::Color;
use crate::core::{Canvas, Drawable};
use crate::error::Result;

pub use bar_legend::{BarLegend, BarLegendPosition};

/// Legend entry shape
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LegendShape {
    /// Line (default)
    Line,
    /// Box/Rectangle
    Box,
    /// Point/Marker with specific shape
    Point(MarkerShape, f32), // (shape, size)
    /// Swatch (filled rectangle, larger than box)
    Swatch,
}

/// Marker shape for point legends
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarkerShape {
    /// Circular marker
    Circle,
    /// Square marker
    Square,
    /// Triangle marker
    Triangle,
    /// Diamond marker
    Diamond,
    /// Plus sign (+)
    Plus,
    /// Cross sign (×)
    Cross,
}

/// Legend entry with label and style
#[derive(Debug, Clone)]
pub struct LegendEntry {
    /// Label text
    pub label: String,
    /// Line/marker color
    pub color: Color,
    /// Line width (if applicable)
    pub line_width: f32,
    /// Shape to display
    pub shape: LegendShape,
}

impl LegendEntry {
    /// Create a new legend entry
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: Color::BLUE,
            line_width: 2.0,
            shape: LegendShape::Line,
        }
    }

    /// Set color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set line width
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set shape to box
    #[must_use]
    pub fn box_shape(mut self) -> Self {
        self.shape = LegendShape::Box;
        self
    }

    /// Set shape to point/marker
    #[must_use]
    pub fn point_shape(mut self, marker: MarkerShape, size: f32) -> Self {
        self.shape = LegendShape::Point(marker, size);
        self
    }

    /// Set shape to swatch (filled rectangle)
    #[must_use]
    pub fn swatch_shape(mut self) -> Self {
        self.shape = LegendShape::Swatch;
        self
    }
}

/// Horizontal alignment for legend placement
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HorizontalAlignment {
    /// Align to the left/start
    Start,
    /// Center alignment
    Center,
    /// Align to the right/end
    End,
}

/// Legend position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LegendPosition {
    /// Upper right corner
    UpperRight,
    /// Upper left corner
    UpperLeft,
    /// Lower right corner
    LowerRight,
    /// Lower left corner
    LowerLeft,
    /// Center right
    CenterRight,
    /// Below the graph - **Default for publications**
    /// This expands the canvas vertically to accommodate the legend
    Below(HorizontalAlignment),
    /// Custom position (x, y in pixels from top-left)
    Custom(i32, i32),
    /// Auto-position in the corner with least plot data density
    Auto,
}

/// Legend configuration and rendering
pub struct Legend {
    entries: Vec<LegendEntry>,
    position: LegendPosition,
    background: Color,
    border_color: Color,
    text_color: Color,
    show_border: bool,
    padding: f32,
    line_sample_length: f32,
    /// Vertical alignment factor for line samples (0.0 = top of text, 1.0 = bottom)
    /// Default is 0.70 which aligns well with `JetBrains` Mono (the embedded font)
    line_vertical_align: f32,
}

impl Legend {
    /// Create a new empty legend
    ///
    /// # Examples
    ///
    /// ```
    /// use velociplot::legend::{Legend, LegendPosition};
    ///
    /// let legend = Legend::new()
    ///     .position(LegendPosition::UpperRight);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            position: LegendPosition::Below(HorizontalAlignment::Center), // Default: below graph centered (publication standard)
            background: Color::rgba(255, 255, 255, 230),
            border_color: Color::BLACK,
            text_color: Color::BLACK,
            show_border: true,
            padding: 10.0,
            line_sample_length: 30.0,
            line_vertical_align: 0.70, // Default: optimized for JetBrains Mono
        }
    }

    /// Add an entry to the legend
    #[must_use]
    pub fn add_entry(mut self, entry: LegendEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Set the legend position
    #[must_use]
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the background color
    #[must_use]
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    /// Set whether to show the border
    #[must_use]
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set the vertical alignment factor for line samples
    ///
    /// Controls where the colored line sample appears relative to the text height.
    /// Different fonts have different proportions, so the optimal alignment varies.
    ///
    /// # Typography Background
    ///
    /// Text has several vertical reference points:
    /// - **Top** (0.0): Top of tallest letters (like 'h', 'k')
    /// - **x-height**: Height of lowercase 'x' (typically 50-60% of font size)
    /// - **Baseline** (~0.75): Where letters sit (mathematical horizontal)
    /// - **Bottom** (1.0): Bottom of descenders (like 'g', 'y')
    ///
    /// The visual/optical center is usually around the middle of x-height (0.65-0.70),
    /// not at 50% of the text box. This is why the default is 0.70.
    ///
    /// # Values
    ///
    /// - `0.0` = top of text bounding box
    /// - `0.5` = geometric middle of text box
    /// - `0.65` = middle of x-height (geometric center)
    /// - `0.70` = **default** (optical center for `JetBrains` Mono)
    /// - `0.75` = baseline (mathematical reference)
    /// - `1.0` = bottom of text bounding box
    ///
    /// # Examples
    ///
    /// ```
    /// use velociplot::legend::Legend;
    ///
    /// // Use default (0.70 - works well with JetBrains Mono)
    /// let legend = Legend::new();
    ///
    /// // Align line with baseline (more mathematical)
    /// let legend = Legend::new().line_vertical_align(0.75);
    ///
    /// // Align higher for fonts with large x-height
    /// let legend = Legend::new().line_vertical_align(0.60);
    /// ```
    ///
    /// # Finding the Best Value
    ///
    /// Run the alignment test example to visually compare:
    /// ```bash
    /// cargo run --example legend_alignment_test
    /// ```
    /// This generates comparison plots at 0.50, 0.55, 0.60, 0.65, 0.70, and 0.75.
    #[must_use]
    pub fn line_vertical_align(mut self, align: f32) -> Self {
        self.line_vertical_align = align.clamp(0.0, 1.0);
        self
    }

    /// Check if legend has entries
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Calculate legend box dimensions
    fn calculate_box(&self) -> (f32, f32) {
        if self.entries.is_empty() {
            return (0.0, 0.0);
        }

        // Estimate text width (rough approximation: 8 pixels per character)
        let max_label_width = self
            .entries
            .iter()
            .map(|e| e.label.len() as f32 * 8.0)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(100.0);

        let width = self.padding * 2.0 + self.line_sample_length + 10.0 + max_label_width;
        let height = self.padding * 2.0 + self.entries.len() as f32 * 20.0;

        (width, height)
    }

    /// Calculate legend position in pixels
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn calculate_position(&self, canvas: &dyn Canvas) -> (f32, f32) {
        let (canvas_width, canvas_height) = canvas.dimensions();
        let (box_width, box_height) = self.calculate_box();
        let margin = 20.0;

        // Define position closures for each corner
        let upper_right = || {
            (
                canvas_width as f32 - box_width - margin - 20.0,
                margin + 40.0,
            )
        };
        let upper_left = || (margin + 60.0, margin + 40.0);
        let lower_right = || {
            (
                canvas_width as f32 - box_width - margin - 20.0,
                canvas_height as f32 - box_height - margin - 40.0,
            )
        };
        let lower_left = || {
            (
                margin + 60.0,
                canvas_height as f32 - box_height - margin - 40.0,
            )
        };

        match self.position {
            LegendPosition::UpperRight => upper_right(),
            LegendPosition::UpperLeft => upper_left(),
            LegendPosition::LowerRight => lower_right(),
            LegendPosition::LowerLeft => lower_left(),
            LegendPosition::CenterRight => (
                canvas_width as f32 - box_width - margin - 20.0,
                (canvas_height as f32 - box_height) / 2.0,
            ),
            LegendPosition::Below(alignment) => {
                // Legend below the graph area
                // Canvas has margins: left=60, top=40, right=20, bottom=40
                // For expanded canvas (e.g., 720px): plot area is y=40 to y=680 (640px)
                // Legend should go in the extra space below the plot area
                let plot_area_bottom = canvas_height as f32 - 40.0; // Where plot ends (before bottom margin)

                // Position legend to fit within canvas bounds
                // Start 25px after plot area (for axis labels), but ensure it fits
                let desired_y = plot_area_bottom + 25.0;
                let legend_y = if desired_y + box_height <= canvas_height as f32 {
                    desired_y
                } else {
                    // Not enough space, fit from bottom with 5px margin
                    (canvas_height as f32 - box_height - 5.0).max(plot_area_bottom + 5.0)
                };

                let x = match alignment {
                    HorizontalAlignment::Start => margin + 60.0, // Align to left
                    HorizontalAlignment::Center => (canvas_width as f32 - box_width) / 2.0, // Centered
                    HorizontalAlignment::End => canvas_width as f32 - box_width - margin - 20.0, // Align to right
                };
                (x, legend_y)
            }
            LegendPosition::Custom(x, y) => (x as f32, y as f32),
            LegendPosition::Auto => {
                // Calculate density in each corner and choose the one with least data
                let (ur_density, ul_density, lr_density, ll_density) =
                    canvas.calculate_corner_densities(box_width as u32, box_height as u32);

                // Find minimum density (best position)
                let min_density = ur_density.min(ul_density).min(lr_density).min(ll_density);

                // Choose the position with minimum density
                // If there's a tie, prefer UpperRight > UpperLeft > LowerRight > LowerLeft
                if (ur_density - min_density).abs() < 0.001 {
                    upper_right()
                } else if (ul_density - min_density).abs() < 0.001 {
                    upper_left()
                } else if (lr_density - min_density).abs() < 0.001 {
                    lower_right()
                } else {
                    lower_left()
                }
            }
        }
    }

    /// Render the legend
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails
    pub fn render(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.entries.is_empty() {
            return Ok(());
        }

        let (x, y) = self.calculate_position(canvas);
        let (box_width, box_height) = self.calculate_box();

        // Draw background
        self.draw_box(canvas, x, y, box_width, box_height)?;

        // Draw each entry
        let mut current_y = y + self.padding;

        for entry in &self.entries {
            let text_size = 12.0;
            let text_x = x + self.padding + self.line_sample_length + 10.0;
            let text_y = current_y;

            // Draw symbol based on shape type
            let symbol_start_x = x + self.padding;
            let symbol_center_y = text_y + (text_size * self.line_vertical_align);

            match entry.shape {
                LegendShape::Line => {
                    // Draw line sample
                    let line_end_x = symbol_start_x + self.line_sample_length;
                    canvas.draw_line_pixels(
                        symbol_start_x,
                        symbol_center_y,
                        line_end_x,
                        symbol_center_y,
                        &entry.color.to_rgba(),
                        entry.line_width,
                    )?;
                }
                LegendShape::Box => {
                    // Draw small box
                    let box_size = 10.0;
                    self.draw_legend_box(
                        canvas,
                        symbol_start_x,
                        symbol_center_y - box_size / 2.0,
                        box_size,
                        &entry.color.to_rgba(),
                    )?;
                }
                LegendShape::Point(marker, size) => {
                    // Draw marker at center of symbol area
                    let marker_x = symbol_start_x + self.line_sample_length / 2.0;
                    self.draw_marker(
                        canvas,
                        marker,
                        marker_x,
                        symbol_center_y,
                        size,
                        &entry.color.to_rgba(),
                    )?;
                }
                LegendShape::Swatch => {
                    // Draw larger filled rectangle (swatch)
                    let swatch_width = self.line_sample_length;
                    let swatch_height = 12.0;
                    self.draw_legend_box(
                        canvas,
                        symbol_start_x,
                        symbol_center_y - swatch_height / 2.0,
                        swatch_width,
                        &entry.color.to_rgba(),
                    )?;
                }
            }

            // Draw label text
            canvas.draw_text_pixels(
                &entry.label,
                text_x,
                text_y,
                text_size,
                &self.text_color.to_rgba(),
            )?;

            current_y += 20.0;
        }

        Ok(())
    }

    /// Draw a small filled box for legend (used for box shape and swatches)
    fn draw_legend_box(
        &self,
        canvas: &mut dyn Canvas,
        x: f32,
        y: f32,
        size_or_width: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        let height = if size_or_width <= 15.0 {
            size_or_width
        } else {
            12.0
        }; // If it's a swatch (width > 15), use fixed height
        let width = size_or_width;

        let steps = (height / 2.0).ceil() as i32;
        for i in 0..steps {
            let y_pos = y + i as f32 * 2.0;
            canvas.draw_line_pixels(x, y_pos, x + width, y_pos, color, 2.0)?;
        }
        Ok(())
    }

    /// Draw a marker for legend
    fn draw_marker(
        &self,
        canvas: &mut dyn Canvas,
        marker: MarkerShape,
        x: f32,
        y: f32,
        size: f32,
        color: &[u8; 4],
    ) -> Result<()> {
        match marker {
            MarkerShape::Circle => {
                // Draw filled circle using multiple horizontal lines
                let radius = size;
                let steps = (radius * 2.0) as i32;
                for i in 0..steps {
                    let dy = i as f32 - radius;
                    let dx = (radius * radius - dy * dy).sqrt();
                    if dx > 0.0 {
                        canvas.draw_line_pixels(x - dx, y + dy, x + dx, y + dy, color, 1.0)?;
                    }
                }
            }
            MarkerShape::Square => {
                let half = size / 2.0;
                self.draw_legend_box(canvas, x - half, y - half, size, color)?;
            }
            MarkerShape::Triangle => {
                let h = size;
                canvas.draw_line_pixels(x, y - h, x - h * 0.866, y + h * 0.5, color, 2.0)?;
                canvas.draw_line_pixels(
                    x - h * 0.866,
                    y + h * 0.5,
                    x + h * 0.866,
                    y + h * 0.5,
                    color,
                    2.0,
                )?;
                canvas.draw_line_pixels(x + h * 0.866, y + h * 0.5, x, y - h, color, 2.0)?;
            }
            MarkerShape::Diamond => {
                let h = size;
                canvas.draw_line_pixels(x, y - h, x + h, y, color, 2.0)?;
                canvas.draw_line_pixels(x + h, y, x, y + h, color, 2.0)?;
                canvas.draw_line_pixels(x, y + h, x - h, y, color, 2.0)?;
                canvas.draw_line_pixels(x - h, y, x, y - h, color, 2.0)?;
            }
            MarkerShape::Plus => {
                let h = size;
                canvas.draw_line_pixels(x - h, y, x + h, y, color, 2.0)?;
                canvas.draw_line_pixels(x, y - h, x, y + h, color, 2.0)?;
            }
            MarkerShape::Cross => {
                let h = size * 0.707;
                canvas.draw_line_pixels(x - h, y - h, x + h, y + h, color, 2.0)?;
                canvas.draw_line_pixels(x - h, y + h, x + h, y - h, color, 2.0)?;
            }
        }
        Ok(())
    }

    fn draw_box(
        &self,
        canvas: &mut dyn Canvas,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<()> {
        // Draw background rectangle (using lines to create a filled box)
        let steps = (height / 2.0).ceil() as i32;
        for i in 0..steps {
            let y_pos = y + i as f32 * 2.0;
            canvas.draw_line_pixels(x, y_pos, x + width, y_pos, &self.background.to_rgba(), 2.0)?;
        }

        // Draw border
        if self.show_border {
            // Top
            canvas.draw_line_pixels(x, y, x + width, y, &self.border_color.to_rgba(), 1.0)?;
            // Right
            canvas.draw_line_pixels(
                x + width,
                y,
                x + width,
                y + height,
                &self.border_color.to_rgba(),
                1.0,
            )?;
            // Bottom
            canvas.draw_line_pixels(
                x,
                y + height,
                x + width,
                y + height,
                &self.border_color.to_rgba(),
                1.0,
            )?;
            // Left
            canvas.draw_line_pixels(x, y, x, y + height, &self.border_color.to_rgba(), 1.0)?;
        }

        Ok(())
    }
}

impl Default for Legend {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Legend {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        self.render(canvas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legend_creation() {
        let legend = Legend::new()
            .add_entry(LegendEntry::new("Series RED").color(Color::RED))
            .add_entry(LegendEntry::new("Series BLUE").color(Color::BLUE));

        assert_eq!(legend.entries.len(), 2);
        assert!(!legend.is_empty());
    }

    #[test]
    fn test_legend_box_calculation() {
        let legend = Legend::new().add_entry(LegendEntry::new("Test").color(Color::RED));

        let (width, height) = legend.calculate_box();
        assert!(width > 0.0);
        assert!(height > 0.0);
    }
}
