//! Basic line plot example

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Basic line plot example");

    // Create some data - a simple parabola
    let data = Series::from_function(0.0, 10.0, 50, |x| x * x);

    // Create a line plot
    let plot = LinePlot::new(data)
        .color(Color::from_hex("#1f77b4").unwrap())
        .line_width(2.5)
        .label("y = x²");

    // Calculate bounds with some padding
    let bounds = plot.bounds().unwrap().with_padding(0.1);

    // Create canvas and draw
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y = X²")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw the plot on top of axes
    plot.draw(&mut canvas)?;

    // Add legend if plot has a label
    if let Some(entry) = plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperRight);
        legend.draw(&mut canvas)?;
    }

    // Save to file in examples folder
    canvas.save_png("examples/images/basic_line.png")?;

    println!("✓ Plot saved to examples/images/basic_line.png");
    println!("  Size: 800x600 pixels");
    println!("  Data points: 50");
    println!(
        "  Bounds: x=[{:.1}, {:.1}], y=[{:.1}, {:.1}]",
        bounds.x_min, bounds.x_max, bounds.y_min, bounds.y_max
    );

    Ok(())
}
