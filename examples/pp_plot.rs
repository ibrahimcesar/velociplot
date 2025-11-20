//! P-P Plot example for distribution comparison

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - P-P Plot example");

    // Generate normally distributed data
    let mut data = Vec::new();
    let mut seed = 123u64;

    for _ in 0..200 {
        // Box-Muller transform
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u1 = seed as f64 / (1u64 << 31) as f64;

        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u2 = seed as f64 / (1u64 << 31) as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let value = 50.0 + 12.0 * z; // Mean=50, StdDev=12
        data.push(value);
    }

    // Create P-P plot against normal distribution
    let pp = PPPlot::new(
        &data,
        Distribution::NormalCustom {
            mean: 50.0,
            std_dev: 12.0,
        },
    )
    .show_reference_line(true)
    .color(Color::from_hex("#3498db").unwrap())
    .label("Sample vs Normal");

    let bounds = pp.bounds().unwrap().with_padding(0.05);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Theoretical Cumulative Probability")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Empirical Cumulative Probability")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw P-P plot
    pp.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = pp.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperLeft);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/pp_plot.png")?;

    println!("✓ Plot saved to examples/images/pp_plot.png");
    println!("  P-P plot comparing cumulative probabilities");
    println!("  Points on diagonal indicate perfect distributional match");

    Ok(())
}
