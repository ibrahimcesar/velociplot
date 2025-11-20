//! Violin plot example showing distribution visualization

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Violin plot example");

    // Generate three different distributions
    let mut group1 = Vec::new();
    let mut group2 = Vec::new();
    let mut group3 = Vec::new();

    let mut seed = 42u64;

    // Group 1: Normal distribution (mean=10, std=2)
    for _ in 0..100 {
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u1 = seed as f64 / (1u64 << 31) as f64;
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u2 = seed as f64 / (1u64 << 31) as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        group1.push(10.0 + 2.0 * z);
    }

    // Group 2: Normal distribution (mean=15, std=3)
    for _ in 0..100 {
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u1 = seed as f64 / (1u64 << 31) as f64;
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u2 = seed as f64 / (1u64 << 31) as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        group2.push(15.0 + 3.0 * z);
    }

    // Group 3: Normal distribution (mean=12, std=1.5)
    for _ in 0..100 {
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u1 = seed as f64 / (1u64 << 31) as f64;
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let u2 = seed as f64 / (1u64 << 31) as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        group3.push(12.0 + 1.5 * z);
    }

    // Create violin plots at different x positions
    let violin1 = ViolinPlot::new(&group1, 1.0)
        .width(0.3)
        .color(Color::from_hex("#9b59b6").unwrap())
        .show_box(true)
        .kernel(Kernel::Gaussian)
        .label("Group A");

    let violin2 = ViolinPlot::new(&group2, 2.0)
        .width(0.3)
        .color(Color::from_hex("#e74c3c").unwrap())
        .show_box(true)
        .kernel(Kernel::Gaussian)
        .label("Group B");

    let violin3 = ViolinPlot::new(&group3, 3.0)
        .width(0.3)
        .color(Color::from_hex("#3498db").unwrap())
        .show_box(true)
        .kernel(Kernel::Gaussian)
        .label("Group C");

    // Calculate combined bounds
    let bounds1 = violin1.bounds().unwrap();
    let bounds2 = violin2.bounds().unwrap();
    let bounds3 = violin3.bounds().unwrap();
    let combined = bounds1.union(&bounds2).union(&bounds3);
    let bounds = combined.with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Groups")
        .tick_count(4)
        .show_grid(false);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Value")
        .tick_count(8)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw violins
    violin1.draw(&mut canvas)?;
    violin2.draw(&mut canvas)?;
    violin3.draw(&mut canvas)?;

    // Add legend
    let mut legend = Legend::new().position(LegendPosition::UpperRight);

    for violin in [&violin1, &violin2, &violin3] {
        if let Some(entry) = violin.legend_entry() {
            legend = legend.add_entry(entry);
        }
    }

    legend.draw(&mut canvas)?;

    canvas.save_png("examples/images/violin_plot.png")?;

    println!("✓ Plot saved to examples/images/violin_plot.png");
    println!("  Three violin plots showing distribution shapes");
    println!("  Wider sections = more data points");
    println!("  Box plot overlays show quartiles and median");

    Ok(())
}
