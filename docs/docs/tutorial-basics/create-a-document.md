---
sidebar_position: 1
---

# Create Your First Plot

Let's create your first plot with Velociplot in **less than 5 minutes**.

## Prerequisites

You need:
- Rust 1.70 or later
- Cargo (comes with Rust)

Install Rust from [rustup.rs](https://rustup.rs/) if you haven't already.

## Create a New Project

```bash
cargo new my_first_plot
cd my_first_plot
```

## Add Velociplot

Edit `Cargo.toml`:

```toml title="Cargo.toml"
[package]
name = "my_first_plot"
version = "0.1.0"
edition = "2021"

[dependencies]
velociplot = { git = "https://github.com/ibrahimcesar/velociplot" }
```

:::tip
Once published to crates.io, you'll use:
```toml
velociplot = "0.0.1"
```
:::

## Your First Line Plot

Replace `src/main.rs` with:

```rust title="src/main.rs"
use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 Creating my first plot!");

    // Step 1: Create data
    let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.1).collect();
    let y: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
    
    // Step 2: Create a plot
    let plot = LinePlot::new(Series::new(x, y)?)
        .color(Color::from_hex("#1f77b4").unwrap())
        .line_width(2.0);
    
    // Step 3: Set up canvas
    let bounds = plot.bounds().unwrap().with_padding(0.1);
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;
    
    // Step 4: Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("x")
        .show_grid(true);
    let y_axis = Axis::new(AxisPosition::Left)
        .label("sin(x)")
        .show_grid(true);
    
    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    
    // Step 5: Draw the plot
    plot.draw(&mut canvas)?;
    
    // Step 6: Save the image
    canvas.save_png("sine_wave.png")?;
    
    println!("✅ Plot saved to sine_wave.png");
    
    Ok(())
}
```

## Run It!

```bash
cargo run
```

You should see:
```
🦖 Creating my first plot!
✅ Plot saved to sine_wave.png
```

Open `sine_wave.png` to see your beautiful sine wave! 🎉

## Understanding the Code

Let's break down what each part does:

### 1. Import the Prelude
```rust
use velociplot::prelude::*;
```
This imports all the commonly used types and traits.

### 2. Create Data
```rust
let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.1).collect();
let y: Vec<f64> = x.iter().map(|&x| x.sin()).collect();
```
We create 50 points from 0 to 5, calculating sin(x) for each.

### 3. Create a Plot
```rust
let plot = LinePlot::new(Series::new(x, y)?)
    .color(Color::from_hex("#1f77b4").unwrap())
    .line_width(2.0);
```
- `LinePlot::new()` creates a line plot
- `.color()` sets the line color (hex code)
- `.line_width()` sets the thickness

### 4. Set Up Canvas
```rust
let bounds = plot.bounds().unwrap().with_padding(0.1);
let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
```
- `bounds()` calculates the data range automatically
- `.with_padding(0.1)` adds 10% padding around the plot
- `SkiaCanvas::new()` creates an 800x600 pixel canvas

### 5. Draw Everything
```rust
x_axis.draw(&mut canvas)?;
y_axis.draw(&mut canvas)?;
plot.draw(&mut canvas)?;
```
Order matters! Axes first, then the plot on top.

### 6. Save the Image
```rust
canvas.save_png("sine_wave.png")?;
```
Saves to the current directory.

## Try It Yourself

Modify the code to create different plots:

### Parabola
```rust
let y: Vec<f64> = x.iter().map(|&x| x * x).collect();
```

### Exponential
```rust
let y: Vec<f64> = x.iter().map(|&x| x.exp()).collect();
```

### Custom Function
```rust
let y: Vec<f64> = x.iter().map(|&x| {
    x.sin() * x.cos() + 0.5 * x
}).collect();
```

## Common Issues

### "bounds is None"
**Problem**: No data to plot
```rust
let plot = LinePlot::new(Series::new(vec![], vec![])?); // Empty!
```
**Solution**: Make sure your vectors have data.

### "vectors have different lengths"
**Problem**: x and y don't match
```rust
let x = vec![1.0, 2.0, 3.0];
let y = vec![1.0, 2.0];  // Only 2 elements!
```
**Solution**: Ensure x and y have the same length.

### Image looks wrong
**Problem**: Forgot to draw axes or plot
**Solution**: Check you called `.draw()` on everything.

## Next Steps

Now that you can create a basic plot, learn about:
- [Adding Multiple Series](./multiple-series.md) - Plot multiple lines
- [Customizing Colors](./customization.md) - Make it beautiful
- [Plot Types](./plot-types.md) - Scatter, bar, histogram, etc.

Happy plotting! 🦖
