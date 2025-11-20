//! Area and stacked area chart implementations
//!
//! Area charts fill the region between a line and the baseline. Stacked area
//! charts show the evolution of multiple series stacked on top of each other.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! // Simple area plot
//! let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
//! let y = vec![1.0, 3.0, 2.0, 4.0, 3.0];
//! let area = AreaPlot::new(x, y).unwrap()
//!     .color(Color::from_hex("#3498db").unwrap())
//!     .opacity(0.6);
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::{Error, Result};
use crate::legend::LegendEntry;

/// Simple area plot (fills region between line and baseline)
pub struct AreaPlot {
    x: Vec<f64>,
    y: Vec<f64>,
    color: Color,
    baseline: f64,
    opacity: f32,
    label: Option<String>,
    line_width: f32,
    show_line: bool,
}

impl AreaPlot {
    /// Create a new area plot
    ///
    /// # Arguments
    ///
    /// * `x` - X-axis values
    /// * `y` - Y-axis values
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let x = vec![0.0, 1.0, 2.0, 3.0];
    /// let y = vec![1.0, 2.0, 1.5, 3.0];
    /// let area = AreaPlot::new(x, y);
    /// ```
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Result<Self> {
        if x.is_empty() || y.is_empty() {
            return Err(Error::InvalidData("Area plot data cannot be empty".into()));
        }

        if x.len() != y.len() {
            return Err(Error::InvalidData(
                "x and y must have the same length".into(),
            ));
        }

        Ok(Self {
            x,
            y,
            color: Color::from_hex("#3498db").unwrap(),
            baseline: 0.0,
            opacity: 0.6,
            label: None,
            line_width: 2.0,
            show_line: true,
        })
    }

    /// Set the fill color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the baseline (default 0.0)
    #[must_use]
    pub fn baseline(mut self, baseline: f64) -> Self {
        self.baseline = baseline;
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

    /// Set the line width for the top edge
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width.max(0.0);
        self
    }

    /// Whether to show the line at the top edge
    #[must_use]
    pub fn show_line(mut self, show: bool) -> Self {
        self.show_line = show;
        self
    }

    /// Get legend entry
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label.as_ref().map(|label| {
            LegendEntry::new(label.clone())
                .color(self.color)
                .swatch_shape()
        })
    }

    /// Get bounds
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.x.is_empty() || self.y.is_empty() {
            return None;
        }

        let x_min = self.x.iter().copied().fold(f64::INFINITY, f64::min);
        let x_max = self.x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let y_min = self
            .y
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min)
            .min(self.baseline);
        let y_max = self
            .y
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(self.baseline);

        Some(Bounds::new(x_min, x_max, y_min, y_max))
    }
}

impl Drawable for AreaPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.x.is_empty() {
            return Ok(());
        }

        // Apply opacity to color
        let mut fill_color = self.color.to_rgba();
        fill_color[3] = (f32::from(fill_color[3]) * self.opacity) as u8;

        // Draw filled area using horizontal lines
        // Find min/max y to know the scan range
        let y_min = self
            .y
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min)
            .min(self.baseline);
        let y_max = self
            .y
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(self.baseline);

        // Convert to pixel space to determine scan lines
        let bounds = canvas.bounds();
        let (_width, height) = canvas.dimensions();
        let pixels_per_unit = f64::from(height) / (bounds.y_max - bounds.y_min);
        let y_range = y_max - y_min;
        let scan_lines = (y_range * pixels_per_unit).ceil() as usize;

        // Draw horizontal scan lines to fill the area
        for i in 0..=scan_lines {
            let y = y_min + (i as f64 / scan_lines as f64) * y_range;

            // Find x_left and x_right for this y scanline
            // We need to interpolate between data points
            let mut x_intersections = Vec::new();

            // Add baseline intersection points
            if (y - self.baseline).abs() < f64::EPSILON {
                x_intersections.push(self.x[0]);
                x_intersections.push(self.x[self.x.len() - 1]);
            }

            // Find intersections with line segments
            for j in 0..self.x.len() - 1 {
                let x1 = self.x[j];
                let y1 = self.y[j];
                let x2 = self.x[j + 1];
                let y2 = self.y[j + 1];

                // Check if scanline crosses this segment
                let y_min_seg = y1.min(y2);
                let y_max_seg = y1.max(y2);

                if y >= y_min_seg && y <= y_max_seg {
                    // Linear interpolation to find x
                    let t = if (y2 - y1).abs() < f64::EPSILON {
                        0.0
                    } else {
                        (y - y1) / (y2 - y1)
                    };
                    let x_intersect = x1 + t * (x2 - x1);
                    x_intersections.push(x_intersect);
                }

                // Also check intersection with baseline
                let baseline_min = y1.min(self.baseline);
                let baseline_max = y1.max(self.baseline);
                if y >= baseline_min && y <= baseline_max {
                    let t = if (self.baseline - y1).abs() < f64::EPSILON {
                        0.0
                    } else {
                        (y - y1) / (self.baseline - y1)
                    };
                    if (0.0..=1.0).contains(&t) {
                        let x_intersect = x1 + t * (x2 - x1);
                        x_intersections.push(x_intersect);
                    }
                }
            }

            if x_intersections.len() >= 2 {
                x_intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let x_left = x_intersections[0];
                let x_right = x_intersections[x_intersections.len() - 1];

                let p1 = Point2D::new(x_left, y);
                let p2 = Point2D::new(x_right, y);
                canvas.draw_line(&p1, &p2, &fill_color, 1.0)?;
            }
        }

        // Draw the top edge line if requested
        if self.show_line {
            let line_color = self.color.to_rgba();
            for i in 0..self.x.len() - 1 {
                let p1 = Point2D::new(self.x[i], self.y[i]);
                let p2 = Point2D::new(self.x[i + 1], self.y[i + 1]);
                canvas.draw_line(&p1, &p2, &line_color, self.line_width)?;
            }
        }

        Ok(())
    }
}

