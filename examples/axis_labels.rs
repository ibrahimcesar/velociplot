//! Example demonstrating axis label positioning and alignment

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Axis label alignment example");

    // Create simple data
    let data = Series::from_function(0.0, 10.0, 50, |x| x * x * 0.1);

    let plot = LinePlot::new(data)
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.5)
        .label("Quadratic");

    let bounds = plot.bounds().unwrap().with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // X-axis with Center alignment (default)
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X Axis (Centered)")
        .label_alignment(LabelAlignment::Center)
        .tick_count(6)
        .show_grid(true);

    // Y-axis with Center alignment and vertical text
    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y Axis Value")
        .label_alignment(LabelAlignment::Center)
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;
    plot.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperRight);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/axis_labels.png")?;

    println!("✓ Plot saved to examples/images/axis_labels.png");
    println!("  X-axis label: horizontal, centered at bottom");
    println!("  Y-axis label: vertical (rotated 90°), centered on left");
    println!("  Labels are now inside the canvas bounds");

    Ok(())
}
