//! Stacked bar chart implementation
//!
//! Stacked bar charts show the composition of categories by stacking bars
//! representing different components on top of each other.
//!
//! # Examples
//!
//! ```
//! # use velociplot::prelude::*;
//! let categories = vec!["Q1", "Q2", "Q3", "Q4"];
//! let series1 = vec![10.0, 15.0, 12.0, 18.0];  // Product A
//! let series2 = vec![8.0, 12.0, 10.0, 14.0];   // Product B
//! let series3 = vec![5.0, 7.0, 6.0, 9.0];      // Product C
//!
//! let stacked = StackedBarPlot::new(categories, vec![series1, series2, series3]).unwrap()
//!     .labels(vec!["Product A", "Product B", "Product C"])
//!     .colors(vec![
//!         Color::from_hex("#3498db").unwrap(),
//!         Color::from_hex("#e74c3c").unwrap(),
//!         Color::from_hex("#2ecc71").unwrap(),
//!     ]);
//! ```

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D};
use crate::error::{Error, Result};
use crate::legend::LegendEntry;
use crate::plots::bar::BarOrientation;

/// Stacked bar chart for showing composition of categories
pub struct StackedBarPlot {
    categories: Vec<String>,
    series: Vec<Vec<f64>>,
    colors: Vec<Color>,
    labels: Vec<String>,
    orientation: BarOrientation,
    bar_width: f64,
    gap: f64,
}

impl StackedBarPlot {
    /// Create a new stacked bar plot
    ///
    /// # Arguments
    ///
    /// * `categories` - Category labels (e.g., ["Q1", "Q2", "Q3"])
    /// * `series` - Vector of data series, each representing a stack component
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let categories = vec!["Jan", "Feb", "Mar"];
    /// let sales = vec![100.0, 120.0, 110.0];
    /// let costs = vec![60.0, 70.0, 65.0];
    /// let stacked = StackedBarPlot::new(categories, vec![sales, costs]);
    /// ```
    pub fn new(categories: Vec<impl Into<String>>, series: Vec<Vec<f64>>) -> Result<Self> {
        if categories.is_empty() {
            return Err(Error::InvalidData("Categories cannot be empty".into()));
        }

        if series.is_empty() {
            return Err(Error::InvalidData("Series cannot be empty".into()));
        }

        let n_categories = categories.len();
        for (i, s) in series.iter().enumerate() {
            if s.len() != n_categories {
                return Err(Error::InvalidData(format!(
                    "Series {} length ({}) doesn't match categories length ({})",
                    i,
                    s.len(),
                    n_categories
                )));
            }
        }

        let n_series = series.len();
        let categories: Vec<String> = categories
            .into_iter()
            .map(std::convert::Into::into)
            .collect();

        // Default colors - cycle through a palette
        let default_colors = vec![
            Color::from_hex("#3498db").unwrap(), // Blue
            Color::from_hex("#e74c3c").unwrap(), // Red
            Color::from_hex("#2ecc71").unwrap(), // Green
            Color::from_hex("#f39c12").unwrap(), // Orange
            Color::from_hex("#9b59b6").unwrap(), // Purple
            Color::from_hex("#1abc9c").unwrap(), // Teal
            Color::from_hex("#e67e22").unwrap(), // Dark orange
            Color::from_hex("#34495e").unwrap(), // Dark gray
        ];

        let mut colors = Vec::new();
        for i in 0..n_series {
            colors.push(default_colors[i % default_colors.len()]);
        }

        let labels = (0..n_series).map(|i| format!("Series {}", i + 1)).collect();

        Ok(Self {
            categories,
            series,
            colors,
            labels,
            orientation: BarOrientation::Vertical,
            bar_width: 0.8,
            gap: 0.2,
        })
    }

    /// Set custom colors for each series
    #[must_use]
    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set labels for each series (for legend)
    #[must_use]
    pub fn labels(mut self, labels: Vec<impl Into<String>>) -> Self {
        self.labels = labels.into_iter().map(std::convert::Into::into).collect();
        self
    }

    /// Set the orientation (vertical or horizontal)
    #[must_use]
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the bar width (relative to spacing, 0.0 to 1.0)
    #[must_use]
    pub fn bar_width(mut self, width: f64) -> Self {
        self.bar_width = width.clamp(0.1, 1.0);
        self
    }

    /// Set the gap between bar groups
    #[must_use]
    pub fn gap(mut self, gap: f64) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Get legend entries for this stacked bar plot
    #[must_use]
    pub fn legend_entries(&self) -> Vec<LegendEntry> {
        self.labels
            .iter()
            .zip(&self.colors)
            .map(|(label, color)| LegendEntry::new(label.clone()).color(*color).swatch_shape())
            .collect()
    }
}