/// Stacked area chart for showing evolution of multiple series
pub struct StackedAreaPlot {
    x: Vec<f64>,
    series: Vec<Vec<f64>>,
    colors: Vec<Color>,
    labels: Vec<String>,
    opacity: f32,
    show_lines: bool,
    line_width: f32,
}

impl StackedAreaPlot {
    /// Create a new stacked area plot
    ///
    /// # Arguments
    ///
    /// * `x` - Shared X-axis values
    /// * `series` - Vector of Y-value series to stack
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let x = vec![0.0, 1.0, 2.0, 3.0];
    /// let series1 = vec![1.0, 2.0, 1.5, 3.0];
    /// let series2 = vec![0.5, 1.0, 0.8, 1.2];
    /// let stacked = StackedAreaPlot::new(x, vec![series1, series2]);
    /// ```
    pub fn new(x: Vec<f64>, series: Vec<Vec<f64>>) -> Result<Self> {
        if x.is_empty() {
            return Err(Error::InvalidData("X data cannot be empty".into()));
        }

        if series.is_empty() {
            return Err(Error::InvalidData("Series cannot be empty".into()));
        }

        for (i, s) in series.iter().enumerate() {
            if s.len() != x.len() {
                return Err(Error::InvalidData(format!(
                    "Series {} length ({}) doesn't match x length ({})",
                    i,
                    s.len(),
                    x.len()
                )));
            }
        }

        let n_series = series.len();

        // Default colors
        let default_colors = [
            Color::from_hex("#3498db").unwrap(),
            Color::from_hex("#e74c3c").unwrap(),
            Color::from_hex("#2ecc71").unwrap(),
            Color::from_hex("#f39c12").unwrap(),
            Color::from_hex("#9b59b6").unwrap(),
            Color::from_hex("#1abc9c").unwrap(),
        ];

        let mut colors = Vec::new();
        for i in 0..n_series {
            colors.push(default_colors[i % default_colors.len()]);
        }

        let labels = (0..n_series).map(|i| format!("Series {}", i + 1)).collect();

        Ok(Self {
            x,
            series,
            colors,
            labels,
            opacity: 0.7,
            show_lines: true,
            line_width: 1.5,
        })
    }

    /// Set custom colors
    #[must_use]
    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set labels for legend
    #[must_use]
    pub fn labels(mut self, labels: Vec<impl Into<String>>) -> Self {
        self.labels = labels.into_iter().map(std::convert::Into::into).collect();
        self
    }

    /// Set opacity
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Whether to show lines at area boundaries
    #[must_use]
    pub fn show_lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Set line width
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width.max(0.0);
        self
    }

    /// Get legend entries
    #[must_use]
    pub fn legend_entries(&self) -> Vec<LegendEntry> {
        self.labels
            .iter()
            .zip(&self.colors)
            .map(|(label, color)| LegendEntry::new(label.clone()).color(*color).swatch_shape())
            .collect()
    }

    /// Get bounds
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.x.is_empty() || self.series.is_empty() {
            return None;
        }

        let x_min = self.x.iter().copied().fold(f64::INFINITY, f64::min);
        let x_max = self.x.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        // Calculate cumulative max for y
        let mut max_cumulative: f64 = 0.0;
        for i in 0..self.x.len() {
            let mut cumulative: f64 = 0.0;
            for series in &self.series {
                cumulative += series[i];
            }
            max_cumulative = max_cumulative.max(cumulative);
        }

        Some(Bounds::new(x_min, x_max, 0.0, max_cumulative))
    }
}

