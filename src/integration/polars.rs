//! Polars integration for velociplot
//!
//! Provides convenient conversions from Polars DataFrames to velociplot Series.

use crate::core::Series;
use crate::error::{Error, Result};
use polars::prelude::*;

/// Extension trait for converting Polars DataFrames to Series
pub trait PolarsSeries {
    /// Convert a DataFrame column to a Series with automatic x-coordinates (0, 1, 2, ...)
    ///
    /// # Arguments
    ///
    /// * `y_col` - Name of the column to use as y values
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "polars-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::PolarsSeries;
    /// use polars::prelude::*;
    ///
    /// let df = df! {
    ///     "y" => &[1.0, 4.0, 9.0, 16.0],
    /// }.unwrap();
    ///
    /// let series = df.to_series("y").unwrap();
    /// # }
    /// ```
    fn to_series(&self, y_col: &str) -> Result<Series>;

    /// Convert DataFrame columns to a Series with custom x and y coordinates
    ///
    /// # Arguments
    ///
    /// * `x_col` - Name of the column to use as x values
    /// * `y_col` - Name of the column to use as y values
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "polars-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::PolarsSeries;
    /// use polars::prelude::*;
    ///
    /// let df = df! {
    ///     "x" => &[0.0, 1.0, 2.0, 3.0],
    ///     "y" => &[1.0, 4.0, 9.0, 16.0],
    /// }.unwrap();
    ///
    /// let series = df.to_series_xy("x", "y").unwrap();
    /// # }
    /// ```
    fn to_series_xy(&self, x_col: &str, y_col: &str) -> Result<Series>;

    /// Convert multiple DataFrame columns to separate Series
    ///
    /// First column is used as x-coordinates, remaining specified columns become separate series
    ///
    /// # Arguments
    ///
    /// * `x_col` - Name of the column to use as x values
    /// * `y_cols` - Names of columns to use as separate y series
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "polars-support")]
    /// # {
    /// use velociplot::prelude::*;
    /// use velociplot::integration::PolarsSeries;
    /// use polars::prelude::*;
    ///
    /// let df = df! {
    ///     "x" => &[0.0, 1.0, 2.0],
    ///     "y1" => &[1.0, 4.0, 9.0],
    ///     "y2" => &[2.0, 3.0, 4.0],
    /// }.unwrap();
    ///
    /// let series_list = df.to_multi_series("x", &["y1", "y2"]).unwrap();
    /// assert_eq!(series_list.len(), 2);
    /// # }
    /// ```
    fn to_multi_series(&self, x_col: &str, y_cols: &[&str]) -> Result<Vec<Series>>;
}

impl PolarsSeries for DataFrame {
    fn to_series(&self, y_col: &str) -> Result<Series> {
        let y_series = self
            .column(y_col)
            .map_err(|e| Error::InvalidData(format!("Column '{}' not found: {}", y_col, e)))?;

        let y_values = y_series
            .f64()
            .map_err(|e| Error::InvalidData(format!("Column '{}' is not f64: {}", y_col, e)))?
            .into_iter()
            .map(|v| v.ok_or_else(|| Error::InvalidData("Null value in y column".into())))
            .collect::<Result<Vec<f64>>>()?;

        let x_values: Vec<f64> = (0..y_values.len()).map(|i| i as f64).collect();

        Series::new(x_values, y_values)
    }

    fn to_series_xy(&self, x_col: &str, y_col: &str) -> Result<Series> {
        let x_series = self
            .column(x_col)
            .map_err(|e| Error::InvalidData(format!("Column '{}' not found: {}", x_col, e)))?;

        let y_series = self
            .column(y_col)
            .map_err(|e| Error::InvalidData(format!("Column '{}' not found: {}", y_col, e)))?;

        let x_values = x_series
            .f64()
            .map_err(|e| Error::InvalidData(format!("Column '{}' is not f64: {}", x_col, e)))?
            .into_iter()
            .map(|v| v.ok_or_else(|| Error::InvalidData("Null value in x column".into())))
            .collect::<Result<Vec<f64>>>()?;

        let y_values = y_series
            .f64()
            .map_err(|e| Error::InvalidData(format!("Column '{}' is not f64: {}", y_col, e)))?
            .into_iter()
            .map(|v| v.ok_or_else(|| Error::InvalidData("Null value in y column".into())))
            .collect::<Result<Vec<f64>>>()?;

        if x_values.len() != y_values.len() {
            return Err(Error::InvalidData(
                "X and Y columns must have the same length".into(),
            ));
        }

        Series::new(x_values, y_values)
    }

    fn to_multi_series(&self, x_col: &str, y_cols: &[&str]) -> Result<Vec<Series>> {
        if y_cols.is_empty() {
            return Err(Error::InvalidData(
                "At least one y column must be specified".into(),
            ));
        }

        let x_series = self
            .column(x_col)
            .map_err(|e| Error::InvalidData(format!("Column '{}' not found: {}", x_col, e)))?;

        let x_values = x_series
            .f64()
            .map_err(|e| Error::InvalidData(format!("Column '{}' is not f64: {}", x_col, e)))?
            .into_iter()
            .map(|v| v.ok_or_else(|| Error::InvalidData("Null value in x column".into())))
            .collect::<Result<Vec<f64>>>()?;

        let mut result = Vec::new();

        for &y_col in y_cols {
            let y_series = self
                .column(y_col)
                .map_err(|e| Error::InvalidData(format!("Column '{}' not found: {}", y_col, e)))?;

            let y_values = y_series
                .f64()
                .map_err(|e| Error::InvalidData(format!("Column '{}' is not f64: {}", y_col, e)))?
                .into_iter()
                .map(|v| v.ok_or_else(|| Error::InvalidData("Null value in y column".into())))
                .collect::<Result<Vec<f64>>>()?;

            if x_values.len() != y_values.len() {
                return Err(Error::InvalidData(format!(
                    "X column and '{}' must have the same length",
                    y_col
                )));
            }

            result.push(Series::new(x_values.clone(), y_values)?);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::DataSeries;

    #[test]
    fn test_df_to_series() {
        let df = df! {
            "y" => &[1.0, 4.0, 9.0, 16.0],
        }
        .unwrap();

        let series = df.to_series("y").unwrap();
        assert_eq!(series.len(), 4);
    }

    #[test]
    fn test_df_to_series_xy() {
        let df = df! {
            "x" => &[0.0, 1.0, 2.0, 3.0],
            "y" => &[1.0, 4.0, 9.0, 16.0],
        }
        .unwrap();

        let series = df.to_series_xy("x", "y").unwrap();
        assert_eq!(series.len(), 4);
    }

    #[test]
    fn test_df_to_multi_series() {
        let df = df! {
            "x" => &[0.0, 1.0, 2.0],
            "y1" => &[1.0, 4.0, 9.0],
            "y2" => &[2.0, 3.0, 4.0],
        }
        .unwrap();

        let series_list = df.to_multi_series("x", &["y1", "y2"]).unwrap();
        assert_eq!(series_list.len(), 2);
    }

    #[test]
    fn test_missing_column() {
        let df = df! {
            "y" => &[1.0, 4.0, 9.0],
        }
        .unwrap();

        let result = df.to_series("missing");
        assert!(result.is_err());
    }
}
