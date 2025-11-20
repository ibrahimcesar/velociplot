//! Box plot implementation
//!
//! Box plots (box-and-whisker plots) display distribution statistics:
//! minimum, first quartile (Q1), median (Q2), third quartile (Q3), and maximum.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
//! let boxplot = BoxPlot::new(data)
//!     .color(Color::from_hex("#3498db").unwrap())
//!     .label("Sample Data");
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::Result;
use crate::legend::LegendEntry;

/// Box plot for displaying distribution statistics
///
/// Shows five-number summary: minimum, Q1, median, Q3, maximum
pub struct BoxPlot {
    data: Vec<f64>,
    position: f64,
    width: f64,
    color: Color,
    label: Option<String>,
    show_outliers: bool,
    outlier_method: OutlierMethod,
}

/// Method for detecting outliers
#[derive(Debug, Clone, Copy)]
pub enum OutlierMethod {
    /// Use 1.5 × IQR (Interquartile Range) - standard Tukey method
    IQR,
    /// No outlier detection, show full range
    None,
}

impl BoxPlot {
    /// Create a new box plot
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let boxplot = BoxPlot::new(data);
    /// ```
    #[must_use]
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            position: 1.0,
            width: 0.6,
            color: Color::from_hex("#3498db").unwrap(),
            label: None,
            show_outliers: true,
            outlier_method: OutlierMethod::IQR,
        }
    }

    /// Set the x-axis position of the box plot
    #[must_use]
    pub fn position(mut self, position: f64) -> Self {
        self.position = position;
        self
    }

    /// Set the width of the box
    #[must_use]
    pub fn width(mut self, width: f64) -> Self {
        self.width = width.clamp(0.1, 2.0);
        self
    }

    /// Set the color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the label for legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set whether to show outliers
    #[must_use]
    pub fn show_outliers(mut self, show: bool) -> Self {
        self.show_outliers = show;
        self
    }

    /// Set the outlier detection method
    #[must_use]
    pub fn outlier_method(mut self, method: OutlierMethod) -> Self {
        self.outlier_method = method;
        self
    }

    /// Calculate statistics for the box plot
    fn calculate_stats(&self) -> BoxStats {
        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted.len();
        if n == 0 {
            return BoxStats::default();
        }

        let q1 = percentile(&sorted, 25.0);
        let median = percentile(&sorted, 50.0);
        let q3 = percentile(&sorted, 75.0);
        let iqr = q3 - q1;

        let (lower_whisker, upper_whisker, outliers) = match self.outlier_method {
            OutlierMethod::IQR => {
                let lower_fence = q1 - 1.5 * iqr;
                let upper_fence = q3 + 1.5 * iqr;

                let lower_whisker = sorted
                    .iter()
                    .find(|&&x| x >= lower_fence)
                    .copied()
                    .unwrap_or(sorted[0]);

                let upper_whisker = sorted
                    .iter()
                    .rev()
                    .find(|&&x| x <= upper_fence)
                    .copied()
                    .unwrap_or(sorted[n - 1]);

                let outliers: Vec<f64> = sorted
                    .iter()
                    .filter(|&&x| x < lower_fence || x > upper_fence)
                    .copied()
                    .collect();

                (lower_whisker, upper_whisker, outliers)
            }
            OutlierMethod::None => {
                let lower_whisker = sorted[0];
                let upper_whisker = sorted[n - 1];
                (lower_whisker, upper_whisker, Vec::new())
            }
        };

        BoxStats {
            q1,
            median,
            q3,
            lower_whisker,
            upper_whisker,
            outliers,
        }
    }

    /// Get legend entry for this box plot
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label.as_ref().map(|label| {
            LegendEntry::new(label.clone())
                .color(self.color)
                .box_shape()
        })
    }
}

#[derive(Debug, Clone, Default)]
struct BoxStats {
    q1: f64,
    median: f64,
    q3: f64,
    lower_whisker: f64,
    upper_whisker: f64,
    outliers: Vec<f64>,
}

/// Calculate percentile of sorted data
fn percentile(sorted: &[f64], p: f64) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return sorted[0];
    }

    let rank = (p / 100.0) * (n - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let fraction = rank - lower as f64;

    sorted[lower] * (1.0 - fraction) + sorted[upper] * fraction
}

