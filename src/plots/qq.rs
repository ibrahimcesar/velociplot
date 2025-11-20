//! Q-Q (Quantile-Quantile) and P-P (Probability-Probability) plots for distribution testing

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable, Point2D, Series};
use crate::error::Result;
use crate::legend::LegendEntry;

/// Theoretical distribution for comparison
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distribution {
    /// Standard normal distribution (mean=0, std=1)
    Normal,
    /// Normal distribution with custom parameters
    NormalCustom {
        /// Mean of the distribution
        mean: f64,
        /// Standard deviation of the distribution
        std_dev: f64,
    },
    /// Uniform distribution [0, 1]
    Uniform,
    /// Uniform distribution with custom range
    UniformCustom {
        /// Minimum value
        min: f64,
        /// Maximum value
        max: f64,
    },
}

impl Distribution {
    /// Calculate the theoretical quantile for a given probability
    fn quantile(&self, p: f64) -> f64 {
        match self {
            Distribution::Normal => {
                // Standard normal quantile (inverse CDF)
                normal_quantile(p)
            }
            Distribution::NormalCustom { mean, std_dev } => mean + std_dev * normal_quantile(p),
            Distribution::Uniform => p,
            Distribution::UniformCustom { min, max } => min + p * (max - min),
        }
    }

    /// Calculate the cumulative distribution function (CDF) value
    fn cdf(&self, x: f64) -> f64 {
        match self {
            Distribution::Normal => normal_cdf(x),
            Distribution::NormalCustom { mean, std_dev } => normal_cdf((x - mean) / std_dev),
            Distribution::Uniform => x.clamp(0.0, 1.0),
            Distribution::UniformCustom { min, max } => ((x - min) / (max - min)).clamp(0.0, 1.0),
        }
    }
}

/// Q-Q Plot for comparing distributions via quantiles
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::plots::qq::{QQPlot, Distribution};
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
///
/// let qq = QQPlot::new(&data, Distribution::Normal)
///     .show_reference_line(true)
///     .color(Color::from_hex("#e74c3c").unwrap())
///     .label("Q-Q Plot");
/// ```
pub struct QQPlot {
    data: Vec<f64>,
    distribution: Distribution,
    color: Color,
    show_reference: bool,
    reference_color: Color,
    label: Option<String>,
}

impl QQPlot {
    /// Create a new Q-Q plot
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::plots::qq::{QQPlot, Distribution};
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let qq = QQPlot::new(&data, Distribution::Normal);
    /// ```
    #[must_use]
    pub fn new(data: &[f64], distribution: Distribution) -> Self {
        Self {
            data: data.to_vec(),
            distribution,
            color: Color::from_hex("#e74c3c").unwrap_or(Color::RED),
            show_reference: true,
            reference_color: Color::BLACK,
            label: None,
        }
    }

    /// Set the point color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show the reference line (y=x)
    #[must_use]
    pub fn show_reference_line(mut self, show: bool) -> Self {
        self.show_reference = show;
        self
    }

    /// Set the label for the legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Calculate Q-Q plot points
    fn calculate_points(&self) -> Vec<Point2D> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = sorted_data.len();
        let mut points = Vec::with_capacity(n);

        for (i, &value) in sorted_data.iter().enumerate() {
            // Calculate the theoretical quantile position
            // Using (i + 0.5) / n to avoid extreme values at 0 and 1
            let p = (i as f64 + 0.5) / n as f64;
            let theoretical = self.distribution.quantile(p);

            points.push(Point2D::new(theoretical, value));
        }

        points
    }

    /// Get the bounding box for the Q-Q plot
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        let points = self.calculate_points();
        if points.is_empty() {
            return None;
        }

        Some(Bounds::from_points(points))
    }

    /// Create a legend entry
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label
            .as_ref()
            .map(|label| LegendEntry::new(label).color(self.color).line_width(2.0))
    }

    /// Convert to a series for rendering
    #[must_use]
    pub fn to_series(&self) -> Series {
        let points = self.calculate_points();
        let (x, y): (Vec<_>, Vec<_>) = points.iter().map(|p| (p.x, p.y)).unzip();
        Series::new(x, y).unwrap_or_else(|_| Series::from_tuples(&[(0.0, 0.0)]))
    }
}

