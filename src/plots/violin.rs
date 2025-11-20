//! Violin plot implementation combining density estimation and box plot

use crate::color::Color;
use crate::core::{Bounds, Canvas, Drawable};
use crate::error::Result;
use crate::legend::LegendEntry;

/// Kernel function for density estimation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kernel {
    /// Gaussian (normal) kernel - most common
    Gaussian,
    /// Epanechnikov kernel - optimal for minimizing MSE
    Epanechnikov,
    /// Uniform (rectangular) kernel
    Uniform,
}

impl Kernel {
    /// Evaluate the kernel at distance u
    fn evaluate(&self, u: f64) -> f64 {
        match self {
            Kernel::Gaussian => {
                // Gaussian: (1/sqrt(2π)) * exp(-u²/2)
                (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-0.5 * u * u).exp()
            }
            Kernel::Epanechnikov => {
                // Epanechnikov: (3/4) * (1 - u²) for |u| ≤ 1
                if u.abs() <= 1.0 {
                    0.75 * (1.0 - u * u)
                } else {
                    0.0
                }
            }
            Kernel::Uniform => {
                // Uniform: 0.5 for |u| ≤ 1
                if u.abs() <= 1.0 {
                    0.5
                } else {
                    0.0
                }
            }
        }
    }
}

/// Violin plot configuration
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// # use velociplot::plots::violin::{ViolinPlot, Kernel};
/// let data = vec![1.0, 2.0, 2.5, 3.0, 3.2, 3.5, 4.0, 5.0];
///
/// let violin = ViolinPlot::new(&data, 0.5) // position at x=0.5
///     .show_box(true)
///     .color(Color::from_hex("#9b59b6").unwrap())
///     .label("Distribution");
/// ```
pub struct ViolinPlot {
    data: Vec<f64>,
    position: f64,
    width: f64,
    color: Color,
    show_box: bool,
    show_median: bool,
    kernel: Kernel,
    bandwidth: Option<f64>,
    label: Option<String>,
    n_points: usize,
}

impl ViolinPlot {
    /// Create a new violin plot
    ///
    /// # Arguments
    /// * `data` - The data points
    /// * `position` - X-axis position for the violin
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::plots::violin::ViolinPlot;
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let violin = ViolinPlot::new(&data, 1.0);
    /// ```
    #[must_use]
    pub fn new(data: &[f64], position: f64) -> Self {
        Self {
            data: data.to_vec(),
            position,
            width: 0.4,
            color: Color::from_hex("#9b59b6").unwrap_or(Color::from_hex("#3498db").unwrap()),
            show_box: true,
            show_median: true,
            kernel: Kernel::Gaussian,
            bandwidth: None,
            label: None,
            n_points: 100,
        }
    }

    /// Set the maximum width of the violin
    #[must_use]
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Set the violin color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set whether to show the box plot overlay
    #[must_use]
    pub fn show_box(mut self, show: bool) -> Self {
        self.show_box = show;
        self
    }

    /// Set whether to show median line
    #[must_use]
    pub fn show_median(mut self, show: bool) -> Self {
        self.show_median = show;
        self
    }

    /// Set the kernel function for density estimation
    #[must_use]
    pub fn kernel(mut self, kernel: Kernel) -> Self {
        self.kernel = kernel;
        self
    }

    /// Set custom bandwidth (if None, uses Scott's rule)
    #[must_use]
    pub fn bandwidth(mut self, h: f64) -> Self {
        self.bandwidth = Some(h);
        self
    }

    /// Set the label for legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Calculate statistics for the data
    fn calculate_stats(&self) -> Stats {
        if self.data.is_empty() {
            return Stats::default();
        }

        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = sorted.len();
        let min = sorted[0];
        let max = sorted[n - 1];

        let q1 = percentile(&sorted, 25.0);
        let median = percentile(&sorted, 50.0);
        let q3 = percentile(&sorted, 75.0);

        let iqr = q3 - q1;

        // Whiskers extend to 1.5 * IQR or to data extremes
        let lower_fence = q1 - 1.5 * iqr;
        let upper_fence = q3 + 1.5 * iqr;

        let lower_whisker = sorted
            .iter()
            .find(|&&x| x >= lower_fence)
            .copied()
            .unwrap_or(min);
        let upper_whisker = sorted
            .iter()
            .rev()
            .find(|&&x| x <= upper_fence)
            .copied()
            .unwrap_or(max);

        Stats {
            min,
            max,
            q1,
            median,
            q3,
            lower_whisker,
            upper_whisker,
        }
    }

