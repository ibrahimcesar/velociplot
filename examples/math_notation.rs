//! Mathematical notation example
//!
//! Demonstrates using math notation in axis labels and plot titles.
//! Supports superscripts, subscripts, and Greek letters using LaTeX-like syntax.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Mathematical notation example");

    // Create sample data for kinetic energy: E_k = 1/2 * m * v^2
    let kinetic_energy = Series::from_function(0.0, 10.0, 50, |v| 0.5 * 2.0 * v.powi(2));

    // Create line plot
    let plot = LinePlot::new(kinetic_energy)
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.5)
        .label("Kinetic Energy");

    let bounds = plot.bounds().unwrap().with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Parse math notation for axis labels
    let x_label_math = parse_math("Velocity (m/s)");
    let y_label_math = parse_math("E_k (Joules)"); // Subscript k

    println!("✓ Math notation parsed:");
    println!("  X-axis: {:?}", x_label_math.to_plain_text());
    println!("  Y-axis: {:?}", y_label_math.to_plain_text());

    // Draw axes with math notation (for now, using plain text conversion)
    // Future: enhance Canvas to render MathElement types with proper sizing
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label(x_label_math.to_plain_text())
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label(y_label_math.to_plain_text())
        .tick_count(8)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw plot
    plot.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperLeft);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/math_notation.png")?;

    println!("\n✓ Plot saved to examples/images/math_notation.png");
    println!("\nSupported math notation:");
    println!("  • Superscripts: x^2 or x^{{10}}");
    println!("  • Subscripts: x_i or x_{{max}}");
    println!("  • Greek letters: \\alpha, \\beta, \\gamma, \\Delta, etc.");
    println!("\nExamples:");
    println!(
        "  E_k = ½mv^2      → {}",
        parse_math("E_k = ½mv^2").to_plain_text()
    );
    println!(
        "  \\alpha + \\beta   → {}",
        parse_math("\\alpha + \\beta").to_plain_text()
    );
    println!(
        "  T^{{-1}}          → {}",
        parse_math("T^{{-1}}").to_plain_text()
    );
    println!(
        "  x_{{max}}^2       → {}",
        parse_math("x_{{max}}^2").to_plain_text()
    );

    Ok(())
}
