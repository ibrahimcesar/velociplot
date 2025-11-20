//! # velociplot 🦖
//!
//! > Scientific plotting at velociraptor speed
//!
//! **velociplot** is a fast, publication-quality plotting library for Rust.
//! Quick, precise, and deadly effective for creating scientific figures.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use velociplot::prelude::*;
//!
//! // Simple line plot
//! plot()
//!     .line(x, y)
//!     .xlabel("Time (s)")
//!     .ylabel("Temperature (K)")
//!     .save("figure.png")?;
//! ```
//!
//! ## Features
//!
//! - 🦖 **Blazingly Fast** - High-performance rendering
//! - 📊 **Publication Quality** - LaTeX, vector output, precise control
//! - 📐 **Scientific Plots** - Line, scatter, histogram, heatmap, contour
//! - 🎨 **Beautiful Defaults** - Perceptually uniform colormaps
//! - 🔧 **Ergonomic API** - Simple for basics, powerful for complex

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// Core plotting types and traits
pub mod core;

/// Rendering backends
pub mod backend;

/// Figure and subplot management
pub mod figure {
    //! High-level figure composition
}

/// Plot types (line, scatter, bar, etc.)
pub mod plots;

/// Color definitions and utilities
pub mod color;

/// Axes, labels, and ticks
pub mod axes;

/// Legend rendering
pub mod legend;

/// Text rendering and math notation support
pub mod text;

/// Data integration (ndarray, polars, etc.)
pub mod integration;

/// Output formats (PNG, PDF, SVG)
pub mod output {
    //! Various output format writers
}

/// Style presets and themes
pub mod style {
    //! Styling and theming
}

/// Error types
pub mod error {
    //! Error definitions

    use thiserror::Error;

    /// Main error type for velociplot
    #[derive(Error, Debug)]
    pub enum Error {
        /// IO error
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),

        /// Invalid data
        #[error("Invalid data: {0}")]
        InvalidData(String),

        /// Rendering error
        #[error("Rendering error: {0}")]
        Rendering(String),
    }

    /// Result type alias
    pub type Result<T> = std::result::Result<T, Error>;
}

/// Prelude for convenient imports
pub mod prelude {
    //! Convenient re-exports
    //!
    //! ```rust
    //! use velociplot::prelude::*;
    //! ```

    pub use crate::axes::{Axis, AxisPosition, LabelAlignment};
    pub use crate::color::{Color, Colormap, Palette};
    pub use crate::core::{Bounds, Canvas, DataSeries, Drawable, Point2D, Series};
    pub use crate::error::{Error, Result};
    pub use crate::legend::{
        BarLegend, BarLegendPosition, HorizontalAlignment, Legend, LegendEntry, LegendPosition,
        LegendShape, MarkerShape as LegendMarker,
    };
    pub use crate::plots::area::{AreaPlot, StackedAreaPlot};
    pub use crate::plots::bar::{BarOrientation, BarPlot};
    pub use crate::plots::boxplot::{BoxPlot, OutlierMethod};
    pub use crate::plots::bubble::BubbleChart;
    pub use crate::plots::datelistplot::{DateListPlot, DateListStyle};
    pub use crate::plots::heatmap::{Heatmap, ValueFormat};
    pub use crate::plots::histogram::{BinStrategy, Histogram};
    pub use crate::plots::line::LinePlot;
    pub use crate::plots::qq::{Distribution, PPPlot, QQPlot};
    pub use crate::plots::scatter::{MarkerShape, MarkerStyle, ScatterPlot};
    pub use crate::plots::stacked_bar::StackedBarPlot;
    pub use crate::plots::timeline::{Timeline, TimelineOrientation};
    pub use crate::plots::treemap::Treemap;
    pub use crate::plots::violin::{Kernel, ViolinPlot};
    pub use crate::text::{parse_math, MathNotation};

    #[cfg(feature = "raster")]
    pub use crate::backend::SkiaCanvas;

    pub use crate::backend::SvgCanvas;
}

// Re-exports
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
