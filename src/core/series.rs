//! Data series abstraction

use super::Point2D;
use crate::error::Result;

/// Trait for data that can be plotted
pub trait DataSeries {
    /// Get the number of data points
    fn len(&self) -> usize;

    /// Check if the series is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a point at the given index
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds
    fn point(&self, index: usize) -> Result<Point2D>;

    /// Get an iterator over all points
    fn points(&self) -> Box<dyn Iterator<Item = Point2D> + '_>;
}

/// A simple data series backed by vectors
#[derive(Debug, Clone)]
pub struct Series {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl Series {
    /// Create a new series from x and y vectors
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let x = vec![1.0, 2.0, 3.0];
    /// let y = vec![2.0, 4.0, 6.0];
    /// let series = Series::new(x, y).unwrap();
    /// assert_eq!(series.len(), 3);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if x and y have different lengths
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Result<Self> {
        if x.len() != y.len() {
            return Err(crate::error::Error::InvalidData(format!(
                "x and y must have same length (got {} and {})",
                x.len(),
                y.len()
            )));
        }
        Ok(Self { x, y })
    }

    /// Create a series from a slice of tuples
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let data = [(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)];
    /// let series = Series::from_tuples(&data);
    /// assert_eq!(series.len(), 3);
    /// ```
    #[must_use]
    pub fn from_tuples(data: &[(f64, f64)]) -> Self {
        let (x, y): (Vec<_>, Vec<_>) = data.iter().copied().unzip();
        Self { x, y }
    }

    /// Create a series from a function
    ///
    /// # Examples
    ///
    /// ```
    /// # use velociplot::prelude::*;
    /// let series = Series::from_function(0.0, 10.0, 100, |x| x * x);
    /// assert_eq!(series.len(), 100);
    /// ```
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn from_function<F>(x_min: f64, x_max: f64, n_points: usize, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let dx = (x_max - x_min) / (n_points - 1) as f64;
        let x: Vec<f64> = (0..n_points).map(|i| x_min + i as f64 * dx).collect();
        let y: Vec<f64> = x.iter().map(|&xi| f(xi)).collect();
        Self { x, y }
    }
}

impl DataSeries for Series {
    fn len(&self) -> usize {
        self.x.len()
    }

    fn point(&self, index: usize) -> Result<Point2D> {
        if index >= self.len() {
            return Err(crate::error::Error::InvalidData(format!(
                "Index {} out of bounds (len = {})",
                index,
                self.len()
            )));
        }
        Ok(Point2D::new(self.x[index], self.y[index]))
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point2D> + '_> {
        Box::new(
            self.x
                .iter()
                .zip(self.y.iter())
                .map(|(&x, &y)| Point2D::new(x, y)),
        )
    }
}
