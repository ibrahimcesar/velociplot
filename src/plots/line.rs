//! Line plot implementation

use crate::color::Color;
use crate::core::{Bounds, Canvas, DataSeries, Drawable, Point2D};
use crate::error::Result;
use crate::legend::LegendEntry;

/// A line plot
pub struct LinePlot<D: DataSeries> {
    data: D,
    color: Color,
    line_width: f32,
    label: Option<String>,
}

impl<D: DataSeries> LinePlot<D> {
    /// Create a new line plot
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let data = Series::from_tuples(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
    /// let plot = LinePlot::new(data);
    /// ```
    #[must_use]
    pub fn new(data: D) -> Self {
        Self {
            data,
            color: Color::from_hex("#1f77b4").unwrap(), // default blue
            line_width: 2.0,
            label: None,
        }
    }

    /// Set the line color
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the line width
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set the plot label for legend
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the data bounds for this plot
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        if self.data.is_empty() {
            return None;
        }

        let points: Vec<Point2D> = self.data.points().collect();
        Some(Bounds::from_points(points))
    }

    /// Get a legend entry for this plot
    ///
    /// Returns None if the plot has no label
    #[must_use]
    pub fn legend_entry(&self) -> Option<LegendEntry> {
        self.label.as_ref().map(|label| {
            LegendEntry::new(label.clone())
                .color(self.color)
                .line_width(self.line_width)
        })
    }

    /// Get the label (if any)
    #[must_use]
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl<D: DataSeries> Drawable for LinePlot<D> {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        if self.data.is_empty() {
            return Ok(());
        }

        let points: Vec<Point2D> = self.data.points().collect();
        let color = self.color.to_rgba();

        // Draw lines connecting consecutive points
        for window in points.windows(2) {
            canvas.draw_line(&window[0], &window[1], &color, self.line_width)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Series;

    #[test]
    fn test_line_plot_creation() {
        let data = Series::from_tuples(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
        let plot = LinePlot::new(data)
            .color(Color::RED)
            .line_width(3.0)
            .label("Test");

        assert_eq!(plot.line_width, 3.0);
        assert_eq!(plot.label, Some("Test".to_string()));
    }

    #[test]
    fn test_line_plot_bounds() {
        let data = Series::from_tuples(&[(0.0, 0.0), (5.0, 10.0)]);
        let plot = LinePlot::new(data);
        let bounds = plot.bounds().unwrap();

        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 5.0);
        assert_eq!(bounds.y_min, 0.0);
        assert_eq!(bounds.y_max, 10.0);
    }
}
