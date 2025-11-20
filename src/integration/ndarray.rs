//! ndarray integration for velociplot
//!
//! Provides convenient conversions from ndarray arrays to velociplot Series.

use crate::core::Series;
use crate::error::{Error, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};

/// Extension trait for converting ndarray arrays to Series
pub trait NdarraySeries {
    /// Convert a 1D array to a Series with automatic x-coordinates (0, 1, 2, ...)
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ndarray-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::NdarraySeries;
    /// use ndarray::array;
    ///
    /// let y = array![1.0, 4.0, 9.0, 16.0];
    /// let series = y.to_series().unwrap();
    /// # }
    /// ```
    fn to_series(&self) -> Result<Series>;

    /// Convert a 1D array to a Series with custom x-coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ndarray-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::NdarraySeries;
    /// use ndarray::array;
    ///
    /// let x = array![0.0, 1.0, 2.0, 3.0];
    /// let y = array![1.0, 4.0, 9.0, 16.0];
    /// let series = y.to_series_with_x(&x).unwrap();
    /// # }
    /// ```
    fn to_series_with_x(&self, x: &Self) -> Result<Series>;
}

impl NdarraySeries for Array1<f64> {
    fn to_series(&self) -> Result<Series> {
        let y = self.to_vec();
        let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();
        Series::new(x, y)
    }

    fn to_series_with_x(&self, x: &Self) -> Result<Series> {
        if self.len() != x.len() {
            return Err(Error::InvalidData(
                "X and Y arrays must have the same length".into(),
            ));
        }
        Series::new(x.to_vec(), self.to_vec())
    }
}

impl NdarraySeries for ArrayView1<'_, f64> {
    fn to_series(&self) -> Result<Series> {
        let y = self.to_vec();
        let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();
        Series::new(x, y)
    }

    fn to_series_with_x(&self, x: &Self) -> Result<Series> {
        if self.len() != x.len() {
            return Err(Error::InvalidData(
                "X and Y arrays must have the same length".into(),
            ));
        }
        Series::new(x.to_vec(), self.to_vec())
    }
}

/// Extension trait for converting 2D arrays to multiple Series
pub trait NdarrayMultiSeries {
    /// Convert columns of a 2D array to multiple Series
    ///
    /// First column is used as x-coordinates, remaining columns become separate series
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ndarray-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::NdarrayMultiSeries;
    /// use ndarray::array;
    ///
    /// let data = array![
    ///     [0.0, 1.0, 2.0],
    ///     [1.0, 4.0, 3.0],
    ///     [2.0, 9.0, 4.0],
    /// ];
    /// let series_list = data.to_multi_series().unwrap();
    /// assert_eq!(series_list.len(), 2); // 2 series (columns 1 and 2, using column 0 as x)
    /// # }
    /// ```
    fn to_multi_series(&self) -> Result<Vec<Series>>;

    /// Convert rows of a 2D array to multiple Series
    ///
    /// Each row becomes a separate series with automatic x-coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ndarray-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::NdarrayMultiSeries;
    /// use ndarray::array;
    ///
    /// let data = array![
    ///     [1.0, 4.0, 9.0],
    ///     [2.0, 5.0, 8.0],
    /// ];
    /// let series_list = data.rows_to_series().unwrap();
    /// assert_eq!(series_list.len(), 2); // 2 series (one per row)
    /// # }
    /// ```
    fn rows_to_series(&self) -> Result<Vec<Series>>;
}

impl NdarrayMultiSeries for Array2<f64> {
    fn to_multi_series(&self) -> Result<Vec<Series>> {
        if self.ncols() < 2 {
            return Err(Error::InvalidData(
                "Array must have at least 2 columns (x and at least one y)".into(),
            ));
        }

        let x_col = self.column(0).to_vec();
        let mut series_vec = Vec::new();

        for col_idx in 1..self.ncols() {
            let y_col = self.column(col_idx).to_vec();
            series_vec.push(Series::new(x_col.clone(), y_col)?);
        }

        Ok(series_vec)
    }

    fn rows_to_series(&self) -> Result<Vec<Series>> {
        let mut series_vec = Vec::new();

        for row in self.rows() {
            let y = row.to_vec();
            let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();
            series_vec.push(Series::new(x, y)?);
        }

        Ok(series_vec)
    }
}

impl NdarrayMultiSeries for ArrayView2<'_, f64> {
    fn to_multi_series(&self) -> Result<Vec<Series>> {
        if self.ncols() < 2 {
            return Err(Error::InvalidData(
                "Array must have at least 2 columns (x and at least one y)".into(),
            ));
        }

        let x_col = self.column(0).to_vec();
        let mut series_vec = Vec::new();

        for col_idx in 1..self.ncols() {
            let y_col = self.column(col_idx).to_vec();
            series_vec.push(Series::new(x_col.clone(), y_col)?);
        }

        Ok(series_vec)
    }

    fn rows_to_series(&self) -> Result<Vec<Series>> {
        let mut series_vec = Vec::new();

        for row in self.rows() {
            let y = row.to_vec();
            let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();
            series_vec.push(Series::new(x, y)?);
        }

        Ok(series_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::DataSeries;
    use ndarray::array;

    #[test]
    fn test_array1_to_series() {
        let y = array![1.0, 4.0, 9.0, 16.0];
        let series = y.to_series().unwrap();
        assert_eq!(series.len(), 4);
    }

    #[test]
    fn test_array1_to_series_with_x() {
        let x = array![0.0, 1.0, 2.0, 3.0];
        let y = array![1.0, 4.0, 9.0, 16.0];
        let series = y.to_series_with_x(&x).unwrap();
        assert_eq!(series.len(), 4);
    }

    #[test]
    fn test_array2_to_multi_series() {
        let data = array![[0.0, 1.0, 2.0], [1.0, 4.0, 3.0], [2.0, 9.0, 4.0],];
        let series_list = data.to_multi_series().unwrap();
        assert_eq!(series_list.len(), 2);
    }

    #[test]
    fn test_array2_rows_to_series() {
        let data = array![[1.0, 4.0, 9.0], [2.0, 5.0, 8.0],];
        let series_list = data.rows_to_series().unwrap();
        assert_eq!(series_list.len(), 2);
    }
}
