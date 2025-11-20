---
sidebar_position: 2
---

# Multiple Series

Learn to plot multiple datasets on a single canvas.

## Basic Multiple Series

The simplest way to plot multiple series is to draw them one after another:

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // Create two datasets
    let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.1).collect();
    
    let y1: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
    let y2: Vec<f64> = x.iter().map(|&x| x.cos()).collect();
    
    // Create two plots
    let plot1 = LinePlot::new(Series::new(x.clone(), y1)?)
        .color(Color::from_hex("#e74c3c").unwrap())  // Red
        .line_width(2.0);
    
    let plot2 = LinePlot::new(Series::new(x.clone(), y2)?)
        .color(Color::from_hex("#3498db").unwrap())  // Blue
        .line_width(2.0);
    
    // Calculate combined bounds
    let mut bounds = plot1.bounds().unwrap();
    bounds = bounds.union(&plot2.bounds().unwrap());
    bounds = bounds.with_padding(0.1);
    
    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("x")
        .show_grid(true);
    let y_axis = Axis::new(AxisPosition::Left)
        .label("y")
        .show_grid(true);
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    // Draw both plots
    plot1.draw(&mut canvas)?;
    plot2.draw(&mut canvas)?;
    
    canvas.save_png("multiple_series.png")?;
    
    Ok(())
}
```

## Adding a Legend

Legends help identify which series is which:

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    // ... (same setup as above)
    
    // Create plots with labels
    let plot1 = LinePlot::new(Series::new(x.clone(), y1)?)
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.0)
        .label("sin(x)");  // Add label
    
    let plot2 = LinePlot::new(Series::new(x.clone(), y2)?)
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.0)
        .label("cos(x)");  // Add label
    
    // ... (draw plots)
    
    // Add legend
    let mut legend = Legend::new()
        .position(LegendPosition::UpperRight);
    
    if let Some(entry) = plot1.legend_entry() {
        legend = legend.add_entry(entry);
    }
    if let Some(entry) = plot2.legend_entry() {
        legend = legend.add_entry(entry);
    }
    
    legend.draw(&mut canvas)?;
    
    canvas.save_png("with_legend.png")?;
    
    Ok(())
}
```

## Many Series

For many series, use a loop:

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    let x: Vec<f64> = (0..100).map(|i| i as f64 * 0.05).collect();
    
    // Different frequencies
    let frequencies = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let colors = vec![
        Color::from_hex("#e74c3c").unwrap(),
        Color::from_hex("#3498db").unwrap(),
        Color::from_hex("#2ecc71").unwrap(),
        Color::from_hex("#f39c12").unwrap(),
        Color::from_hex("#9b59b6").unwrap(),
    ];
    
    // Create all plots
    let mut plots = Vec::new();
    for (freq, color) in frequencies.iter().zip(colors.iter()) {
        let y: Vec<f64> = x.iter().map(|&x| (x * freq).sin()).collect();
        let plot = LinePlot::new(Series::new(x.clone(), y)?)
            .color(color.clone())
            .line_width(1.5)
            .label(format!("freq = {}", freq));
        plots.push(plot);
    }
    
    // Calculate combined bounds
    let mut bounds = plots[0].bounds().unwrap();
    for plot in &plots[1..] {
        bounds = bounds.union(&plot.bounds().unwrap());
    }
    bounds = bounds.with_padding(0.1);
    
    let mut canvas = SkiaCanvas::new(1000, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Draw axes
    Axis::new(AxisPosition::Bottom).label("x").show_grid(true).draw(&mut canvas)?;
    Axis::new(AxisPosition::Left).label("y").show_grid(true).draw(&mut canvas)?;
    
    // Draw all plots
    for plot in &plots {
        plot.draw(&mut canvas)?;
    }
    
    // Add legend
    let mut legend = Legend::new().position(LegendPosition::UpperRight);
    for plot in &plots {
        if let Some(entry) = plot.legend_entry() {
            legend = legend.add_entry(entry);
        }
    }
    legend.draw(&mut canvas)?;
    
    canvas.save_png("many_series.png")?;
    
    Ok(())
}
```

## Different Plot Types

Mix different plot types on one canvas:

```rust
use velociplot::prelude::*;