impl Drawable for BoxPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let stats = self.calculate_stats();

        let half_width = self.width / 2.0;
        let left = self.position - half_width;
        let right = self.position + half_width;

        // Draw whiskers (vertical lines)
        canvas.draw_line(
            &Point2D::new(self.position, stats.lower_whisker),
            &Point2D::new(self.position, stats.q1),
            &self.color.to_rgba(),
            1.5,
        )?;

        canvas.draw_line(
            &Point2D::new(self.position, stats.q3),
            &Point2D::new(self.position, stats.upper_whisker),
            &self.color.to_rgba(),
            1.5,
        )?;

        // Draw whisker caps (horizontal lines)
        let cap_width = self.width * 0.3;
        canvas.draw_line(
            &Point2D::new(self.position - cap_width / 2.0, stats.lower_whisker),
            &Point2D::new(self.position + cap_width / 2.0, stats.lower_whisker),
            &self.color.to_rgba(),
            1.5,
        )?;

        canvas.draw_line(
            &Point2D::new(self.position - cap_width / 2.0, stats.upper_whisker),
            &Point2D::new(self.position + cap_width / 2.0, stats.upper_whisker),
            &self.color.to_rgba(),
            1.5,
        )?;

        // Draw box (Q1 to Q3) using four lines
        // Left side
        canvas.draw_line(
            &Point2D::new(left, stats.q1),
            &Point2D::new(left, stats.q3),
            &self.color.to_rgba(),
            2.0,
        )?;
        // Right side
        canvas.draw_line(
            &Point2D::new(right, stats.q1),
            &Point2D::new(right, stats.q3),
            &self.color.to_rgba(),
            2.0,
        )?;
        // Top
        canvas.draw_line(
            &Point2D::new(left, stats.q3),
            &Point2D::new(right, stats.q3),
            &self.color.to_rgba(),
            2.0,
        )?;
        // Bottom
        canvas.draw_line(
            &Point2D::new(left, stats.q1),
            &Point2D::new(right, stats.q1),
            &self.color.to_rgba(),
            2.0,
        )?;

        // Draw median line
        canvas.draw_line(
            &Point2D::new(left, stats.median),
            &Point2D::new(right, stats.median),
            &self.color.to_rgba(),
            2.5,
        )?;

        // Draw outliers if enabled
        if self.show_outliers {
            for &outlier in &stats.outliers {
                canvas.draw_circle(
                    &Point2D::new(self.position, outlier),
                    3.0,
                    &self.color.to_rgba(),
                    true, // filled
                )?;
            }
        }

        Ok(())
    }
}

impl BoxPlot {
    /// Get bounds for this box plot
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.data.is_empty() {
            return None;
        }

        let stats = self.calculate_stats();
        let half_width = self.width / 2.0;

        let y_min = if self.show_outliers && !stats.outliers.is_empty() {
            stats
                .outliers
                .iter()
                .copied()
                .fold(stats.lower_whisker, f64::min)
        } else {
            stats.lower_whisker
        };

        let y_max = if self.show_outliers && !stats.outliers.is_empty() {
            stats
                .outliers
                .iter()
                .copied()
                .fold(stats.upper_whisker, f64::max)
        } else {
            stats.upper_whisker
        };

        Some(Bounds::new(
            self.position - half_width, // x_min
            self.position + half_width, // x_max
            y_min,                      // y_min
            y_max,                      // y_max
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxplot_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let boxplot = BoxPlot::new(data);
        assert!(boxplot.bounds().is_some());
    }

    #[test]
    fn test_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&data, 0.0), 1.0);
        assert_eq!(percentile(&data, 50.0), 3.0);
        assert_eq!(percentile(&data, 100.0), 5.0);
    }

    #[test]
    fn test_boxplot_stats() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let boxplot = BoxPlot::new(data);
        let stats = boxplot.calculate_stats();

        assert_eq!(stats.median, 5.5);
        assert!(stats.q1 > 0.0 && stats.q1 < stats.median);
        assert!(stats.q3 < 11.0 && stats.q3 > stats.median);
    }

    #[test]
    fn test_boxplot_with_outliers() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier
        let boxplot = BoxPlot::new(data).outlier_method(OutlierMethod::IQR);
        let stats = boxplot.calculate_stats();

        assert!(!stats.outliers.is_empty());
    }

    #[test]
    fn test_boxplot_bounds() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let boxplot = BoxPlot::new(data).position(2.0).width(0.8);
        let bounds = boxplot.bounds().unwrap();

        // Check that bounds exist and are reasonable
        // The box is centered at 2.0 with width 0.8, so it spans 1.6 to 2.4
        assert!(bounds.x_min > 0.0 && bounds.x_min < 2.0);
        assert!(bounds.x_max > 2.0 && bounds.x_max < 3.0);
        assert!(bounds.y_min >= 1.0);
        assert!(bounds.y_max <= 5.0);
    }
}
