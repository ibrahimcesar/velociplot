//! Histogram example showing data distribution

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Histogram example");

    // Generate sample data: simulated normal distribution
    let mut data = Vec::new();

    // Simple pseudo-random number generator for demonstration
    let mut seed = 42u64;
    for _ in 0..1000 {
        // Box-Muller transform for normal distribution
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u1 = seed as f64 / (1u64 << 31) as f64;

        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u2 = seed as f64 / (1u64 << 31) as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let value = 50.0 + 15.0 * z; // Mean=50, StdDev=15
        data.push(value);
    }

    // Create histogram with automatic binning
    let hist = Histogram::new(&data)
        .bin_strategy(BinStrategy::Auto)
        .color(Color::from_hex("#2ecc71").unwrap())
        .show_outline(true)
        .label("Sample Distribution");

    let bounds = hist.bounds().unwrap().with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Value")
        .tick_count(8)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Frequency")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw histogram
    hist.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = hist.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperRight);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/histogram.png")?;

    println!("✓ Plot saved to examples/images/histogram.png");
    println!("  Distribution of 1000 data points");
    println!("  Using automatic binning (Sturges' formula)");

    Ok(())
}
