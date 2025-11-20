---
sidebar_position: 4
---

# API Reference

Quick reference for all Velociplot types and methods.

:::tip Complete Documentation
For detailed API documentation with all methods and examples, visit:
**[docs.rs/velociplot](https://docs.rs/velociplot)**
:::

## Core Concepts

### Drawable Trait
All plot types implement the `Drawable` trait:
```rust
pub trait Drawable {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()>;
}
```

### Canvas
The rendering backend:
```rust
let mut canvas = SkiaCanvas::new(width, height, bounds)?;
canvas.fill_background(&color)?;
canvas.save_png("output.png")?;
```

### Bounds
Data coordinate system:
```rust
let bounds = Bounds::new(x_min, x_max, y_min, y_max);
let bounds = bounds.with_padding(0.1);  // Add 10% padding
let bounds = bounds.with_padding_top(0.1);  // Only top padding
```

### Series
Data container:
```rust
let series = Series::new(x_values, y_values)?;
let series = Series::from_function(0.0, 10.0, 100, |x| x.sin());
```

## Plot Types (15)

### LinePlot
```rust
LinePlot::new(series)
    .color(Color::RED)
    .line_width(2.0)
    .draw(&mut canvas)?;
```

### ScatterPlot
```rust
ScatterPlot::new(x, y)?
    .marker_shape(MarkerShape::Circle)
    .marker_size(5.0)
    .color(Color::BLUE)
    .opacity(0.7)
    .draw(&mut canvas)?;
```

**Marker Shapes**: Circle, Square, Triangle, Diamond, Plus, Cross

### BarPlot
```rust
BarPlot::new(series)?
    .orientation(BarOrientation::Vertical)
    .bar_width(0.8)
    .color(Color::GREEN)
    .draw(&mut canvas)?;
```

### StackedBarPlot
```rust
StackedBarPlot::new(x, series_list, labels)?
    .orientation(BarOrientation::Vertical)
    .bar_width(0.8)
    .draw(&mut canvas)?;
```

### Histogram
```rust
// From raw data
Histogram::from_data(data, num_bins)?
    .color(Color::PURPLE)
    .draw(&mut canvas)?;

// From pre-computed bins
Histogram::from_bins(bin_edges, frequencies)?
    .draw(&mut canvas)?;
```

### BoxPlot
```rust
BoxPlot::new(data)
    .position(1.0)
    .width(0.6)
    .color(Color::ORANGE)
    .draw(&mut canvas)?;
```

### ViolinPlot
```rust
ViolinPlot::new(data)?
    .position(1.0)
    .width(0.8)
    .color(Color::from_hex("#9b59b6").unwrap())
    .draw(&mut canvas)?;
```

### QQPlot & PPPlot
```rust
// Quantile-Quantile
QQPlot::new(data1, data2)?
    .draw(&mut canvas)?;

// Probability-Probability
PPPlot::new(data, expected_distribution)?
    .draw(&mut canvas)?;
```

### Heatmap
```rust
Heatmap::new(data, rows, cols)
    .colormap(Colormap::Viridis)
    .draw(&mut canvas)?;
```

**Colormaps**: Viridis, Plasma, Inferno, Coolwarm, Magma

### BubbleChart
```rust
BubbleChart::new(x, y, sizes)?
    .color(Color::RED)
    .opacity(0.6)
    .min_size(5.0)
    .max_size(30.0)
    .draw(&mut canvas)?;
```

### AreaPlot
```rust
AreaPlot::new(x, y)?
    .baseline(0.0)
    .color(Color::BLUE)
    .opacity(0.5)
    .draw(&mut canvas)?;
```

### StackedAreaPlot
```rust
StackedAreaPlot::new(x, series_list, labels)?
    .draw(&mut canvas)?;
```

### Treemap
```rust
let mut treemap = Treemap::new();
treemap.add_item("Label", value, Color::RED);
treemap.draw(&mut canvas)?;
```

### Timeline
```rust
let mut timeline = Timeline::new();
timeline
    .add_event(2020.0, "Event", Some("Description".to_string()))
    .orientation(TimelineOrientation::Horizontal)
    .draw(&mut canvas)?;
```

### DateListPlot
```rust
DateListPlot::new(times, values)?
    .style(DateListStyle::LinePoints)
    .show_grid(true)
    .draw(&mut canvas)?;
```

**Styles**: Line, Points, LinePoints

## Axes & Layout

### Axis
```rust
Axis::new(AxisPosition::Bottom)
    .label("X Axis")
    .show_grid(true)
    .draw(&mut canvas)?;
```

**Positions**: Top, Bottom, Left, Right

### Legend
```rust
let mut legend = Legend::new()
    .position(LegendPosition::UpperRight);

legend.add_entry(entry);
legend.draw(&mut canvas)?;
```

**Positions**:
- UpperLeft, UpperCenter, UpperRight
- MiddleLeft, MiddleCenter, MiddleRight
- LowerLeft, LowerCenter, LowerRight

### Legend Types
```rust
// Line legend
LegendEntry::new("Series 1")
    .color(Color::RED)
    .line_width(2.0)

// Point legend
LegendEntry::new("Data")
    .point_shape(LegendMarker::Circle, 5.0)
    .color(Color::BLUE)

// Swatch legend
LegendEntry::new("Category")
    .swatch_shape()
    .color(Color::GREEN)
```

### BarLegend (Colorbar)
```rust
BarLegend::new(min_value, max_value, Colormap::Viridis)
    .label("Scale")
    .position(BarLegendPosition::Right)
    .orientation(BarOrientation::Vertical)
    .draw(&mut canvas)?;
```

## Colors

### Named Colors
```rust
Color::RED
Color::GREEN
Color::BLUE
Color::WHITE
Color::BLACK
```

### Hex Colors
```rust
Color::from_hex("#1f77b4").unwrap()
Color::from_hex("#ff7f0e").unwrap()
```

### RGB
```rust
Color::from_rgb(255, 127, 14)
```

### RGBA
```rust
color.to_rgba()  // Returns [u8; 4]
```

### Colormaps
```rust
Colormap::Viridis.get(0.5)    // Get color at 50%
Colormap::Plasma.get(t)       // t from 0.0 to 1.0
Colormap::Inferno
Colormap::Coolwarm
Colormap::Magma
```

## Error Handling

All fallible operations return `Result<T>`:

```rust
pub enum Error {
    InvalidData(String),
    RenderError(String),
    IoError(std::io::Error),
    // ...
}
```

Use `?` operator:
```rust
fn main() -> Result<()> {
    let plot = LinePlot::new(data)?;
    plot.draw(&mut canvas)?;
    canvas.save_png("plot.png")?;
    Ok(())
}
```

## Builder Pattern

Most types use the builder pattern:

```rust
ScatterPlot::new(x, y)?
    .marker_shape(MarkerShape::Circle)  // Returns Self
    .marker_size(5.0)                   // Returns Self
    .color(Color::BLUE)                 // Returns Self
    .opacity(0.7)                       // Returns Self
    .draw(&mut canvas)?;                // Consumes
```

Methods marked with `#[must_use]` should be chained or assigned.

## Data Integration

### ndarray
```rust
use ndarray::Array1;
let x = Array1::linspace(0.0, 10.0, 100);
let y = x.mapv(|x| x.sin());
```

### polars
```rust
use polars::prelude::*;
let df = CsvReader::from_path("data.csv")?.finish()?;
let x: Vec<f64> = df.column("x")?.f64()?.into_no_null_iter().collect();
```

## Performance Tips

1. **Preallocate**: Use `Vec::with_capacity()` for large datasets
2. **Reuse Canvas**: Create once, draw multiple plots
3. **Sample Large Data**: Plot subset for exploration
4. **Release Mode**: Always benchmark in `--release`

## Common Patterns

### Multiple Series
```rust
let series1 = LinePlot::new(data1)?.color(Color::RED);
let series2 = LinePlot::new(data2)?.color(Color::BLUE);

series1.draw(&mut canvas)?;
series2.draw(&mut canvas)?;
```

### Custom Bounds
```rust
let bounds = Bounds::new(0.0, 10.0, -1.0, 1.0);
let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
```

### Text Annotations
```rust
canvas.draw_text(
    "Important Point",
    x as f32,
    y as f32,
    12.0,  // Font size
    &Color::BLACK.to_rgba()
)?;
```

### Lines and Shapes
```rust
// Line
canvas.draw_line(&point1, &point2, &color, width)?;

// Circle
canvas.draw_circle(&center, radius, &color, filled)?;

// Rectangle
canvas.draw_rectangle(&bottom_left, width, height, &color)?;
```

## Examples

See the [examples directory](https://github.com/ibrahimcesar/velociplot/tree/main/examples) for 30+ complete examples.

## Further Reading

- [Tutorial](./tutorial-basics/create-a-document.md) - Step-by-step guide
- [Real-World Examples](./real-world-examples/climate-data.md) - Use actual datasets
- [docs.rs](https://docs.rs/velociplot) - Complete API documentation

---

**Questions?** Open an [issue](https://github.com/ibrahimcesar/velociplot/issues) or [discussion](https://github.com/ibrahimcesar/velociplot/discussions)!
