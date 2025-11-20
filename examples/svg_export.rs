//! SVG export example - vector graphics for publications
//!
//! Demonstrates exporting plots as SVG for use in publications,
//! presentations, and web graphics. SVG is resolution-independent
//! and can be scaled without quality loss.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - SVG export example");

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

    // Create SVG canvas
    let mut canvas = SvgCanvas::new(800, 600, bounds);
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

    // Add legend with auto-positioning
    let mut legend = Legend::new().position(LegendPosition::Auto);

    for line in [&line1, &line2, &line3] {
        if let Some(entry) = line.legend_entry() {
            legend = legend.add_entry(entry);
        }
    }

    legend.draw(&mut canvas)?;

    // Save as SVG
    canvas.save_svg("examples/plot.svg")?;

    println!("✓ SVG saved to examples/plot.svg");
    println!("  Vector graphics - scales infinitely without quality loss");
    println!("  Perfect for publications, presentations, and web graphics");
    println!("  Can be opened in web browsers, Inkscape, Adobe Illustrator, etc.");

    Ok(())
}
