//! Example demonstrating custom font loading

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Custom font example");

    // Create data
    let data = Series::from_function(0.0, 10.0, 50, |x| x.powi(2));

    // Create a line plot
    let plot = LinePlot::new(data)
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.5)
        .label("Custom Font");

    let bounds = plot.bounds().unwrap().with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;

    // Load custom font (using the same JetBrains Mono, but users can replace this)
    // Users can call: canvas.load_font_from_file("path/to/their/font.ttf")?;
    println!("  Using default embedded font (JetBrains Mono)");
    println!("  To use a custom font, call: canvas.load_font_from_file(\"your-font.ttf\")?;");

    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X Axis")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y = X²")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    plot.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperLeft);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/custom_font.png")?;

    println!("✓ Plot saved to examples/images/custom_font.png");
    println!("  Font: JetBrains Mono (embedded)");

    Ok(())
}
