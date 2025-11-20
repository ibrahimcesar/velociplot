//! Bar plot example showing categorical data visualization

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Bar plot example");

    // Example data: Monthly sales
    let months = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let sales = vec![12.5, 15.3, 18.1, 14.7, 21.2, 19.8];

    let data = Series::new(months, sales)?;

    // Create vertical bar plot
    let bar_plot = BarPlot::new(data)
        .orientation(BarOrientation::Vertical)
        .bar_width(0.7)
        .color(Color::from_hex("#3498db").unwrap())
        .label("Monthly Sales");

    let bounds = bar_plot.bounds().unwrap().with_padding(0.1);

    // Create canvas
    let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
    canvas.fill_background(&Color::WHITE.to_rgba())?;

    // Draw axes
    let x_axis = Axis::new(AxisPosition::Bottom)
        .label("Month")
        .tick_count(6)
        .show_grid(true);

    let y_axis = Axis::new(AxisPosition::Left)
        .label("Sales (thousands)")
        .tick_count(6)
        .show_grid(true);

    x_axis.draw(&mut canvas)?;
    y_axis.draw(&mut canvas)?;

    // Draw bar plot
    bar_plot.draw(&mut canvas)?;

    // Add legend
    if let Some(entry) = bar_plot.legend_entry() {
        let legend = Legend::new()
            .add_entry(entry)
            .position(LegendPosition::UpperRight);
        legend.draw(&mut canvas)?;
    }

    canvas.save_png("examples/images/bar_plot.png")?;

    println!("✓ Plot saved to examples/images/bar_plot.png");
    println!("  Vertical bar chart showing monthly sales data");

    Ok(())
}
