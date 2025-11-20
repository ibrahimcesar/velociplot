//! Data integration modules
//!
//! Support for creating Series from various data sources like ndarray, polars, etc.

#[cfg(feature = "ndarray-support")]
pub mod ndarray;

#[cfg(feature = "ndarray-support")]
pub use self::ndarray::*;

#[cfg(feature = "polars-support")]
pub mod polars;

#[cfg(feature = "polars-support")]
pub use self::polars::*;
