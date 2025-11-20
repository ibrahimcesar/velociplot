//! Rendering backends

#[cfg(feature = "raster")]
pub mod skia;

#[cfg(feature = "raster")]
pub use skia::SkiaCanvas;

pub mod svg;
pub use svg::SvgCanvas;