fn main() -> Result<()> {
    let x: Vec<f64> = (0..20).map(|i| i as f64).collect();
    let y_line: Vec<f64> = x.iter().map(|&x| x * 2.0 + 10.0).collect();
    let y_scatter: Vec<f64> = x.iter().enumerate()
        .map(|(i, &x)| x * 2.0 + 10.0 + (i as f64 * 3.7).sin() * 5.0)
        .collect();
    
    // Line plot (trend)
    let line = LinePlot::new(Series::new(x.clone(), y_line)?)
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.0)
        .label("Trend");
    
    // Scatter plot (actual data)
    let scatter = ScatterPlot::new(x.clone(), y_scatter)?
        .marker_shape(MarkerShape::Circle)
        .marker_size(6.0)
        .color(Color::from_hex("#e74c3c").unwrap())
        .label("Measurements");
    
    // Calculate bounds
    let bounds = line.bounds().unwrap()
        .union(&scatter.bounds().unwrap())
        .with_padding(0.1);
    
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Draw everything
    Axis::new(AxisPosition::Bottom).label("Time").draw(&mut canvas)?;
    Axis::new(AxisPosition::Left).label("Value").draw(&mut canvas)?;
    
    line.draw(&mut canvas)?;
    scatter.draw(&mut canvas)?;
    
    // Legend with mixed types
    let mut legend = Legend::new().position(LegendPosition::UpperLeft);
    if let Some(entry) = line.legend_entry() {
        legend = legend.add_entry(entry);
    }
    if let Some(entry) = scatter.legend_entry() {
        legend = legend.add_entry(entry);
    }
    legend.draw(&mut canvas)?;
    
    canvas.save_png("mixed_types.png")?;
    
    Ok(())
}
```

## Tips for Multiple Series

### 1. Choose Distinct Colors

```rust
// Use colorblind-friendly palette
let colors = vec![
    Color::from_hex("#0173B2").unwrap(),  // Blue
    Color::from_hex("#DE8F05").unwrap(),  // Orange
    Color::from_hex("#029E73").unwrap(),  // Green
    Color::from_hex("#CC78BC").unwrap(),  // Purple
];
```

### 2. Vary Line Styles

```rust
let plot1 = LinePlot::new(data1)?.line_width(2.0);   // Thick
let plot2 = LinePlot::new(data2)?.line_width(1.0);   // Thin
```

### 3. Use Opacity for Overlap

```rust
let scatter = ScatterPlot::new(x, y)?
    .opacity(0.5);  // Semi-transparent
```

### 4. Order Matters

```rust
// Draw background elements first
grid.draw(&mut canvas)?;
axes.draw(&mut canvas)?;

// Then main plots
plot1.draw(&mut canvas)?;
plot2.draw(&mut canvas)?;

// Finally annotations and legend
legend.draw(&mut canvas)?;
```

## Common Patterns

### Comparison Plot

```rust
// Actual vs Predicted
let actual = LinePlot::new(data_actual)?
    .color(Color::BLACK)
    .label("Actual");

let predicted = LinePlot::new(data_pred)?
    .color(Color::RED)
    .label("Predicted");
```

### Confidence Bands

```rust
// Mean line
let mean = LinePlot::new(mean_data)?
    .color(Color::BLUE)
    .label("Mean");

// Upper/lower bounds as area plots
let upper = AreaPlot::new(x.clone(), upper_bound)?
    .color(Color::BLUE)
    .opacity(0.2);

let lower = AreaPlot::new(x, lower_bound)?
    .color(Color::BLUE)
    .opacity(0.2);
```

## Next Steps

- [Customization](./customization.md) - Colors, styles, fonts
- [Plot Types Guide](./plot-types.md) - Choose the right visualization
- [API Reference](../api-reference.md) - Complete method documentation

---

**Try it yourself!** Modify the examples to plot your own multi-series data.
