---
sidebar_position: 1
---

# Getting Started

**Velociplot** is a fast, publication-quality scientific plotting library for Rust. Quick, precise, and deadly effective 🦖

## Why Velociplot?

- 🦖 **Blazingly Fast** - Pure Rust with no Python/C dependencies
- 📊 **Publication Quality** - High-quality output suitable for papers and reports
- 🎨 **Beautiful Defaults** - Perceptually uniform colormaps, professional styling
- 🔧 **Ergonomic API** - Simple for basics, powerful for complex figures
- 📁 **Multiple Formats** - PNG output (SVG/PDF coming soon)
- 🚀 **15 Plot Types** - From basic lines to advanced treemaps

## Installation

Add velociplot to your `Cargo.toml`:

```toml
[dependencies]
velociplot = "0.0.1"
```

Or install from source:

```bash
git clone https://github.com/ibrahimcesar/velociplot
cd velociplot
cargo build --release
```

## Quick Example

Let's create your first plot - a simple line chart:

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Create data - a parabola
    let data = Series::from_function(0.0, 10.0, 50, |x| x * x);
    
    // Create the plot
    let plot = LinePlot::new(data)
        .color(Color::from_hex("#1f77b4").unwrap())
        .line_width(2.5);
    
    // Set up canvas
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Add axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X")
        .show_grid(true);
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y = X²")
        .show_grid(true);
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    plot.draw(&mut canvas)?;
    
    // Save
    canvas.save_png("parabola.png")?;
    
    Ok(())
}
```

Run this and you'll get a beautiful plot in `parabola.png`!

## Features

### Plot Types

Velociplot supports 15 plot types:

| Plot Type | Use Case |
|-----------|----------|
| **LinePlot** | Continuous data, trends |
| **ScatterPlot** | Point data, correlations |
| **BarPlot** | Categorical comparisons |
| **StackedBarPlot** | Cumulative categories |
| **Histogram** | Distributions |
| **BoxPlot** | Statistical summaries |
| **ViolinPlot** | Distribution density |
| **QQPlot/PPPlot** | Distribution comparison |
| **Heatmap** | 2D density, matrices |
| **BubbleChart** | 3-variable relationships |
| **AreaPlot** | Cumulative trends |
| **StackedAreaPlot** | Multiple cumulative series |
| **Treemap** | Hierarchical data |
| **Timeline** | Chronological events |
| **DateListPlot** | Time series data |

### Styling Options

- **Colors**: Hex codes, RGB, named colors
- **Colormaps**: Viridis, Plasma, Inferno, Coolwarm, Magma
- **Markers**: 6 shapes (Circle, Square, Triangle, Diamond, Plus, Cross)
- **Lines**: Customizable width and style
- **Legends**: 4 types with 9 positioning options

### Data Integration

Works seamlessly with:
- **ndarray** - N-dimensional arrays
- **polars** - DataFrames
- **Vec<f64>** - Plain vectors
- **Functions** - Generate data on the fly

## Next Steps

Choose your path:

- 📚 **[Tutorial](./tutorial-basics/create-a-document.md)** - Step-by-step guide
- 🌍 **[Real-World Examples](./category/real-world-examples)** - Use public datasets
- 📖 **[API Reference](https://docs.rs/velociplot)** - Complete documentation
- 💡 **[Examples Gallery](./examples/overview.md)** - Visual showcase

## Philosophy

Velociplot is inspired by matplotlib but built from the ground up in Rust:

- **Performance First** - 10-100x faster than Python
- **Type Safety** - Catch errors at compile time
- **Modern Defaults** - Beautiful out of the box
- **Progressive Disclosure** - Easy to start, powerful when needed

## Community

- 🐛 [Report bugs](https://github.com/ibrahimcesar/velociplot/issues)
- 💡 [Request features](https://github.com/ibrahimcesar/velociplot/discussions)
- 💬 [Ask questions](https://github.com/ibrahimcesar/velociplot/discussions)
- 🤝 [Contribute](https://github.com/ibrahimcesar/velociplot/blob/main/CONTRIBUTING.md)

Happy plotting! 🦖
