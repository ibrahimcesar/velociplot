//! Histogram implementation with automatic binning

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable};
use crate::error::Result;
use crate::legend::LegendEntry;

/// Binning strategy for histograms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinStrategy {
    /// Automatic bin selection using Sturges' formula
    Auto,
    /// Square root rule (good for larger datasets)
    SquareRoot,
    /// Rice rule
    Rice,
    /// Scott's normal reference rule
    Scott,
}

/// Histogram for visualizing data distributions
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::plots::histogram::{Histogram, BinStrategy};
/// let data = vec![1.0, 2.0, 2.5, 3.0, 3.2, 3.5, 4.0, 5.0, 5.5, 6.0];
///
/// let hist = Histogram::new(&data)
///     .bins(10)
///     .color(Color::from_hex("#2ecc71").unwrap())
///     .label("Distribution");
/// ```
pub struct Histogram {
    data: Vec<f64>,
    bin_count: Option<usize>,
    bin_edges: Option<Vec<f64>>,
    bin_strategy: BinStrategy,
    color: Color,
    label: Option<String>,
    show_outline: bool,
    outline_color: Color,
}

impl Histogram {
    /// Create a new histogram from raw data
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::plots::histogram::Histogram;
    /// let data = vec![1.0, 2.0, 3.0, 2.5, 3.5, 4.0];
    /// let hist = Histogram::new(&data);
    /// ```
    #[must_use]
    pub fn new(data: &[f64]) -> Self {
        Self {
            data: data.to_vec(),
            bin_count: None,
            bin_edges: None,
            bin_strategy: BinStrategy::Auto,
            color: Color::from_hex("#2ecc71").unwrap_or(Color::GREEN),
            label: None,
            show_outline: true,
            outline_color: Color::BLACK,
        }
    }

    /// Set the number of bins explicitly
    #[must_use]
    pub fn bins(mut self, count: usize) -> Self {
        self.bin_count = Some(count.max(1));
        self
    }

    /// Set custom bin edges
    #[must_use]
    pub fn bin_edges(mut self, edges: Vec<f64>) -> Self {
        self.bin_edges = Some(edges);
        self
    }

    /// Set the binning strategy
    #[must_use]
    pub fn bin_strategy(mut self, strategy: BinStrategy) -> Self {
        self.bin_strategy = strategy;
        self
    }

    /// Set the histogram color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show bar outlines
    #[must_use]
    pub fn show_outline(mut self, show: bool) -> Self {
        self.show_outline = show;
        self
    }

    /// Set the label for the legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Calculate optimal number of bins based on strategy
    fn calculate_bin_count(&self) -> usize {
        if let Some(count) = self.bin_count {
            return count;
        }

        let n = self.data.len() as f64;

        match self.bin_strategy {
            BinStrategy::Auto => {
                // Sturges' formula: k = ceil(log2(n) + 1)
                (n.log2() + 1.0).ceil() as usize
            }
            BinStrategy::SquareRoot => {
                // Square root rule: k = ceil(sqrt(n))
                n.sqrt().ceil() as usize
            }
            BinStrategy::Rice => {
                // Rice rule: k = ceil(2 * n^(1/3))
                (2.0 * n.powf(1.0 / 3.0)).ceil() as usize
            }
            BinStrategy::Scott => {
                // Scott's rule uses bin width, convert to count
                if self.data.is_empty() {
                    return 10;
                }
                let min = self.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = self.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let range = max - min;

                // Calculate standard deviation
                let mean = self.data.iter().sum::<f64>() / n;
                let variance = self.data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
                let std_dev = variance.sqrt();

                // Scott's bin width: h = 3.5 * σ / n^(1/3)
                let bin_width = 3.5 * std_dev / n.powf(1.0 / 3.0);

                if bin_width > 0.0 {
                    (range / bin_width).ceil() as usize
                } else {
                    10
                }
            }
        }
    }

    /// Calculate bin edges
    fn calculate_bin_edges(&self) -> Vec<f64> {
        if let Some(ref edges) = self.bin_edges {
            return edges.clone();
        }

        if self.data.is_empty() {
            return vec![0.0, 1.0];
        }

        let min = self.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = self.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let bin_count = self.calculate_bin_count();
        let bin_width = (max - min) / bin_count as f64;

        (0..=bin_count)
            .map(|i| min + i as f64 * bin_width)
            .collect()
    }

