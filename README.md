<div align="center">
    
# Velociplot 🦖

_Scientific plotting at velociraptor speed_

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

**Early Development** - Basic plotting is now functional!

Current version: `0.0.1-alpha`

✅ **What works now:**
- ✅ Line plots with customizable colors and widths
- ✅ **Axes with tick marks and labels**
- ✅ **Grid lines**
- ✅ **Legends with automatic positioning**
- ✅ **Text rendering** (axis labels, legend text, annotations)
- ✅ PNG output via tiny-skia backend
- ✅ Data from functions, vectors, or tuples
- ✅ Automatic bounds calculation
- ✅ Pure Rust, no C dependencies

🚧 **Coming soon:** More plot types (scatter points, bar, histogram), SVG/PDF output, LaTeX math rendering

## 🎓 Quick Example

```rust
use velociplot::prelude::*;

// Create data - a simple parabola
let data = Series::from_function(0.0, 10.0, 50, |x| x * x);

// Create and customize the plot
let plot = LinePlot::new(data)
    .color(Color::from_hex("#1f77b4").unwrap())
    .line_width(2.5);

// Set up canvas and render
let bounds = plot.bounds().unwrap().with_padding(0.1);
let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
canvas.fill_background(&Color::WHITE.to_rgba())?;

// Add axes with grid
let x_axis = Axis::new(AxisPosition::Bottom)
    .label("X")
    .show_grid(true);
let y_axis = Axis::new(AxisPosition::Left)
    .label("Y = X²")
    .show_grid(true);

x_axis.draw(&mut canvas)?;
y_axis.draw(&mut canvas)?;
plot.draw(&mut canvas)?;

// Add legend
if let Some(entry) = plot.legend_entry() {
    let legend = Legend::new()
        .add_entry(entry)
        .position(LegendPosition::UpperRight);
    legend.draw(&mut canvas)?;
}

// Save to PNG
canvas.save_png("plot.png")?;
```

See [examples/](examples/) for more including multi-series plots with legends!

### Custom Fonts

You can use custom fonts for text rendering:

```rust
let mut canvas = SkiaCanvas::new(800, 600, bounds)?;

// Load a custom font from a file
canvas.load_font_from_file("path/to/your/font.ttf")?;

// Or load from bytes
let font_bytes = include_bytes!("../assets/MyFont.ttf");
canvas.load_font_from_bytes(font_bytes)?;
```

By default, velociplot uses JetBrains Mono embedded in the library.

### Legend Customization

#### Understanding Vertical Alignment

Legends display a colored line next to text labels. The vertical alignment determines where that line appears relative to the text. Different fonts have different proportions (x-height, ascenders, descenders), so the "visually centered" position varies.

**Why this matters:**
- Typography has multiple reference points: baseline, x-height, cap-height
- The **optical center** (where text "feels" centered) isn't at 50% of the text box
- Most lowercase letters (a, e, o, n) sit on the **baseline** and extend to the **x-height**
- The visual center is typically around the middle of the x-height

**How it works:**
```
Text Box:  ┌─────────┐  ← 0.0 (top)
           │    T    │
           │    e  ──│─ 0.70 (default - optical center)
           │    x    │
           │    t  ──│─ 0.75 (baseline)
           └─────────┘  ← 1.0 (bottom)
```

Fine-tune the alignment to match your font:

```rust
let legend = Legend::new()
    .add_entry(entry)
    .position(LegendPosition::UpperRight)
    .line_vertical_align(0.70);  // 0.0 = top, 1.0 = bottom
```

**Recommended values:**
- `0.55` - Higher (fonts with large x-height like Verdana)
- `0.65` - Geometric center of x-height
- `0.70` - **Default** (optimized for JetBrains Mono)
- `0.75` - Baseline (mathematical horizontal reference)

**Testing your font:**
```bash
cargo run --example legend_alignment_test
# Generates 6 comparison plots (0.50, 0.55, 0.60, 0.65, 0.70, 0.75)
# Open the PNG files and pick the one that looks best centered
```

The default (0.70) was chosen after visual testing with JetBrains Mono, the embedded font. If you load a custom font, you may want to adjust this value.

## 📦 Installation

```toml
# Add to Cargo.toml (not yet on crates.io)
[dependencies]
velociplot = { git = "https://github.com/ibrahimcesar/velociplot" }
```

Or build from source:
```bash
git clone https://github.com/ibrahimcesar/velociplot
cd velociplot
cargo build --release
cargo run --example basic_line
```

## 🗺️ Roadmap

### Phase 1: Foundation ✅ **DONE!**
- [x] Core architecture and traits
- [x] Basic 2D coordinate system
- [x] Simple line plots
- [x] PNG output (raster via tiny-skia)
- [x] Color and style system

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

## 📸 Example Output

<div align="center">

### Basic Line Plot
![Basic Line Plot](examples/basic_line.png)

### Multiple Series
![Multiple Series](examples/scatter.png)

</div>

---

*Status: ✅ **Phase 1 Complete** - Basic plotting now functional!*

**Star** ⭐ this repo to follow development!

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://ibrahimcesar.cloud"><img src="https://github.com/ibrahimcesar.png" width="100px;" alt="Ibrahim Cesar"/><br /><sub><b>Ibrahim Cesar</b></sub></a><br /><a href="https://github.com/ibrahimcesar/velociplot/commits?author=ibrahimcesar" title="Code">💻</a> <a href="https://github.com/ibrahimcesar/velociplot/commits?author=ibrahimcesar" title="Documentation">📖</a> <a href="#example-ibrahimcesar" title="Examples">💡</a> <a href="#ideas-ibrahimcesar" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-ibrahimcesar" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-ibrahimcesar" title="Maintenance">🚧</a> <a href="https://github.com/ibrahimcesar/velociplot/commits?author=ibrahimcesar" title="Tests">✅</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
