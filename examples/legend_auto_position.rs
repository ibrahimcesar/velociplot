//! Example demonstrating automatic legend positioning
//!
//! The legend with `LegendPosition::Auto` will automatically position itself
//! in the corner with the least plot data density.

use velociplot::prelude::*;

fn main() -> Result<()> {
    println!("🦖 velociplot - Auto legend positioning example");

    // Example 1: Data in upper left - legend should go to upper right
    {
        let data1 = Series::from_function(0.0, 5.0, 50, |x| 8.0 + x * 0.4);
        let data2 = Series::from_function(0.0, 5.0, 50, |x| 7.0 + x * 0.3);

        let line1 = LinePlot::new(data1)
            .color(Color::from_hex("#3498db").unwrap())
            .label("Series 1");

        let line2 = LinePlot::new(data2)
            .color(Color::from_hex("#e74c3c").unwrap())
            .label("Series 2");

        let bounds1 = line1.bounds().unwrap();
        let bounds2 = line2.bounds().unwrap();
        let bounds = bounds1.union(&bounds2).with_padding(0.1);

        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("X Axis")
            .tick_count(6)
            .show_grid(true);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Y Axis")
            .tick_count(6)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        line1.draw(&mut canvas)?;
        line2.draw(&mut canvas)?;

        // Auto-position legend - should go to upper right (data is on left)
        let mut legend = Legend::new().position(LegendPosition::Auto);

        if let Some(entry) = line1.legend_entry() {
            legend = legend.add_entry(entry);
        }
        if let Some(entry) = line2.legend_entry() {
            legend = legend.add_entry(entry);
        }

        legend.draw(&mut canvas)?;
        canvas.save_png("examples/images/legend_auto_upper_left_data.png")?;
        println!("✓ Example 1: Data in upper left → Legend in upper right");
    }

    // Example 2: Data in upper right - legend should go to upper left
    {
        let data1 = Series::from_function(5.0, 10.0, 50, |x| 8.0 + (x - 5.0) * 0.4);
        let data2 = Series::from_function(5.0, 10.0, 50, |x| 7.0 + (x - 5.0) * 0.3);

        let line1 = LinePlot::new(data1)
            .color(Color::from_hex("#3498db").unwrap())
            .label("Series A");

        let line2 = LinePlot::new(data2)
            .color(Color::from_hex("#e74c3c").unwrap())
            .label("Series B");

        let bounds1 = line1.bounds().unwrap();
        let bounds2 = line2.bounds().unwrap();
        let bounds = bounds1.union(&bounds2).with_padding(0.1);

        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("X Axis")
            .tick_count(6)
            .show_grid(true);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Y Axis")
            .tick_count(6)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        line1.draw(&mut canvas)?;
        line2.draw(&mut canvas)?;

        // Auto-position legend - should go to upper left (data is on right)
        let mut legend = Legend::new().position(LegendPosition::Auto);

        if let Some(entry) = line1.legend_entry() {
            legend = legend.add_entry(entry);
        }
        if let Some(entry) = line2.legend_entry() {
            legend = legend.add_entry(entry);
        }

        legend.draw(&mut canvas)?;
        canvas.save_png("examples/images/legend_auto_upper_right_data.png")?;
        println!("✓ Example 2: Data in upper right → Legend in upper left");
    }

    // Example 3: Scatter plot with auto-positioning
    {
        // Create scattered data in lower left
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();
        let mut seed = 42u64;
        for _ in 0..100 {
            seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
            let x = (seed as f64 / (1u64 << 31) as f64) * 3.0; // 0 to 3
            seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
            let y = (seed as f64 / (1u64 << 31) as f64) * 3.0; // 0 to 3
            x_data.push(x);
            y_data.push(y);
        }

        let scatter = ScatterPlot::new(Series::new(x_data, y_data)?)
            .color(Color::from_hex("#9b59b6").unwrap())
            .marker_shape(MarkerShape::Circle)
            .label("Data Points");

        let bounds = scatter.bounds().unwrap().with_padding(0.1);

        let mut canvas = SkiaCanvas::new(800, 600, bounds)?;
        canvas.fill_background(&Color::WHITE.to_rgba())?;

        let x_axis = Axis::new(AxisPosition::Bottom)
            .label("X Values")
            .tick_count(6)
            .show_grid(true);

        let y_axis = Axis::new(AxisPosition::Left)
            .label("Y Values")
            .tick_count(6)
            .show_grid(true);

        x_axis.draw(&mut canvas)?;
        y_axis.draw(&mut canvas)?;

        scatter.draw(&mut canvas)?;

        // Auto-position legend - should go to upper right (data is in lower left)
        let mut legend = Legend::new().position(LegendPosition::Auto);

        if let Some(entry) = scatter.legend_entry() {
            legend = legend.add_entry(entry);
        }

        legend.draw(&mut canvas)?;
        canvas.save_png("examples/images/legend_auto_scatter.png")?;
        println!("✓ Example 3: Scatter in lower left → Legend in upper right");
    }

    println!("\n✓ All examples saved!");
    println!("  Auto-positioning finds the corner with least plot data");
    println!("  This prevents legends from obscuring important data");

    Ok(())
}