    /// Calculate kernel density estimate
    fn calculate_kde(&self) -> Vec<(f64, f64)> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let stats = self.calculate_stats();
        let range = stats.max - stats.min;

        // Extend range slightly for better visualization
        let y_min = stats.min - 0.1 * range;
        let y_max = stats.max + 0.1 * range;

        // Calculate bandwidth using Scott's rule if not provided
        let bandwidth = self.bandwidth.unwrap_or_else(|| {
            let n = self.data.len() as f64;
            let std_dev = calculate_std_dev(&self.data);
            1.06 * std_dev * n.powf(-0.2)
        });

        let mut kde_points = Vec::with_capacity(self.n_points);

        for i in 0..self.n_points {
            let y = y_min + (i as f64 / (self.n_points - 1) as f64) * (y_max - y_min);

            // Calculate density at this point
            let density: f64 = self
                .data
                .iter()
                .map(|&x| {
                    let u = (y - x) / bandwidth;
                    self.kernel.evaluate(u)
                })
                .sum::<f64>()
                / (self.data.len() as f64 * bandwidth);

            kde_points.push((y, density));
        }

        // Normalize to maximum density for scaling
        if let Some(&(_, max_density)) = kde_points
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        {
            if max_density > 0.0 {
                for (_, density) in &mut kde_points {
                    *density /= max_density;
                }
            }
        }

        kde_points
    }

    /// Get bounds for the violin plot
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.data.is_empty() {
            return None;
        }

        let stats = self.calculate_stats();
        let range = stats.max - stats.min;

        Some(Bounds::new(
            self.position - self.width,
            self.position + self.width,
            stats.min - 0.1 * range,
            stats.max + 0.1 * range,
        ))
    }

    /// Create legend entry
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label
            .as_ref()
            .map(|label| LegendEntry::new(label).color(self.color).line_width(2.0))
    }
}

impl Drawable for ViolinPlot {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.data.is_empty() {
            return Ok(());
        }

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

        let kde_points = self.calculate_kde();
        let stats = self.calculate_stats();

        let color = self.color.to_rgba();

