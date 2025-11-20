//! Scatter plot example with multiple series

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Scatter plot example");

    // Create data with some scatter
    let data1 = Series::from_function(0.0, 10.0, 30, |x| x + (x * 0.3).sin() * 2.0);

    let data2 = Series::from_function(0.0, 10.0, 30, |x| x * x * 0.1 + (x * 0.5).cos() * 1.5);

    let data3 = Series::from_function(0.0, 10.0, 50, |x| {
        5.0 + 3.0 * (x * 0.5).sin() + (x * 1.2).cos() * 0.8
    });

    // Create scatter plots with different markers and styles
    let plot1 = ScatterPlot::new(data1)
        .marker_shape(MarkerShape::Circle)
        .marker_size(5.0)
        .marker_style(MarkerStyle::Filled)
        .color(Color::from_hex("#1f77b4").unwrap())
        .label("Linear");

    let plot2 = ScatterPlot::new(data2)
        .marker_shape(MarkerShape::Square)
        .marker_size(5.0)
        .marker_style(MarkerStyle::Filled)
        .color(Color::from_hex("#ff7f0e").unwrap())
        .label("Quadratic");

    let plot3 = ScatterPlot::new(data3)
        .marker_shape(MarkerShape::Triangle)
        .marker_size(6.0)
        .marker_style(MarkerStyle::Filled)
        .color(Color::from_hex("#2ca02c").unwrap())
        .label("Sine");

    // Calculate combined bounds from all plots
    let bounds1 = plot1.bounds().unwrap();
    let bounds2 = plot2.bounds().unwrap();
    let bounds3 = plot3.bounds().unwrap();
    let combined_bounds = bounds1.union(&bounds2).union(&bounds3);
    let bounds = combined_bounds.with_padding(0.1);

    // Create canvas and draw all plots
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes with grid
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("X")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Y")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw plots on top
    plot1.draw(&mut canvas)?;
    plot2.draw(&mut canvas)?;
    plot3.draw(&mut canvas)?;

    // Create and draw legend
    let mut legend = Legend::new().position(LegendPosition::UpperLeft);

    if let Some(entry) = plot1.legend_entry() {
        legend = legend.add_entry(entry);
    }
    if let Some(entry) = plot2.legend_entry() {
        legend = legend.add_entry(entry);
    }
    if let Some(entry) = plot3.legend_entry() {
        legend = legend.add_entry(entry);
    }

    legend.draw(&mut canvas)?;

    canvas.save_png("examples/images/scatter.png")?;

    println!("✓ Plot saved to examples/images/scatter.png");
    println!("  Three scatter series with different marker shapes:");
    println!("    • Circles (blue)");
    println!("    • Squares (orange)");
    println!("    • Triangles (green)");

    Ok(())
}