/// P-P Plot for comparing distributions via probabilities
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::plots::qq::{PPPlot, Distribution};
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
///
/// let pp = PPPlot::new(&data, Distribution::Normal)
///     .show_reference_line(true)
///     .color(Color::from_hex("#3498db").unwrap())
///     .label("P-P Plot");
/// ```
pub struct PPPlot {
    data: Vec<f64>,
    distribution: Distribution,
    color: Color,
    show_reference: bool,
    reference_color: Color,
    label: Option<String>,
}

impl PPPlot {
    /// Create a new P-P plot
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::plots::qq::{PPPlot, Distribution};
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let pp = PPPlot::new(&data, Distribution::Normal);
    /// ```
    #[must_use]
    pub fn new(data: &[f64], distribution: Distribution) -> Self {
        Self {
            data: data.to_vec(),
            distribution,
            color: Color::from_hex("#3498db").unwrap_or(Color::BLUE),
            show_reference: true,
            reference_color: Color::BLACK,
            label: None,
        }
    }

    /// Set the point color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show the reference line (y=x)
    #[must_use]
    pub fn show_reference_line(mut self, show: bool) -> Self {
        self.show_reference = show;
        self
    }

    /// Set the label for the legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Calculate P-P plot points
    fn calculate_points(&self) -> Vec<Point2D> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = sorted_data.len();
        let mut points = Vec::with_capacity(n);

        for (i, &value) in sorted_data.iter().enumerate() {
            // Empirical CDF value
            let empirical_p = (i as f64 + 1.0) / (n as f64 + 1.0);

            // Theoretical CDF value
            let theoretical_p = self.distribution.cdf(value);

            points.push(Point2D::new(theoretical_p, empirical_p));
        }

        points
    }

    /// Get the bounding box for the P-P plot
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        let points = self.calculate_points();
        if points.is_empty() {
            return None;
        }

        // P-P plots are always in [0, 1] x [0, 1]
        Some(Bounds::new(0.0, 1.0, 0.0, 1.0))
    }

    /// Create a legend entry
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label
            .as_ref()
            .map(|label| LegendEntry::new(label).color(self.color).line_width(2.0))
    }

    /// Convert to a series for rendering
    #[must_use]
    pub fn to_series(&self) -> Series {
        let points = self.calculate_points();
        let (x, y): (Vec<_>, Vec<_>) = points.iter().map(|p| (p.x, p.y)).unzip();
        Series::new(x, y).unwrap_or_else(|_| Series::from_tuples(&[(0.0, 0.0)]))
    }
}

impl Drawable for QQPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        use crate::plots::scatter::{MarkerShape, MarkerStyle, ScatterPlot};

        let series = self.to_series();

        // Draw reference line first (y = x) if enabled
        if self.show_reference {
            let bounds = canvas.bounds();
            let (width, height) = canvas.dimensions();

            let margin_left = 60.0;
            let margin_right = 20.0;
            let margin_top = 40.0;
            let margin_bottom = 40.0;

            let pixel_min_x = margin_left;
            let pixel_max_x = width as f32 - margin_right;
            let pixel_min_y = margin_top;
            let pixel_max_y = height as f32 - margin_bottom;

            // Draw diagonal reference line
            let x1 = value_to_pixel_x(
                bounds.x_min,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let y1 = value_to_pixel_y(
                bounds.y_min,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );
            let x2 = value_to_pixel_x(
                bounds.x_max,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let y2 = value_to_pixel_y(
                bounds.y_max,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );

            canvas.draw_line_pixels(x1, y1, x2, y2, &self.reference_color.to_rgba(), 1.5)?;
        }

        // Draw scatter points
        let scatter = ScatterPlot::new(series)
            .marker_shape(MarkerShape::Circle)
            .marker_size(4.0)
            .marker_style(MarkerStyle::Filled)
            .color(self.color);

        scatter.draw(canvas)?;

        Ok(())
    }
}