        // Draw violin shape (density curves on both sides)
        for i in 0..kde_points.len() - 1 {
            let (y1, d1) = kde_points[i];
            let (y2, d2) = kde_points[i + 1];

            // Scale density to width
            let width1 = d1 * self.width;
            let width2 = d2 * self.width;

            let y1_pixel =
                value_to_pixel_y(y1, bounds.y_min, bounds.y_max, pixel_min_y, pixel_max_y);
            let y2_pixel =
                value_to_pixel_y(y2, bounds.y_min, bounds.y_max, pixel_min_y, pixel_max_y);

            let x_center = value_to_pixel_x(
                self.position,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let x1_left = value_to_pixel_x(
                self.position - width1,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let x2_left = value_to_pixel_x(
                self.position - width2,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let x1_right = value_to_pixel_x(
                self.position + width1,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let x2_right = value_to_pixel_x(
                self.position + width2,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );

            // Fill left side
            let steps = ((x_center - x1_left.min(x2_left)).abs().ceil() as i32).max(1);
            for step in 0..steps {
                let t = step as f32 / steps as f32;
                let x = x1_left + t * (x2_left - x1_left);
                let y_start = y1_pixel + t * (y2_pixel - y1_pixel);
                canvas.draw_line_pixels(x, y_start, x_center, y_start, &color, 1.0)?;
            }

            // Fill right side
            let steps = ((x1_right.max(x2_right) - x_center).abs().ceil() as i32).max(1);
            for step in 0..steps {
                let t = step as f32 / steps as f32;
                let x = x_center + t * (x1_right - x_center);
                let y_start = y1_pixel + t * (y2_pixel - y1_pixel);
                canvas.draw_line_pixels(x_center, y_start, x, y_start, &color, 1.0)?;
            }

            // Draw outline
            canvas.draw_line_pixels(x1_left, y1_pixel, x2_left, y2_pixel, &[0, 0, 0, 255], 1.0)?;
            canvas.draw_line_pixels(
                x1_right,
                y1_pixel,
                x2_right,
                y2_pixel,
                &[0, 0, 0, 255],
                1.0,
            )?;
        }

        // Draw box plot overlay if enabled
        if self.show_box {
            let box_width = self.width * 0.15;
            let x_center = value_to_pixel_x(
                self.position,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let x_left = value_to_pixel_x(
                self.position - box_width,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let x_right = value_to_pixel_x(
                self.position + box_width,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );

            let q1_y = value_to_pixel_y(
                stats.q1,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );
            let median_y = value_to_pixel_y(
                stats.median,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );
            let q3_y = value_to_pixel_y(
                stats.q3,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );
            let lower_whisker_y = value_to_pixel_y(
                stats.lower_whisker,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );
            let upper_whisker_y = value_to_pixel_y(
                stats.upper_whisker,
                bounds.y_min,
                bounds.y_max,
                pixel_min_y,
                pixel_max_y,
            );

            let box_color = [255, 255, 255, 200];

            // Draw IQR box
            for y in (q3_y as i32..q1_y as i32).step_by(1) {
                canvas.draw_line_pixels(x_left, y as f32, x_right, y as f32, &box_color, 1.0)?;
            }

            // Box outline
            canvas.draw_line_pixels(x_left, q1_y, x_right, q1_y, &[0, 0, 0, 255], 1.5)?;
            canvas.draw_line_pixels(x_left, q3_y, x_right, q3_y, &[0, 0, 0, 255], 1.5)?;
            canvas.draw_line_pixels(x_left, q1_y, x_left, q3_y, &[0, 0, 0, 255], 1.5)?;
            canvas.draw_line_pixels(x_right, q1_y, x_right, q3_y, &[0, 0, 0, 255], 1.5)?;

            // Median line
            if self.show_median {
                canvas.draw_line_pixels(
                    x_left,
                    median_y,
                    x_right,
                    median_y,
                    &[0, 0, 0, 255],
                    2.0,
                )?;
            }

            // Whiskers
            canvas.draw_line_pixels(
                x_center,
                q1_y,
                x_center,
                lower_whisker_y,
                &[0, 0, 0, 255],
                1.0,
            )?;
            canvas.draw_line_pixels(
                x_center,
                q3_y,
                x_center,
                upper_whisker_y,
                &[0, 0, 0, 255],
                1.0,
            )?;

            // Whisker caps
            let cap_width = box_width * 0.5;
            let cap_left = value_to_pixel_x(
                self.position - cap_width,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            let cap_right = value_to_pixel_x(
                self.position + cap_width,
                bounds.x_min,
                bounds.x_max,
                pixel_min_x,
                pixel_max_x,
            );
            canvas.draw_line_pixels(
                cap_left,
                lower_whisker_y,
                cap_right,
                lower_whisker_y,
                &[0, 0, 0, 255],
                1.0,
            )?;
            canvas.draw_line_pixels(
                cap_left,
                upper_whisker_y,
                cap_right,
                upper_whisker_y,
                &[0, 0, 0, 255],
                1.0,
            )?;
        }

        Ok(())
    }
}

/// Statistical summary
#[derive(Debug, Clone, Copy, Default)]
struct Stats {
    min: f64,
    max: f64,
    q1: f64,
    median: f64,
    q3: f64,
    lower_whisker: f64,
    upper_whisker: f64,
}

/// Calculate percentile from sorted data
fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    let n = sorted_data.len();
    let rank = p / 100.0 * (n - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let fraction = rank - lower as f64;

    if lower == upper {
        sorted_data[lower]
    } else {
        sorted_data[lower] * (1.0 - fraction) + sorted_data[upper] * fraction
    }
}

/// Calculate standard deviation
fn calculate_std_dev(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
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
    fn test_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&data, 0.0), 1.0);
        assert_eq!(percentile(&data, 50.0), 3.0);
        assert_eq!(percentile(&data, 100.0), 5.0);
    }

    #[test]
    fn test_std_dev() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std = calculate_std_dev(&data);
        assert!((std - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_violin_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let violin = ViolinPlot::new(&data, 1.0);

        assert_eq!(violin.data.len(), 5);
        assert_eq!(violin.position, 1.0);
    }

    #[test]
    fn test_violin_bounds() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let violin = ViolinPlot::new(&data, 1.0);

        let bounds = violin.bounds();
        assert!(bounds.is_some());
    }

    #[test]
    fn test_kernel_evaluation() {
        assert!((Kernel::Gaussian.evaluate(0.0) - 0.3989).abs() < 0.01);
        assert_eq!(Kernel::Epanechnikov.evaluate(2.0), 0.0);
        assert_eq!(Kernel::Uniform.evaluate(0.5), 0.5);
    }
}
