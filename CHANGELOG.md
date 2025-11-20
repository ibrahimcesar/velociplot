# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and documentation
- Production readiness assessment
- Contributing guidelines
- Changelog

## [0.0.1] - 2025-11-20

### Added

#### Core Features
- Core plotting architecture with `Drawable` trait
- 2D coordinate system with automatic bounds calculation
- Asymmetric padding support (`with_padding_top`, `with_padding_right`)
- Color system with hex, RGB, and named colors
- 5 perceptually uniform colormaps (Viridis, Plasma, Inferno, Coolwarm, Magma)
- Series abstraction for data handling
- Comprehensive error handling with `thiserror`

#### Plot Types (15 total)
- **LinePlot** - Basic line charts with customizable colors and widths
- **ScatterPlot** - Scatter plots with 6 marker shapes (Circle, Square, Triangle, Diamond, Plus, Cross)
- **BarPlot** - Vertical and horizontal bar charts
- **StackedBarPlot** - Cumulative stacked bars with proper baseline anchoring
- **Histogram** - Distribution visualization with automatic binning
- **BoxPlot** - Statistical summary boxes with quartiles and outliers
- **ViolinPlot** - Distribution density visualization
- **QQPlot** - Quantile-Quantile plots for distribution comparison
- **PPPlot** - Probability-Probability plots
- **Heatmap** - 2D density visualization with colormap support
- **BubbleChart** - 3-variable scatter plots with size encoding
- **AreaPlot** - Filled area charts with custom baseline
- **StackedAreaPlot** - Cumulative stacked area charts
- **Treemap** - Hierarchical data visualization with squarified layout
- **Timeline** - Chronological event visualization (horizontal/vertical)
- **DateListPlot** - Time series visualization with Line/Points/LinePoints styles

#### Rendering & Output
- Skia backend via `tiny-skia` for high-quality raster rendering
- PNG output format
- Custom font support (TTF files)
- Embedded JetBrains Mono font as default
- Text rendering with `fontdue`
- Anti-aliased graphics

#### Axes & Layout
- Axis system with configurable position (Top, Bottom, Left, Right)
- Automatic tick generation and formatting
- Axis labels and titles
- Grid lines (major)
- Multiple axis support

#### Legends (4 types)
- **LineLegend** - Styled lines with color and width
- **PointLegend** - Marker shapes with customization
- **SwatchLegend** - Filled color boxes
- **BarLegend** - Color gradient bars for heatmaps
- 9 positioning options (UpperLeft, UpperCenter, UpperRight, MiddleLeft, etc.)
- Automatic legend positioning
- Legend alignment below plots
- Separate legend rendering
- Customizable vertical alignment for text/line alignment

#### Data Integration
- **ndarray** support - Plot from n-dimensional arrays
- **polars** support - Plot from DataFrames
- Vector-based data input
- Function-based data generation
- Tuple data format

#### CLI Tool (Framework)
- Command-line interface structure with `clap`
- Plot type selection
- Column specification
- Output file control
- Version command

#### Examples & Documentation
- 30 comprehensive examples covering all plot types
- 98 generated example images
- Complete API documentation
- 67 doc tests
- Usage examples in all modules

#### Testing
- 101 unit tests
- 67 doc tests
- 168 total tests with 100% pass rate
- Data validation tests
- Error handling tests
- Boundary condition tests

#### Developer Experience
- Builder pattern for ergonomic API
- Method chaining for fluent interface
- `#[must_use]` attributes on builders
- Sensible defaults throughout
- Comprehensive error messages
- Feature flags for optional dependencies

### Performance
- Pure Rust implementation (no Python/C dependencies for core)
- LTO optimization in release builds
- Single compilation unit for maximum optimization
- Efficient rendering with tiny-skia

### Infrastructure
- MIT license
- README with quick start guide
- Cargo.toml with feature flags
- Example organization
- Documentation comments throughout

## Project Statistics

- **Source Lines:** 9,759
- **Example Lines:** 4,943
- **Total Tests:** 168 (100% pass)
- **Plot Types:** 15
- **Examples:** 30
- **Generated Images:** 98
- **Dependencies:** Minimal (thiserror, tiny-skia, fontdue, png)

## Platform Support

- Linux ✅
- macOS ✅
- Windows ✅ (untested)

## Known Limitations

### Not Yet Implemented
- CLI functionality (framework exists but not functional)
- SVG output (placeholder feature flag)
- PDF output (placeholder feature flag)
- LaTeX math rendering (placeholder feature flag)
- 3D plots
- Contour plots
- Animation support
- Interactive plots
- Jupyter notebook integration

### Documentation Gaps
- User guide (API docs are complete)
- Tutorial series
- Performance benchmarks
- Migration guides

### Infrastructure Gaps
- CI/CD pipeline
- Automated testing on multiple platforms
- Performance regression tests
- Security audit

## Breaking Changes

None (initial release)

## Migration Guide

Not applicable (initial release)

## Contributors

- Ibrahim Cesar (@ibrahimcesar) - Initial implementation

## Acknowledgments

Inspired by:
- matplotlib (Python)
- Wolfram Language plotting
- plotters (Rust)

---

[Unreleased]: https://github.com/ibrahimcesar/velociplot/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/ibrahimcesar/velociplot/releases/tag/v0.0.1