    /// Calculate histogram counts
    fn calculate_counts(&self) -> Vec<usize> {
        let edges = self.calculate_bin_edges();
        let mut counts = vec![0; edges.len().saturating_sub(1)];

        for &value in &self.data {
            // Find which bin this value belongs to
            for i in 0..counts.len() {
                if value >= edges[i] && value < edges[i + 1] {
                    counts[i] += 1;
                    break;
                }
                // Handle the last bin edge inclusively
                if i == counts.len() - 1 && value == edges[i + 1] {
                    counts[i] += 1;
                    break;
                }
            }
        }

        counts
    }

    /// Get the bounding box for the histogram
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.data.is_empty() {
            return None;
        }

        let edges = self.calculate_bin_edges();
        let counts = self.calculate_counts();

        let x_min = edges.first().copied().unwrap_or(0.0);
        let x_max = edges.last().copied().unwrap_or(1.0);
        let y_max = counts.iter().max().copied().unwrap_or(1) as f64;

        Some(Bounds::new(x_min, x_max, 0.0, y_max))
    }

    /// Create a legend entry for this histogram
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label
            .as_ref()
            .map(|label| LegendEntry::new(label).color(self.color).line_width(2.0))
    }
}

impl Drawable for Histogram {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.data.is_empty() {
            return Ok(());
        }

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

        let edges = self.calculate_bin_edges();
        let counts = self.calculate_counts();

        // Find baseline (y=0) position
        let zero_y = value_to_pixel_y(0.0, bounds.y_min, bounds.y_max, pixel_min_y, pixel_max_y);

        let color = self.color.to_rgba();
        let outline_color = self.outline_color.to_rgba();

        // Draw each bin as a bar
        for (i, &count) in counts.iter().enumerate() {
            let x_left = edges[i];
            let x_right = edges[i + 1];
            let y_value = count as f64;

            let x1 = value_to_pixel_x(x_left, bounds.x_min, bounds.x_max, pixel_min_x, pixel_max_x);
            let x2 = value_to_pixel_x(
                x_right,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let y_top = value_to_pixel_y(
                y_value,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );

            // Draw filled bar
            let bar_height = (zero_y - y_top).abs();
            let steps = (bar_height.ceil() as i32).max(1);

            for j in 0..steps {
                let y = y_top + j as f32;
                if y <= zero_y {
                    canvas.draw_line_pixels(x1, y, x2, y, &color, 1.0)?;
                }
            }

            // Draw outline if enabled
            if self.show_outline {
                // Top
                canvas.draw_line_pixels(x1, y_top, x2, y_top, &outline_color, 1.0)?;
                // Right
                canvas.draw_line_pixels(x2, y_top, x2, zero_y, &outline_color, 1.0)?;
                // Left
                canvas.draw_line_pixels(x1, y_top, x1, zero_y, &outline_color, 1.0)?;
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

    #[test]
    fn test_histogram_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let hist = Histogram::new(&data);

        assert_eq!(hist.data.len(), 5);
    }

    #[test]
    fn test_bin_count_calculation() {
        let data: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let hist = Histogram::new(&data);

        let bin_count = hist.calculate_bin_count();
        assert!(bin_count > 0);
        assert!(bin_count < 50); // Reasonable for 100 points
    }

    #[test]
    fn test_custom_bins() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let hist = Histogram::new(&data).bins(5);

        assert_eq!(hist.bin_count, Some(5));
    }

    #[test]
    fn test_histogram_bounds() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let hist = Histogram::new(&data);

        let bounds = hist.bounds().unwrap();
        assert_eq!(bounds.x_min, 1.0);
        assert_eq!(bounds.x_max, 5.0);
        assert_eq!(bounds.y_min, 0.0);
    }

    #[test]
    fn test_different_bin_strategies() {
        let data: Vec<f64> = (0..50).map(|x| x as f64).collect();

        for strategy in [
            BinStrategy::Auto,
            BinStrategy::SquareRoot,
            BinStrategy::Rice,
            BinStrategy::Scott,
        ] {
            let hist = Histogram::new(&data).bin_strategy(strategy);
            let count = hist.calculate_bin_count();
            assert!(count > 0);
        }
    }
}
