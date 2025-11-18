<div align="center">
    
# velociplot 🦖

> Scientific plotting at velociraptor speed

**velociplot** (Velociraptor + plot) is a fast, publication-quality plotting library for Rust. Quick, precise, and deadly effective for creating scientific figures.

</div>

## 🎯 What is velociplot?

*Veloci-plot* = Velocity + plot  
Like a velociraptor: **quick, precise, and deadly effective**

A pure Rust plotting library designed for scientists, engineers, and developers who need high-performance, publication-ready visualizations.

## ✨ Features (Planned)

- 🦖 **Blazingly Fast** - Plot millions of points in milliseconds
- 📊 **Publication Quality** - LaTeX math, precise DPI control, vector output
- 📐 **Scientific Plots** - Line, scatter, histogram, heatmap, contour, 3D surface
- 🎨 **Beautiful Defaults** - Perceptually uniform colormaps, colorblind-friendly
- 📁 **Multiple Formats** - PDF, SVG, PNG, EPS (vector + raster)
- 🔧 **Ergonomic API** - Simple for basics, powerful for complex figures
- 🚀 **Pure Rust** - No Python/matplotlib required

## 🚧 Status

**Work in Progress** - This project is in early development.

Current version: `0.1.0-alpha`

## 🎓 Quick Example (Planned API)
```rust
use velociplot::prelude::*;

// Simple plot
plot()
    .line(x, y)
    .xlabel("Time (s)")
    .ylabel("Temperature (K)")
    .save("figure.pdf")?;

// Publication-quality figure
Figure::new()
    .size_inches(3.5, 2.5)  // Single-column width
    .dpi(300)
    .add_subplot()
        .line(x, y)
            .label("Experimental")
            .color("#1f77b4")
            .linewidth(1.5)
        .scatter(x2, y2)
            .label("Theoretical")
            .marker('o')
        .xlabel("Time (s)")
        .ylabel("Temperature (K)")
        .legend()
            .location(Location::UpperRight)
        .grid(true)
    .save("figure.pdf")?;
```

## 📦 Installation
```bash
# Not yet published to crates.io
# For now, build from source:
git clone https://github.com/yourusername/velociplot
cd velociplot
cargo build --release
```

## 🗺️ Roadmap

### Phase 1: Foundation (Current)
- [ ] Core architecture and traits
- [ ] Basic 2D coordinate system
- [ ] Simple line and scatter plots
- [ ] PNG output (raster)
- [ ] Color and style system

### Phase 2: Core Plots
- [ ] Bar charts and histograms
- [ ] Error bars and bands
- [ ] Multiple series support
- [ ] Legend and annotations
- [ ] Axis customization (labels, limits, scales)

### Phase 3: Publication Quality
- [ ] LaTeX math rendering
- [ ] Vector output (PDF, SVG, EPS)
- [ ] DPI and size control
- [ ] Multiple subplots and layouts
- [ ] Publication templates (Nature, Science, IEEE, ACS)

### Phase 4: Advanced Plots
- [ ] Heatmaps and colormaps
- [ ] Contour plots
- [ ] 3D surface plots
- [ ] Polar plots
- [ ] Vector fields (quiver)

### Phase 5: Ecosystem Integration
- [ ] Integration with `ndarray`
- [ ] Integration with `polars` DataFrames
- [ ] Jupyter notebook support
- [ ] CLI tool for quick plotting
- [ ] Python bindings (PyO3)

## 🎨 Design Philosophy

**Inspired by matplotlib, but Rust-native:**

- ✅ **Performance** - 10-100x faster than Python/matplotlib
- ✅ **Type Safety** - Catch errors at compile time
- ✅ **No Dependencies** - No Python, no C libraries (for core features)
- ✅ **Modern Defaults** - Beautiful out-of-the-box
- ✅ **Progressive Disclosure** - Easy to start, powerful when needed

**API Principles:**
- Simple one-liners for common tasks
- Builder pattern for complex figures
- Method chaining for fluent API
- Sensible defaults (but full control when needed)

## 🎯 Use Cases

- **Academic Papers** - Publication-ready figures with LaTeX
- **Data Analysis** - Quick exploratory plots
- **Engineering** - Technical visualizations and reports
- **Real-time Monitoring** - High-performance streaming plots
- **Web Services** - Generate plots server-side (no GUI needed)

## 🔧 Architecture

```bash
velociplot/
├── velociplot-core/      # Core plotting engine
├── velociplot-backend/   # Rendering backends (Cairo, Skia, etc.)
├── velociplot-formats/   # Output formats (PDF, SVG, PNG)
├── velociplot-styles/    # Style presets and themes
└── velociplot-cli/       # Command-line tool (optional)
```

## 🤝 Contributing

Contributions are welcome! This project is in early stages.

**How to contribute:**
- 🐛 Report bugs or suggest features via Issues
- 💻 Submit PRs for bug fixes or features
- 📝 Improve documentation
- 🎨 Create plot examples or style templates
- 🧪 Add test cases

## 📚 Documentation

- [API Documentation](https://docs.rs/velociplot) (coming soon)
- [User Guide](https://velociplot.rs) (coming soon)
- [Examples Gallery](./examples/) (coming soon)

## 🙏 Acknowledgments

Standing on the shoulders of giants:

- [matplotlib](https://matplotlib.org/) - The gold standard for scientific plotting
- [plotters](https://github.com/plotters-rs/plotters) - Rust plotting pioneer
- [egui_plot](https://github.com/emilk/egui) - Interactive plotting in Rust
- [poloto](https://github.com/tiby312/poloto) - SVG plotting techniques
- [resvg](https://github.com/RazrFalcon/resvg) - SVG rendering

## 📝 License

MIT OR Apache-2.0

## 🦖 Why "velociplot"?

Because your scientific plots should be as fast and efficient as a velociraptor hunting prey. No more waiting minutes for matplotlib to render complex figures!

---

**velociplot** - *Because your plots shouldn't take ages to render* 🦖⚡

*Status: 🚧 Pre-alpha - Core architecture in development*

**Star** ⭐ this repo to follow development!
