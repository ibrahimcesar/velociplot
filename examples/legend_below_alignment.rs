//! Legend below alignment demonstration
//!
//! Shows the three alignment options for legends positioned below the graph:
//! Start (left-aligned), Center (default), and End (right-aligned)

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Legend below alignment options");

    // Create sample data
    let data1 = Series::from_function(0.0, 10.0, 50, |x| x.powi(2) * 0.1);
    let data2 = Series::from_function(0.0, 10.0, 50, |x| x * 1.5);

    let line1 = LinePlot::new(data1.clone())
        .color(Color::from_hex("#3498db").unwrap())
        .line_width(2.5)
        .label("Quadratic");

    let line2 = LinePlot::new(data2.clone())
        .color(Color::from_hex("#e74c3c").unwrap())
        .line_width(2.5)
        .label("Linear");

    let bounds = line1
        .bounds()
        .unwrap()
        .union(&line2.bounds().unwrap())
        .with_padding(0.1);

    // Example 1: Start (left-aligned)
    {
        let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

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

        line1.draw(&mut canvas)?;
        line2.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::Below(HorizontalAlignment::Start));

        for line in [&line1, &line2] {
            if let Some(entry) = line.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }

        legend.draw(&mut canvas)?;
        canvas.save_png("examples/images/legend_below_start.png")?;
        println!("✓ Start alignment (left-aligned) saved");
    }

    // Example 2: Center (default)
    {
        let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

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

        line1.draw(&mut canvas)?;
        line2.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::Below(HorizontalAlignment::Center));

        for line in [&line1, &line2] {
            if let Some(entry) = line.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }

        legend.draw(&mut canvas)?;
        canvas.save_png("examples/images/legend_below_center.png")?;
        println!("✓ Center alignment (centered) saved");
    }

    // Example 3: End (right-aligned)
    {
        let mut canvas = SkiaCanvas::new(800, 720, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

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

        line1.draw(&mut canvas)?;
        line2.draw(&mut canvas)?;

        let mut legend = Legend::new().position(LegendPosition::Below(HorizontalAlignment::End));

        for line in [&line1, &line2] {
            if let Some(entry) = line.legend_entry() {
                legend = legend.add_entry(entry);
            }
        }

        legend.draw(&mut canvas)?;
        canvas.save_png("examples/images/legend_below_end.png")?;
        println!("✓ End alignment (right-aligned) saved");
    }

    println!("\n✓ All three alignment options demonstrated!");
    println!("  • Start: Left-aligned legend");
    println!("  • Center: Centered legend (default)");
    println!("  • End: Right-aligned legend");

    Ok(())
}
