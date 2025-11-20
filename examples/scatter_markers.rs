//! Comprehensive scatter plot example showing all marker types

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Scatter plot markers showcase");

    // Create different data series for each marker type
    let y_offset = 0.0;
    let circle_data = Series::from_function(0.0, 10.0, 15, |x| 1.0 + y_offset + 0.3 * x.sin());

    let y_offset = 2.0;
    let square_data =
        Series::from_function(0.0, 10.0, 15, |x| 1.0 + y_offset + 0.3 * (x + 1.0).sin());

    let y_offset = 4.0;
    let triangle_data =
        Series::from_function(0.0, 10.0, 15, |x| 1.0 + y_offset + 0.3 * (x + 2.0).sin());

    let y_offset = 6.0;
    let diamond_data =
        Series::from_function(0.0, 10.0, 15, |x| 1.0 + y_offset + 0.3 * (x + 3.0).sin());

    let y_offset = 8.0;
    let plus_data =
        Series::from_function(0.0, 10.0, 15, |x| 1.0 + y_offset + 0.3 * (x + 4.0).sin());

    let y_offset = 10.0;
    let cross_data =
        Series::from_function(0.0, 10.0, 15, |x| 1.0 + y_offset + 0.3 * (x + 5.0).sin());

    // Create scatter plots with different marker shapes
    let circle_plot = ScatterPlot::new(circle_data)
        .marker_shape(MarkerShape::Circle)
        .marker_size(6.0)
        .color(Color::from_hex("#1f77b4").unwrap())
        .label("Circle");

    let square_plot = ScatterPlot::new(square_data)
        .marker_shape(MarkerShape::Square)
        .marker_size(6.0)
        .color(Color::from_hex("#ff7f0e").unwrap())
        .label("Square");

    let triangle_plot = ScatterPlot::new(triangle_data)
        .marker_shape(MarkerShape::Triangle)
        .marker_size(6.0)
        .color(Color::from_hex("#2ca02c").unwrap())
        .label("Triangle");

    let diamond_plot = ScatterPlot::new(diamond_data)
        .marker_shape(MarkerShape::Diamond)
        .marker_size(6.0)
        .color(Color::from_hex("#d62728").unwrap())
        .label("Diamond");

    let plus_plot = ScatterPlot::new(plus_data)
        .marker_shape(MarkerShape::Plus)
        .marker_size(6.0)
        .color(Color::from_hex("#9467bd").unwrap())
        .label("Plus");

    let cross_plot = ScatterPlot::new(cross_data)
        .marker_shape(MarkerShape::Cross)
        .marker_size(6.0)
        .color(Color::from_hex("#8c564b").unwrap())
        .label("Cross");

    // Calculate combined bounds
    let bounds1 = circle_plot.bounds().unwrap();
    let bounds2 = square_plot.bounds().unwrap();
    let bounds3 = triangle_plot.bounds().unwrap();
    let bounds4 = diamond_plot.bounds().unwrap();
    let bounds5 = plus_plot.bounds().unwrap();
    let bounds6 = cross_plot.bounds().unwrap();

    let combined_bounds = bounds1
        .union(&bounds2)
        .union(&bounds3)
        .union(&bounds4)
        .union(&bounds5)
        .union(&bounds6);
    let bounds = combined_bounds.with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
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

    // Draw all scatter plots
    circle_plot.draw(&mut canvas)?;
    square_plot.draw(&mut canvas)?;
    triangle_plot.draw(&mut canvas)?;
    diamond_plot.draw(&mut canvas)?;
    plus_plot.draw(&mut canvas)?;
    cross_plot.draw(&mut canvas)?;

    // Create legend
    let mut legend = Legend::new().position(LegendPosition::UpperRight);

    for plot in [
        &circle_plot,
        &square_plot,
        &triangle_plot,
        &diamond_plot,
        &plus_plot,
        &cross_plot,
    ] {
        if let Some(entry) = plot.legend_entry() {
            legend = legend.add_entry(entry);
        }
    }

    legend.draw(&mut canvas)?;

    // Save
    canvas.save_png("examples/images/scatter_markers.png")?;

    println!("✓ Plot saved to examples/images/scatter_markers.png");
    println!("  Showcasing all 6 marker types:");
    println!("    • Circle (filled)");
    println!("    • Square (filled)");
    println!("    • Triangle (filled)");
    println!("    • Diamond (filled)");
    println!("    • Plus (always outline)");
    println!("    • Cross (always outline)");

    Ok(())
}
