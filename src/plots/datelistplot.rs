//! `DateListPlot` - Time series data visualization
//!
//! Plot data values as a function of time/date. Similar to Wolfram's `DateListPlot`,
//! this plots temporal data with automatic time axis formatting.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let times = vec![0.0, 1.0, 2.0, 3.0, 4.0];
//! let values = vec![10.0, 15.0, 12.0, 18.0, 14.0];
//! let plot = DateListPlot::new(times, values).unwrap()
//!     .label("Temperature");
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::{Error, Result};
use crate::legend::LegendEntry;

/// Style for rendering time series
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateListStyle {
    /// Connected line plot
    Line,
    /// Individual points
    Points,
    /// Both lines and points
    LinePoints,
}

/// A single time series in a `DateListPlot`
#[derive(Debug, Clone)]
pub struct TimeSeries {
    /// Time points (e.g., timestamps, dates, sequential time)
    pub times: Vec<f64>,
    /// Data values at each time point
    pub values: Vec<f64>,
    /// Color for this series
    pub color: Color,
    /// Label for legend
    pub label: Option<String>,
    /// Plotting style
    pub style: DateListStyle,
    /// Line width (for Line and `LinePoints` styles)
    pub line_width: f32,
    /// Point size (for Points and `LinePoints` styles)
    pub point_size: f32,
}

/// `DateListPlot` for time series visualization
///
/// Plots one or more time series with automatic time axis handling.
pub struct DateListPlot {
    series: Vec<TimeSeries>,
    show_grid: bool,
    grid_color: Color,
}

impl DateListPlot {
    /// Create a new `DateListPlot` with a single series
    ///
    /// # Arguments
    ///
    /// * `times` - Time points (must be sorted)
    /// * `values` - Data values at each time point
    ///
    /// # Errors
    ///
    /// Returns an error if times and values have different lengths or are empty
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let times = vec![0.0, 1.0, 2.0, 3.0];
    /// let values = vec![5.0, 8.0, 6.0, 9.0];
    /// let plot = DateListPlot::new(times, values).unwrap();
    /// ```
    pub fn new(times: Vec<f64>, values: Vec<f64>) -> Result<Self> {
        if times.is_empty() || values.is_empty() {
            return Err(Error::InvalidData(
                "Time series data cannot be empty".into(),
            ));
        }

        if times.len() != values.len() {
            return Err(Error::InvalidData(
                "Times and values must have the same length".into(),
            ));
        }

        let series = TimeSeries {
            times,
            values,
            color: Color::from_hex("#3498db").unwrap(),
            label: None,
            style: DateListStyle::Line,
            line_width: 2.0,
            point_size: 4.0,
        };

        Ok(Self {
            series: vec![series],
            show_grid: false,
            grid_color: Color::from_hex("#ecf0f1").unwrap(),
        })
    }

    /// Create an empty `DateListPlot` to add multiple series
    #[must_use]
    pub fn empty() -> Self {
        Self {
            series: Vec::new(),
            show_grid: false,
            grid_color: Color::from_hex("#ecf0f1").unwrap(),
        }
    }

    /// Add a time series
    pub fn add_series(
        &mut self,
        times: Vec<f64>,
        values: Vec<f64>,
        label: impl Into<Option<String>>,
    ) -> Result<&mut Self> {
        if times.is_empty() || values.is_empty() {
            return Err(Error::InvalidData(
                "Time series data cannot be empty".into(),
            ));
        }

        if times.len() != values.len() {
            return Err(Error::InvalidData(
                "Times and values must have the same length".into(),
            ));
        }

        // Auto-assign color
        let colors = [
            Color::from_hex("#3498db").unwrap(),
            Color::from_hex("#e74c3c").unwrap(),
            Color::from_hex("#2ecc71").unwrap(),
            Color::from_hex("#f39c12").unwrap(),
            Color::from_hex("#9b59b6").unwrap(),
            Color::from_hex("#1abc9c").unwrap(),
        ];

        let color = colors[self.series.len() % colors.len()];

        self.series.push(TimeSeries {
            times,
            values,
            color,
            label: label.into(),
            style: DateListStyle::Line,
            line_width: 2.0,
            point_size: 4.0,
        });

        Ok(self)
    }