impl Drawable for StackedAreaPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.x.is_empty() || self.series.is_empty() {
            return Ok(());
        }

        // Calculate cumulative series (bottom to top)
        let n_points = self.x.len();
        let n_series = self.series.len();

        // Draw from bottom to top
        for series_idx in 0..n_series {
            // Calculate cumulative y values for this layer
            let mut y_bottom = vec![0.0; n_points];
            let mut y_top = vec![0.0; n_points];

            for i in 0..n_points {
                // Sum all series below this one
                for j in 0..series_idx {
                    y_bottom[i] += self.series[j][i];
                }
                y_top[i] = y_bottom[i] + self.series[series_idx][i];
            }

            // Get color with opacity
            let mut fill_color = self.colors[series_idx % self.colors.len()].to_rgba();
            fill_color[3] = (f32::from(fill_color[3]) * self.opacity) as u8;

            // Fill the area between y_bottom and y_top
            self.draw_filled_area(canvas, &self.x, &y_bottom, &y_top, &fill_color)?;

            // Draw top edge line if requested
            if self.show_lines {
                let line_color = self.colors[series_idx % self.colors.len()].to_rgba();
                for i in 0..n_points - 1 {
                    let p1 = Point2D::new(self.x[i], y_top[i]);
                    let p2 = Point2D::new(self.x[i + 1], y_top[i + 1]);
                    canvas.draw_line(&p1, &p2, &line_color, self.line_width)?;
                }
            }
        }

        Ok(())
    }
}

impl StackedAreaPlot {
    fn draw_filled_area(
        &self,
        canvas: &mut dyn Canvas,
        x: &[f64],
        y_bottom: &[f64],
        y_top: &[f64],
        color: &[u8; 4],
    ) -> Result<()> {
        // Find y range for scanning
        let y_min = y_bottom
            .iter()
            .chain(y_top.iter())
            .copied()
            .fold(f64::INFINITY, f64::min);
        let y_max = y_bottom
            .iter()
            .chain(y_top.iter())
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        if (y_max - y_min).abs() < f64::EPSILON {
            return Ok(());
        }

        // Determine number of scan lines based on pixel resolution
        let bounds = canvas.bounds();
        let (_width, height) = canvas.dimensions();
        let pixels_per_unit = f64::from(height) / (bounds.y_max - bounds.y_min);
        let y_range = y_max - y_min;
        let scan_lines = ((y_range * pixels_per_unit).ceil() as usize).max(10);

        // Draw horizontal scan lines
        for i in 0..=scan_lines {
            let y = y_min + (i as f64 / scan_lines as f64) * y_range;

            // Find x range for this y by interpolating in both curves
            let mut x_left = f64::INFINITY;
            let mut x_right = f64::NEG_INFINITY;

            // Interpolate along top curve
            for j in 0..x.len() - 1 {
                let y1_top = y_top[j];
                let y2_top = y_top[j + 1];

                if (y >= y1_top.min(y2_top)) && (y <= y1_top.max(y2_top)) {
                    let t = if (y2_top - y1_top).abs() < f64::EPSILON {
                        0.5
                    } else {
                        (y - y1_top) / (y2_top - y1_top)
                    };
                    let x_intersect = x[j] + t * (x[j + 1] - x[j]);
                    x_left = x_left.min(x_intersect);
                    x_right = x_right.max(x_intersect);
                }
            }

            // Interpolate along bottom curve
            for j in 0..x.len() - 1 {
                let y1_bottom = y_bottom[j];
                let y2_bottom = y_bottom[j + 1];

                if (y >= y1_bottom.min(y2_bottom)) && (y <= y1_bottom.max(y2_bottom)) {
                    let t = if (y2_bottom - y1_bottom).abs() < f64::EPSILON {
                        0.5
                    } else {
                        (y - y1_bottom) / (y2_bottom - y1_bottom)
                    };
                    let x_intersect = x[j] + t * (x[j + 1] - x[j]);
                    x_left = x_left.min(x_intersect);
                    x_right = x_right.max(x_intersect);
                }
            }

            if x_left < f64::INFINITY && x_right > f64::NEG_INFINITY {
                let p1 = Point2D::new(x_left, y);
                let p2 = Point2D::new(x_right, y);
                canvas.draw_line(&p1, &p2, color, 1.0)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_plot_creation() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![1.0, 2.0, 1.5];
        let area = AreaPlot::new(x, y).unwrap();
        assert!(area.bounds().is_some());
    }

    #[test]
    fn test_stacked_area_creation() {
        let x = vec![0.0, 1.0, 2.0];
        let series = vec![vec![1.0, 2.0, 1.5], vec![0.5, 1.0, 0.8]];
        let stacked = StackedAreaPlot::new(x, series).unwrap();
        assert!(stacked.bounds().is_some());
    }

    #[test]
    fn test_stacked_area_bounds() {
        let x = vec![0.0, 1.0, 2.0];
        let series = vec![vec![1.0, 2.0, 1.5], vec![0.5, 1.0, 0.8]];
        let stacked = StackedAreaPlot::new(x, series).unwrap();
        let bounds = stacked.bounds().unwrap();

        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 2.0);
        assert_eq!(bounds.y_min, 0.0);
        // Max cumulative: max(1+0.5, 2+1, 1.5+0.8) = 3.0
        assert_eq!(bounds.y_max, 3.0);
    }
}
