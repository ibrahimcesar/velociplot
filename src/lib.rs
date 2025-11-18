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
pub mod core {
    //! Core abstractions for plotting
}

/// Figure and subplot management
pub mod figure {
    //! High-level figure composition
}

/// Plot types (line, scatter, bar, etc.)
pub mod plots {
    //! Different plot types
    
    /// Line plot
    pub mod line {}
    
    /// Scatter plot
    pub mod scatter {}
    
    /// Bar chart
    pub mod bar {}
    
    /// Histogram
    pub mod histogram {}
}

/// Axes, labels, and ticks
pub mod axes {
    //! Axis configuration and rendering
}

/// Color handling and colormaps
pub mod color {
    //! Color definitions and palettes
}

/// Text rendering and LaTeX support
pub mod text {
    //! Text and math rendering
}

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
    
    pub use crate::core::*;
    pub use crate::figure::*;
    pub use crate::plots::*;
    pub use crate::error::{Error, Result};
}

// Re-exports
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
