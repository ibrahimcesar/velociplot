//! Legend below graph example - Publication standard
//!
//! Demonstrates placing the legend below the graph area, which is the
//! standard format for scientific publications and manuscripts.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Legend below graph example");

    // Create sample data
    let data1 = Series::from_function(0.0, 10.0, 50, |x| x.powi(2) * 0.1);
    let data2 = Series::from_function(0.0, 10.0, 50, |x| x * 1.5);
    let data3 = Series::from_function(0.0, 10.0, 50, |x| (x * 0.5).sin() * 3.0 + 5.0);

    // Create line plots
    let line1 = LinePlot::new(data1)
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.5)
        .label("Quadratic");

    let line2 = LinePlot::new(data2)
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.5)
        .label("Linear");

    let line3 = LinePlot::new(data3)
        .color(Color::from_hex("#2ecc71").unwrap())
        .line_width(2.5)
        .label("Sinusoidal");

    // Calculate bounds
    let bounds1 = line1.bounds().unwrap();
    let bounds2 = line2.bounds().unwrap();
    let bounds3 = line3.bounds().unwrap();
    let bounds = bounds1.union(&bounds2).union(&bounds3).with_padding(0.1);

    // Create canvas with extra height for legend below
    // Standard size (800x600) + ~120px for axis labels, spacing, and legend
    // The legend will be positioned in this extra space below the plot area
    let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X Axis")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y Axis")
        .tick_count(8)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw plots
    line1.draw(&mut canvas)?;
    line2.draw(&mut canvas)?;
    line3.draw(&mut canvas)?;

    // Add legend - default position is Below(Center)
    // You can also use Below(HorizontalAlignment::Start) or Below(HorizontalAlignment::End)
    let mut legend = Legend::new(); // Uses LegendPosition::Below(HorizontalAlignment::Center) by default

    for line in [&line1, &line2, &line3] {
        if let Some(entry) = line.legend_entry() {
            legend = legend.add_entry(entry);
        }
    }

    legend.draw(&mut canvas)?;

    canvas.save_png("examples/images/legend_below.png")?;

    println!("✓ Plot saved to examples/images/legend_below.png");
    println!("\nLegend placement:");
    println!("  • Default: Below(Center) - centered below graph (publication standard)");
    println!("  • Canvas expanded vertically to accommodate legend");
    println!("  • Legend doesn't obscure data");
    println!("\nAlignment options for Below:");
    println!("  • Below(HorizontalAlignment::Start) - Left-aligned");
    println!("  • Below(HorizontalAlignment::Center) - Centered (default)");
    println!("  • Below(HorizontalAlignment::End) - Right-aligned");
    println!("\nOther positions available:");
    println!("  • LegendPosition::UpperRight - Corner placement");
    println!("  • LegendPosition::Auto - Smart auto-positioning");
    println!("  • LegendPosition::Custom(x, y) - Manual placement");

    Ok(())
}