impl Drawable for PPPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        use crate::plots::scatter::{MarkerShape, MarkerStyle, ScatterPlot};

        let series = self.to_series();

        // Draw reference line first (y = x) if enabled
        if self.show_reference {
            let bounds = canvas.bounds();
            let (width, height) = canvas.dimensions();

            let margin_left = 60.0;
            let margin_right = 20.0;
            let margin_top = 40.0;
            let margin_bottom = 40.0;

            let pixel_min_x = margin_left;
            let pixel_max_x = width as f32 - margin_right;
            let pixel_min_y = margin_top;
            let pixel_max_y = height as f32 - margin_bottom;

            // Draw diagonal reference line
            let x1 = value_to_pixel_x(0.0, bounds.x_min, bounds.x_max, pixel_min_x, pixel_max_x);
            let y1 = value_to_pixel_y(0.0, bounds.y_min, bounds.y_max, pixel_min_y, pixel_max_y);
            let x2 = value_to_pixel_x(1.0, bounds.x_min, bounds.x_max, pixel_min_x, pixel_max_x);
            let y2 = value_to_pixel_y(1.0, bounds.y_min, bounds.y_max, pixel_min_y, pixel_max_y);

            canvas.draw_line_pixels(x1, y1, x2, y2, &self.reference_color.to_rgba(), 1.5)?;
        }

        // Draw scatter points
        let scatter = ScatterPlot::new(series)
            .marker_shape(MarkerShape::Circle)
            .marker_size(4.0)
            .marker_style(MarkerStyle::Filled)
            .color(self.color);

        scatter.draw(canvas)?;

        Ok(())
    }
}

// Helper functions for normal distribution

/// Approximate inverse CDF (quantile function) for standard normal distribution
/// Uses rational approximation by Beasley-Springer-Moro
fn normal_quantile(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }

    let a = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383_577_518_672_69e2,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];

    let b = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];

    let c = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];

    let d = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];

    let p_low = 0.02425;
    let p_high = 1.0 - p_low;

    if p < p_low {
        // Rational approximation for lower region
        let q = (-2.0 * p.ln()).sqrt();
        (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    } else if p <= p_high {
        // Rational approximation for central region
        let q = p - 0.5;
        let r = q * q;
        (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
            / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
    } else {
        // Rational approximation for upper region
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
    }
}

/// Approximate CDF for standard normal distribution
/// Uses approximation by Zelen & Severo (1964)
fn normal_cdf(x: f64) -> f64 {
    let b1 = 0.319381530;
    let b2 = -0.356563782;
    let b3 = 1.781477937;
    let b4 = -1.821255978;
    let b5 = 1.330274429;
    let p = 0.2316419;
    let c = 0.39894228;

    if x >= 0.0 {
        let t = 1.0 / (1.0 + p * x);
        1.0 - c * (-x * x / 2.0).exp() * t * (t * (t * (t * (t * b5 + b4) + b3) + b2) + b1)
    } else {
        let t = 1.0 / (1.0 - p * x);
        c * (-x * x / 2.0).exp() * t * (t * (t * (t * (t * b5 + b4) + b3) + b2) + b1)
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
    pixel_max - normalized as f32 * pixel_range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cdf() {
        // Test known values
        assert!((normal_cdf(0.0) - 0.5).abs() < 0.001);
        assert!((normal_cdf(1.0) - 0.8413).abs() < 0.01);
        assert!((normal_cdf(-1.0) - 0.1587).abs() < 0.01);
    }

    #[test]
    fn test_normal_quantile() {
        assert!((normal_quantile(0.5) - 0.0).abs() < 0.001);
        assert!((normal_quantile(0.8413) - 1.0).abs() < 0.01);
        assert!((normal_quantile(0.1587) - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn test_qq_plot_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let qq = QQPlot::new(&data, Distribution::Normal);

        assert_eq!(qq.data.len(), 5);
        assert!(qq.show_reference);
    }

    #[test]
    fn test_pp_plot_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let pp = PPPlot::new(&data, Distribution::Normal);

        assert_eq!(pp.data.len(), 5);
        assert!(pp.show_reference);
    }

    #[test]
    fn test_qq_bounds() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let qq = QQPlot::new(&data, Distribution::Normal);

        let bounds = qq.bounds();
        assert!(bounds.is_some());
    }

    #[test]
    fn test_pp_bounds() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let pp = PPPlot::new(&data, Distribution::Normal);

        let bounds = pp.bounds().unwrap();
        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 1.0);
    }
}