impl Drawable for StackedBarPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let n_categories = self.categories.len();

        for cat_idx in 0..n_categories {
            let x_pos = cat_idx as f64;
            let mut cumulative = 0.0;

            // Draw each series component for this category
            for (series_idx, series_data) in self.series.iter().enumerate() {
                let value = series_data[cat_idx];
                if value < 0.0 {
                    return Err(Error::InvalidData(
                        "Stacked bar plots do not support negative values".into(),
                    ));
                }

                let color = self
                    .colors
                    .get(series_idx)
                    .unwrap_or(&self.colors[0])
                    .to_rgba();

                match self.orientation {
                    BarOrientation::Vertical => {
                        // Vertical bars - start from the bottom (cumulative) and go up by value
                        let x_left = x_pos - self.bar_width / 2.0;
                        let y_bottom = cumulative; // Bottom of this segment

                        let bottom_left = Point2D::new(x_left, y_bottom);

                        canvas.draw_rectangle(&bottom_left, self.bar_width, value, &color)?;
                    }
                    BarOrientation::Horizontal => {
                        // Horizontal bars - start from left (cumulative) and go right by value
                        let y_center = x_pos;
                        let y_bottom = y_center - self.bar_width / 2.0;
                        let x_left = cumulative;

                        let bottom_left = Point2D::new(x_left, y_bottom);

                        canvas.draw_rectangle(&bottom_left, value, self.bar_width, &color)?;
                    }
                }

                cumulative += value;
            }
        }

        Ok(())
    }
}

impl StackedBarPlot {
    /// Get bounds for this stacked bar plot
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.categories.is_empty() || self.series.is_empty() {
            return None;
        }

        let n_categories = self.categories.len();

        // Calculate max cumulative value
        let mut max_cumulative: f64 = 0.0;
        for cat_idx in 0..n_categories {
            let mut cumulative: f64 = 0.0;
            for series_data in &self.series {
                cumulative += series_data[cat_idx];
            }
            max_cumulative = max_cumulative.max(cumulative);
        }

        match self.orientation {
            BarOrientation::Vertical => {
                // X: -0.5 to n_categories - 0.5
                // Y: 0 to max_cumulative
                Some(Bounds::new(
                    -0.5,
                    n_categories as f64 - 0.5,
                    0.0,
                    max_cumulative,
                ))
            }
            BarOrientation::Horizontal => {
                // X: 0 to max_cumulative
                // Y: -0.5 to n_categories - 0.5
                Some(Bounds::new(
                    0.0,
                    max_cumulative,
                    -0.5,
                    n_categories as f64 - 0.5,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stacked_bar_creation() {
        let categories = vec!["A", "B", "C"];
        let series = vec![vec![10.0, 20.0, 15.0], vec![5.0, 10.0, 8.0]];
        let stacked = StackedBarPlot::new(categories, series).unwrap();
        assert!(stacked.bounds().is_some());
    }

    #[test]
    fn test_stacked_bar_empty_categories() {
        let categories: Vec<&str> = vec![];
        let series = vec![vec![10.0, 20.0]];
        let result = StackedBarPlot::new(categories, series);
        assert!(result.is_err());
    }

    #[test]
    fn test_stacked_bar_empty_series() {
        let categories = vec!["A", "B"];
        let series: Vec<Vec<f64>> = vec![];
        let result = StackedBarPlot::new(categories, series);
        assert!(result.is_err());
    }

    #[test]
    fn test_stacked_bar_mismatched_lengths() {
        let categories = vec!["A", "B", "C"];
        let series = vec![vec![10.0, 20.0], vec![5.0, 10.0, 8.0]]; // Mismatched
        let result = StackedBarPlot::new(categories, series);
        assert!(result.is_err());
    }

    #[test]
    fn test_stacked_bar_bounds() {
        let categories = vec!["A", "B"];
        let series = vec![vec![10.0, 20.0], vec![5.0, 10.0]];
        let stacked = StackedBarPlot::new(categories, series).unwrap();
        let bounds = stacked.bounds().unwrap();

        // Vertical: x from -0.5 to 1.5, y from 0 to 30
        assert_eq!(bounds.x_min, -0.5);
        assert_eq!(bounds.x_max, 1.5);
        assert_eq!(bounds.y_min, 0.0);
        assert_eq!(bounds.y_max, 30.0); // max(10+5, 20+10) = 30
    }

    #[test]
    fn test_stacked_bar_horizontal_bounds() {
        let categories = vec!["A", "B"];
        let series = vec![vec![10.0, 20.0], vec![5.0, 10.0]];
        let stacked = StackedBarPlot::new(categories, series)
            .unwrap()
            .orientation(BarOrientation::Horizontal);
        let bounds = stacked.bounds().unwrap();

        // Horizontal: x from 0 to 30, y from -0.5 to 1.5
        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 30.0);
        assert_eq!(bounds.y_min, -0.5);
        assert_eq!(bounds.y_max, 1.5);
    }
}