    /// Set the label for the first series
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        if let Some(series) = self.series.first_mut() {
            series.label = Some(label.into());
        }
        self
    }

    /// Set the color for the first series
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        if let Some(series) = self.series.first_mut() {
            series.color = color;
        }
        self
    }

    /// Set the plotting style for the first series
    #[must_use]
    pub fn style(mut self, style: DateListStyle) -> Self {
        if let Some(series) = self.series.first_mut() {
            series.style = style;
        }
        self
    }

    /// Set line width for the first series
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        if let Some(series) = self.series.first_mut() {
            series.line_width = width.max(0.0);
        }
        self
    }

    /// Set point size for the first series
    #[must_use]
    pub fn point_size(mut self, size: f32) -> Self {
        if let Some(series) = self.series.first_mut() {
            series.point_size = size.max(0.0);
        }
        self
    }

    /// Set whether to show grid
    #[must_use]
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Get legend entries for all series
    #[must_use]
    pub fn legend_entries(&self) -> Vec<LegendEntry> {
        self.series
            .iter()
            .filter_map(|s| {
                s.label.as_ref().map(|label| {
                    LegendEntry::new(label.clone())
                        .color(s.color)
                        .line_width(s.line_width)
                })
            })
            .collect()
    }

    /// Get bounds for all series
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.series.is_empty() {
            return None;
        }

        let mut min_time = f64::INFINITY;
        let mut max_time = f64::NEG_INFINITY;
        let mut min_value = f64::INFINITY;
        let mut max_value = f64::NEG_INFINITY;

        for series in &self.series {
            for &t in &series.times {
                min_time = min_time.min(t);
                max_time = max_time.max(t);
            }
            for &v in &series.values {
                min_value = min_value.min(v);
                max_value = max_value.max(v);
            }
        }

        Some(Bounds::new(min_time, max_time, min_value, max_value))
    }
}

impl Drawable for DateListPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.series.is_empty() {
            return Ok(());
        }

        // Draw grid if enabled
        if self.show_grid {
            let bounds = canvas.bounds();
            let grid_color = self.grid_color.to_rgba();

            // Vertical grid lines
            let num_v_lines = 10;
            for i in 0..=num_v_lines {
                let t = f64::from(i) / f64::from(num_v_lines);
                let x = bounds.x_min + t * (bounds.x_max - bounds.x_min);
                canvas.draw_line(
                    &Point2D::new(x, bounds.y_min),
                    &Point2D::new(x, bounds.y_max),
                    &grid_color,
                    0.5,
                )?;
            }

            // Horizontal grid lines
            let num_h_lines = 8;
            for i in 0..=num_h_lines {
                let t = f64::from(i) / f64::from(num_h_lines);
                let y = bounds.y_min + t * (bounds.y_max - bounds.y_min);
                canvas.draw_line(
                    &Point2D::new(bounds.x_min, y),
                    &Point2D::new(bounds.x_max, y),
                    &grid_color,
                    0.5,
                )?;
            }
        }

        // Draw each series
        for series in &self.series {
            let color = series.color.to_rgba();

            // Draw lines
            if matches!(
                series.style,
                DateListStyle::Line | DateListStyle::LinePoints
            ) {
                for i in 0..series.times.len() - 1 {
                    let p1 = Point2D::new(series.times[i], series.values[i]);
                    let p2 = Point2D::new(series.times[i + 1], series.values[i + 1]);
                    canvas.draw_line(&p1, &p2, &color, series.line_width)?;
                }
            }

            // Draw points
            if matches!(
                series.style,
                DateListStyle::Points | DateListStyle::LinePoints
            ) {
                for i in 0..series.times.len() {
                    let p = Point2D::new(series.times[i], series.values[i]);
                    canvas.draw_circle(&p, series.point_size, &color, true)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datelistplot_creation() {
        let times = vec![0.0, 1.0, 2.0];
        let values = vec![5.0, 8.0, 6.0];
        let plot = DateListPlot::new(times, values).unwrap();
        assert_eq!(plot.series.len(), 1);
    }

    #[test]
    fn test_empty_data() {
        let times: Vec<f64> = vec![];
        let values: Vec<f64> = vec![];
        let result = DateListPlot::new(times, values);
        assert!(result.is_err());
    }

    #[test]
    fn test_mismatched_lengths() {
        let times = vec![0.0, 1.0];
        let values = vec![5.0, 8.0, 6.0];
        let result = DateListPlot::new(times, values);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_series() {
        let mut plot = DateListPlot::empty();
        plot.add_series(vec![0.0, 1.0], vec![5.0, 8.0], Some("Series 1".to_string()))
            .unwrap();
        plot.add_series(vec![0.0, 1.0], vec![3.0, 6.0], Some("Series 2".to_string()))
            .unwrap();
        assert_eq!(plot.series.len(), 2);
    }

    #[test]
    fn test_bounds() {
        let times = vec![0.0, 1.0, 2.0];
        let values = vec![5.0, 8.0, 6.0];
        let plot = DateListPlot::new(times, values).unwrap();
        let bounds = plot.bounds().unwrap();

        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 2.0);
        assert_eq!(bounds.y_min, 5.0);
        assert_eq!(bounds.y_max, 8.0);
    }
}
