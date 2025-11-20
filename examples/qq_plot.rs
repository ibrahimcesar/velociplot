//! Q-Q Plot example for testing normality

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Q-Q Plot example");

    // Generate normally distributed data
    let mut data = Vec::new();
    let mut seed = 42u64;

    for _ in 0..200 {
        // Box-Muller transform for normal distribution
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u1 = seed as f64 / (1u64 << 31) as f64;

        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u2 = seed as f64 / (1u64 << 31) as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let value = 10.0 + 3.0 * z; // Mean=10, StdDev=3
        data.push(value);
    }

    // Create Q-Q plot against standard normal
    let qq = QQPlot::new(
        &data,
        Distribution::NormalCustom {
            mean: 10.0,
            std_dev: 3.0,
        },
    )
    .show_reference_line(true)
    .color(Color::from_hex("#e74c3c").unwrap())
    .label("Sample Data");

    let bounds = qq.bounds().unwrap().with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Theoretical Quantiles")
        .tick_count(8)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Sample Quantiles")
        .tick_count(8)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw Q-Q plot
    qq.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = qq.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperLeft);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/qq_plot.png")?;

    println!("✓ Plot saved to examples/images/qq_plot.png");
    println!("  Q-Q plot comparing sample data to Normal(10, 3) distribution");
    println!("  Points close to diagonal line indicate good fit");

    Ok(())
}
