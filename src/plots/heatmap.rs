//! Heatmap implementation for 2D matrix visualization
//!
//! Heatmaps display 2D data as colored rectangles, with colors representing values.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let data = vec![
//!     vec![1.0, 2.0, 3.0],
//!     vec![4.0, 5.0, 6.0],
//!     vec![7.0, 8.0, 9.0],
//! ];
//! let heatmap = Heatmap::new(data).unwrap()
//!     .colormap(Colormap::viridis());
//! ```

use crate::color::{Color, Colormap};
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::{Error, Result};

/// Heatmap for visualizing 2D matrices with color coding
pub struct Heatmap {
    data: Vec<Vec<f64>>,
    colormap: Colormap,
    x_labels: Option<Vec<String>>,
    y_labels: Option<Vec<String>>,
    show_values: bool,
    value_format: ValueFormat,
}

/// Format for displaying values on heatmap cells
#[derive(Debug, Clone, Copy)]
pub enum ValueFormat {
    /// No decimal places (integer)
    Integer,
    /// Fixed decimal places
    Decimal(usize),
    /// Scientific notation
    Scientific,
}

impl Heatmap {
    /// Create a new heatmap from 2D data
    ///
    /// # Arguments
    ///
    /// * `data` - 2D vector where data[row][col] represents the value at (row, col)
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let data = vec![
    ///     vec![1.0, 2.0, 3.0],
    ///     vec![4.0, 5.0, 6.0],
    /// ];
    /// let heatmap = Heatmap::new(data);
    /// ```
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self> {
        if data.is_empty() {
            return Err(Error::InvalidData("Heatmap data cannot be empty".into()));
        }

        let first_len = data[0].len();
        if first_len == 0 {
            return Err(Error::InvalidData("Heatmap rows cannot be empty".into()));
        }

        for row in &data {
            if row.len() != first_len {
                return Err(Error::InvalidData(
                    "All heatmap rows must have the same length".into(),
                ));
            }
        }

        Ok(Self {
            data,
            colormap: Colormap::viridis(),
            x_labels: None,
            y_labels: None,
            show_values: false,
            value_format: ValueFormat::Decimal(1),
        })
    }

    /// Set the colormap for the heatmap
    #[must_use]
    pub fn colormap(mut self, colormap: Colormap) -> Self {
        self.colormap = colormap;
        self
    }

    /// Set x-axis labels
    #[must_use]
    pub fn x_labels(mut self, labels: Vec<String>) -> Self {
        self.x_labels = Some(labels);
        self
    }

    /// Set y-axis labels
    #[must_use]
    pub fn y_labels(mut self, labels: Vec<String>) -> Self {
        self.y_labels = Some(labels);
        self
    }

    /// Show values in cells
    #[must_use]
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set value format
    #[must_use]
    pub fn value_format(mut self, format: ValueFormat) -> Self {
        self.value_format = format;
        self
    }

    /// Get dimensions (rows, cols)
    #[must_use]
    pub fn dimensions(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };
        (rows, cols)
    }

    /// Get min and max values in the data
    fn value_range(&self) -> (f64, f64) {
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;

        for row in &self.data {
            for &val in row {
                if val < min_val {
                    min_val = val;
                }
                if val > max_val {
                    max_val = val;
                }
            }
        }

        (min_val, max_val)
    }

    /// Normalize value to 0-1 range for colormap lookup
    fn normalize_value(&self, value: f64, min_val: f64, max_val: f64) -> f32 {
        if (max_val - min_val).abs() < f64::EPSILON {
            return 0.5;
        }
        ((value - min_val) / (max_val - min_val)) as f32
    }

    /// Format value for display
    fn format_value(&self, value: f64) -> String {
        match self.value_format {
            ValueFormat::Integer => format!("{value:.0}"),
            ValueFormat::Decimal(places) => format!("{value:.places$}"),
            ValueFormat::Scientific => format!("{value:.2e}"),
        }
    }
}

impl Drawable for Heatmap {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let (rows, _cols) = self.dimensions();
        let (min_val, max_val) = self.value_range();

        // Calculate cell dimensions in data coordinates
        let cell_width = 1.0;
        let cell_height = 1.0;

        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                let x = col_idx as f64 * cell_width;
                let y = (rows - 1 - row_idx) as f64 * cell_height; // Flip y-axis

                // Get color from colormap
                let t = self.normalize_value(value, min_val, max_val);
                let color = self.colormap.get(t);

                // Draw filled rectangle for the cell
                canvas.draw_rectangle(
                    &Point2D::new(x, y),
                    cell_width,
                    cell_height,
                    &color.to_rgba(),
                )?;

                // Draw value text if enabled
                if self.show_values {
                    let text = self.format_value(value);
                    let text_x = x + cell_width / 2.0;
                    let text_y = y + cell_height / 2.0;

                    // Determine text color (white for dark cells, black for light cells)
                    let luminance = 0.299 * f64::from(color.r)
                        + 0.587 * f64::from(color.g)
                        + 0.114 * f64::from(color.b);
                    let text_color = if luminance > 128.0 {
                        Color::BLACK
                    } else {
                        Color::WHITE
                    };

                    canvas.draw_text(
                        &text,
                        text_x as f32,
                        text_y as f32,
                        12.0,
                        &text_color.to_rgba(),
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl Heatmap {
    /// Get bounds for this heatmap
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        let (rows, cols) = self.dimensions();
        if rows == 0 || cols == 0 {
            return None;
        }

        Some(Bounds::new(
            0.0,         // x_min
            cols as f64, // x_max
            0.0,         // y_min
            rows as f64, // y_max
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_creation() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let heatmap = Heatmap::new(data).unwrap();
        assert_eq!(heatmap.dimensions(), (2, 2));
    }

    #[test]
    fn test_heatmap_empty_data() {
        let data: Vec<Vec<f64>> = vec![];
        let result = Heatmap::new(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_heatmap_inconsistent_rows() {
        let data = vec![vec![1.0, 2.0], vec![3.0]];
        let result = Heatmap::new(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_heatmap_value_range() {
        let data = vec![vec![1.0, 5.0], vec![2.0, 8.0]];
        let heatmap = Heatmap::new(data).unwrap();
        let (min_val, max_val) = heatmap.value_range();
        assert_eq!(min_val, 1.0);
        assert_eq!(max_val, 8.0);
    }

    #[test]
    fn test_heatmap_normalize() {
        let data = vec![vec![0.0, 50.0, 100.0]];
        let heatmap = Heatmap::new(data).unwrap();
        assert_eq!(heatmap.normalize_value(0.0, 0.0, 100.0), 0.0);
        assert_eq!(heatmap.normalize_value(50.0, 0.0, 100.0), 0.5);
        assert_eq!(heatmap.normalize_value(100.0, 0.0, 100.0), 1.0);
    }

    #[test]
    fn test_heatmap_bounds() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let heatmap = Heatmap::new(data).unwrap();
        let bounds = heatmap.bounds().unwrap();
        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 3.0);
        assert_eq!(bounds.y_min, 0.0);
        assert_eq!(bounds.y_max, 2.0);
    }
}
